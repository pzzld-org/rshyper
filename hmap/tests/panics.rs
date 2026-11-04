/*
    Appellation: panics <test>
    Contrib: @FL03
*/
//! #125: the public panics outside id allocation, and a random walk over the checked API that
//! must never panic and must leave the graph untouched whenever it returns an error.
use rshyper_core::idx::{EdgeId, IndexTracker, VertexId};
use rshyper_core::{Error, GraphProps, Mode, Result, Weight};
use rshyper_hmap::{DiHyperMap, HyperMap, UnHyperMap};

/// P-4: a member without a node entry is an error, never a panic or a silent skip
#[test]
fn load_edge_nodes_rejects_a_dangling_member() -> Result<()> {
    let mut g = UnHyperMap::<usize, usize>::undirected();
    let [a, b] = [g.add_vertex()?, g.add_vertex()?];
    let e = g.add_link([a, b])?;
    assert_eq!(g.load_edge_nodes(&e)?.len(), 2);
    g.get_domain_mut(&e)?.insert(VertexId::new(1234));
    let before = (g.order(), g.size(), g.history().clone(), *g.position());
    assert!(matches!(g.load_edge_nodes(&e), Err(Error::NodeNotFound)));
    #[allow(deprecated)]
    let old = g.get_edge_nodes(&e);
    assert!(matches!(old, Err(Error::NodeNotFound)));
    assert_eq!(
        before,
        (g.order(), g.size(), g.history().clone(), *g.position())
    );
    // the same after `retain_nodes` drops a member but keeps its edge
    let mut g = UnHyperMap::<usize, usize>::undirected();
    let [a, b] = [g.add_vertex()?, g.add_vertex()?];
    let e = g.add_link([a, b])?;
    unsafe { g.retain_nodes(|v, _| *v != b) };
    assert!(matches!(g.load_edge_nodes(&e), Err(Error::NodeNotFound)));
    assert!(matches!(
        g.load_edge_nodes(&EdgeId::new(99)),
        Err(Error::EdgeNotFound)
    ));
    Ok(())
}

/// P-6 / P-7: a `Mode`-kinded graph has no mode, and asking is not a panic
#[test]
fn mode_is_none_for_a_mode_kinded_graph() {
    let dynamic = HyperMap::<usize, usize, rshyper_core::Attrs<usize, Mode>>::dynamic();
    assert_eq!(dynamic.mode(), None);
    assert_eq!(
        UnHyperMap::<usize, usize>::undirected().mode(),
        Some(Mode::Undirected)
    );
    assert_eq!(
        DiHyperMap::<usize, usize>::directed().mode(),
        Some(Mode::Directed)
    );
    assert_eq!(Mode::from_type::<Mode>(), None);
    assert_eq!(rshyper_core::Attrs::<usize, Mode>::new().mode(), None);
    assert_eq!(
        GraphProps::mode(&rshyper_core::Attrs::<usize, Mode>::new()),
        None
    );
}

/// a #105 regression witness (acceptance 3), not a fix: an id inserted through the `node()`
/// entry API sits where the cursor allocates next, so `add_node` hands out the same id and
/// replaces the stored weight without an error. Restricting setters or guarding allocation is
/// the #105 decision; flip this test when it lands.
#[test]
fn entry_api_insert_then_add_node_overwrites_105_witness() -> Result<()> {
    let mut g = UnHyperMap::<usize, usize>::undirected();
    let id = VertexId::new(0);
    g.node(id).or_insert(rshyper_core::Node::new(id, 42));
    assert_eq!(g.add_node(Weight(7))?, id);
    assert_eq!(g.get_node_weight(&id)?, &7);
    assert_eq!(g.order(), 1);
    Ok(())
}

/// splitmix64, so the test needs no new dependency
struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

const SEEDS: u64 = 8;
const STEPS: usize = 100_000;

/// the invariants.rs predicates for a graph that only the checked API has touched
macro_rules! assert_invariants {
    ($g:expr, $ix:ty) => {{
        let g = &$g;
        let h: &IndexTracker<$ix> = g.history();
        // history lists every live id once, in insertion (= ascending) order
        let mut nodes = g.vertices().copied().collect::<Vec<_>>();
        nodes.sort();
        assert_eq!(h.nodes(), &nodes);
        let mut edges = g.iter_edge_keys().copied().collect::<Vec<_>>();
        edges.sort();
        assert_eq!(h.edges(), &edges);
        assert_eq!((g.order(), g.size()), (nodes.len(), edges.len()));
        // ids are never reissued: every live id is below the cursor
        let cursor = *g.position();
        assert!(nodes.iter().all(|v| v < cursor.node()));
        assert!(edges.iter().all(|e| e < cursor.edge()));
        // every edge is non-empty and names live vertices only
        for (_, edge) in g.iter_edges() {
            assert!(!edge.domain().is_empty());
            assert!(edge.domain().iter().all(|v| g.contains_node(v)));
        }
    }};
}

/// the members of `e`, sorted
fn members<Ix: rshyper_core::idx::HashIndex + Copy + Ord>(
    g: &UnHyperMap<usize, usize, Ix>,
    e: &EdgeId<Ix>,
) -> Result<Vec<VertexId<Ix>>> {
    let mut d = g.get_domain(e)?.iter().copied().collect::<Vec<_>>();
    d.sort();
    Ok(d)
}

/// one random walk over the checked API; `ix` is the index type
macro_rules! random_walk {
    ($name:ident, $ix:ty) => {
        #[test]
        fn $name() -> Result<()> {
            type G = UnHyperMap<usize, usize, $ix>;
            // ids past the type's range saturate at MAX, itself a dead id
            let id = |n: usize| <$ix>::try_from(n).unwrap_or(<$ix>::MAX);
            let (mut ok, mut err) = (0usize, 0usize);
            for seed in 0..SEEDS {
                let mut rng = SplitMix(seed);
                let mut g = G::undirected();
                let mut step = 0;
                while step < STEPS {
                    // sorted, so a seed replays the same walk under the randomly seeded hasher
                    let mut verts = g.vertices().copied().collect::<Vec<_>>();
                    verts.sort();
                    let mut edges = g.iter_edge_keys().copied().collect::<Vec<_>>();
                    edges.sort();
                    // a live id most of the time, an arbitrary one otherwise
                    let v = |rng: &mut SplitMix| match verts.len() {
                        n if n > 0 && rng.below(8) > 0 => verts[rng.below(n)],
                        _ => VertexId::new(id(rng.below(300))),
                    };
                    let e = |rng: &mut SplitMix| match edges.len() {
                        n if n > 0 && rng.below(8) > 0 => edges[rng.below(n)],
                        _ => EdgeId::new(id(rng.below(300))),
                    };
                    let at = format!("seed {seed} step {step}");
                    let before = (g.order(), g.size(), g.history().clone(), *g.position());
                    // `None`: the draw named no operation (a capped add, or no `clear`); redraw
                    let result: Option<Result<()>> = match rng.below(20) {
                        0..=3 if verts.len() < 64 => Some(g.add_node(Weight(step)).map(drop)),
                        4..=7 if edges.len() < 128 => {
                            let k = rng.below(5);
                            let mut domain = (0..k).map(|_| v(&mut rng)).collect::<Vec<_>>();
                            let res = g.add_edge(domain.clone(), Weight(step));
                            if let Ok(id) = &res {
                                // rejection, not truncation: every member was live and is kept
                                assert!(domain.iter().all(|u| verts.contains(u)), "{at}");
                                domain.sort();
                                domain.dedup();
                                assert_eq!(members(&g, id)?, domain, "{at}");
                            }
                            Some(res.map(drop))
                        }
                        8..=9 => {
                            let x = e(&mut rng);
                            let res = g.remove_edge(&x).map(drop);
                            assert!(res.is_err() || !g.contains_edge(&x), "{at}");
                            Some(res)
                        }
                        10 => {
                            let x = v(&mut rng);
                            let res = g.remove_node(&x).map(drop);
                            assert!(res.is_err() || !g.contains_node(&x), "{at}");
                            Some(res)
                        }
                        11 => {
                            let (a, b) = (e(&mut rng), e(&mut rng));
                            let mut union = [a, b]
                                .iter()
                                .filter_map(|x| members(&g, x).ok())
                                .flatten()
                                .collect::<Vec<_>>();
                            union.sort();
                            union.dedup();
                            let res = g.merge_edges(&a, &b);
                            if let Ok(id) = &res {
                                // both operands are gone, and the merged edge holds both domains
                                assert!(a != b, "{at}");
                                assert!(!g.contains_edge(&a) && !g.contains_edge(&b), "{at}");
                                assert_eq!(members(&g, id)?, union, "{at}");
                            }
                            Some(res.map(drop))
                        }
                        12 => Some(g.load_edge_nodes(&e(&mut rng)).map(drop)),
                        13 => Some(g.find_node_neighbors(&v(&mut rng)).map(drop)),
                        14 => Some(g.set_edge_weight(&e(&mut rng), Weight(step)).map(drop)),
                        15 => {
                            let salt = rng.below(5);
                            g.retain_edges(|id, _| (usize::from(*id.get()) + salt) % 5 != 0);
                            Some(Ok(()))
                        }
                        16 if rng.below(50) == 0 => {
                            g.clear();
                            Some(Ok(()))
                        }
                        17 => Some(g.bft().search(v(&mut rng)).map(drop).map_err(Into::into)),
                        18 => Some(g.dft().search(v(&mut rng)).map(drop).map_err(Into::into)),
                        19 => {
                            let (s, t) = (v(&mut rng), v(&mut rng));
                            Some(g.dijkstra().find_path(s, t).map(drop).map_err(Into::into))
                        }
                        _ => None,
                    };
                    match result {
                        None => continue,
                        Some(Ok(())) => ok += 1,
                        Some(Err(_)) => {
                            err += 1;
                            let after = (g.order(), g.size(), g.history().clone(), *g.position());
                            assert_eq!(before, after, "{at}");
                        }
                    }
                    assert_invariants!(g, $ix);
                    step += 1;
                }
            }
            // both outcomes were exercised, by real calls only
            assert_eq!(ok + err, SEEDS as usize * STEPS);
            assert!(ok > 0 && err > 0, "ok {ok} err {err}");
            Ok(())
        }
    };
}

random_walk!(random_checked_walk_never_panics_usize, usize);
random_walk!(random_checked_walk_never_panics_u8, u8);
