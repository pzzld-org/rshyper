/*
    Appellation: incidence <tests>
    Contrib: @FL03
*/
//! #123: the vertex → incident-edge index stays exact through checked mutations and escape
//! hatches, lists each incident edge once in ascending id order, and reports unknown vertices.
mod common;

use common::{RevBuild, SplitMix};
use core::hash::BuildHasher;
use core::panic::{RefUnwindSafe, UnwindSafe};
use rshyper_core::edge::HyperEdge;
use rshyper_core::idx::IndexFrame;
use rshyper_core::idx::{EdgeId, VertexId};
use rshyper_core::{Error, GraphProps, Node, Result, VertexSet, Weight};
use rshyper_hmap::{DiHyperMap, HyperMap, UnHyperMap};
use std::collections::{BTreeMap, BTreeSet};

type Ix = usize;
type Map<A, S> = HyperMap<usize, usize, A, S>;

const SEEDS: u64 = 8;
const STEPS: usize = 10_000;
const MAX_NODES: usize = 64;
const MAX_EDGES: usize = 200;

fn sorted_vertices<A, S>(g: &Map<A, S>) -> Vec<VertexId<Ix>>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
{
    let mut v = g.vertices().copied().collect::<Vec<_>>();
    v.sort_unstable();
    v
}

fn sorted_edges<A, S>(g: &Map<A, S>) -> Vec<EdgeId<Ix>>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
{
    let mut e = g.iter_edge_keys().copied().collect::<Vec<_>>();
    e.sort_unstable();
    e
}

/// the incidence lists by brute force: one pass over every edge, each list sorted
fn brute<A, S>(g: &Map<A, S>) -> BTreeMap<VertexId<Ix>, Vec<EdgeId<Ix>>>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
{
    let mut out = BTreeMap::<_, Vec<_>>::new();
    for (id, edge) in g.edges() {
        for v in edge.domain() {
            out.entry(*v).or_default().push(*id);
        }
    }
    out.values_mut().for_each(|l| l.sort_unstable());
    out
}

/// what one [`check`] exercised
#[derive(Default)]
struct Checked {
    /// checks answered by the maintained index rather than the stale fallback
    fresh: usize,
    /// vertices still named by edges after a node-map escape hatch removed them
    dead: usize,
}

/// asserts every incidence read against the brute-force scan
fn check<A, S>(g: &Map<A, S>, seen: &mut Checked)
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher + Default,
{
    let expect = brute(g);
    for v in g.vertices() {
        let want = expect.get(v).cloned().unwrap_or_default();
        let got = g.incident_edges(v).unwrap().copied().collect::<Vec<_>>();
        assert_eq!(got, want, "incident_edges({v})");
        assert_eq!(g.get_node_degree(v), want.len(), "get_node_degree({v})");
        let neighbors = g.find_node_neighbors(v).unwrap();
        let want_neighbors = want
            .iter()
            .flat_map(|e| g.get_domain(e).unwrap().iter().copied())
            .filter(|u| u != v)
            .collect::<BTreeSet<_>>();
        assert_eq!(
            neighbors.into_iter().collect::<BTreeSet<_>>(),
            want_neighbors
        );
    }
    // vertices that edges still name after an escape hatch removed them from the node map
    for (v, want) in &expect {
        if !g.contains_node(v) {
            seen.dead += 1;
            assert!(matches!(g.incident_edges(v), Err(Error::NodeNotFound)));
            let got = g.find_edges_with_node(v).copied().collect::<Vec<_>>();
            assert_eq!(&got, want, "find_edges_with_node({v}) on a dead vertex");
        }
    }
    assert!(matches!(
        g.incident_edges(&VertexId::new(usize::MAX)),
        Err(Error::NodeNotFound)
    ));
    if !g.is_incidence_stale() {
        seen.fresh += 1;
    }
}

/// one fixed-seed run of random checked mutations and escape-hatch edits, checked after
/// every step
fn run_model<A, S>(seed: u64, mut g: Map<A, S>) -> Result<Checked>
where
    A: GraphProps<Ix = Ix>,
    A::Kind: Clone,
    S: BuildHasher + Clone + Default + Send + Sync,
{
    let mut rng = SplitMix(seed);
    let mut seen = Checked::default();
    for step in 0..STEPS {
        let verts = sorted_vertices(&g);
        let edges = sorted_edges(&g);
        let pick_v = |rng: &mut SplitMix| verts[rng.below(verts.len())];
        let pick_e = |rng: &mut SplitMix| edges[rng.below(edges.len())];
        let salt = rng.below(11);
        match rng.below(100) {
            0..=13 if verts.len() < MAX_NODES => {
                g.add_vertex()?;
            }
            0..=43 | 90..=99 if !verts.is_empty() && edges.len() < MAX_EDGES => {
                let k = 1 + rng.below(8);
                let domain = (0..k).map(|_| pick_v(&mut rng)).collect::<Vec<_>>();
                g.add_edge(domain, Weight(step))?;
            }
            0..=53 | 90..=99 if !edges.is_empty() => {
                g.remove_edge(&pick_e(&mut rng))?;
            }
            54..=59 if !verts.is_empty() => {
                g.remove_node(&pick_v(&mut rng))?;
            }
            60..=63 if edges.len() >= 2 => {
                let (a, b) = (pick_e(&mut rng), pick_e(&mut rng));
                if a != b {
                    g.merge_edges(&a, &b)?;
                }
            }
            64..=65 => {
                g.retain_edges(|id, _| (*id.get() + salt) % 5 != 0);
            }
            66 if rng.below(10) == 0 => {
                g.clear();
            }
            67..=68 => g = g.clone(),
            69..=72 if !edges.is_empty() && !verts.is_empty() => {
                let (e, v) = (pick_e(&mut rng), pick_v(&mut rng));
                g.get_domain_mut(&e)?.insert(v);
            }
            73..=75 if !edges.is_empty() => {
                let domain = g.get_domain_mut(&pick_e(&mut rng))?;
                if domain.len() > 1 {
                    let first = *domain.iter().min().unwrap();
                    domain.remove(&first);
                }
            }
            76..=77 if !edges.is_empty() => {
                g.edges_mut().remove(&pick_e(&mut rng));
            }
            78..=79 if !verts.is_empty() => {
                // an edge the history never saw, through the entry API
                let id = EdgeId::new(1_000_000 + step);
                let domain = (0..1 + rng.below(4))
                    .map(|_| pick_v(&mut rng))
                    .collect::<VertexSet<Ix, S>>();
                g.edge(id)
                    .or_insert(HyperEdge::from_parts(id, domain, Weight(0)));
            }
            80..=81 if !verts.is_empty() => {
                let v = pick_v(&mut rng);
                g.iter_edges_mut()
                    .filter(|(id, _)| *id.get() % 7 == salt % 7)
                    .for_each(|(_, edge)| {
                        edge.domain_mut().insert(v);
                    });
            }
            82 => {
                g.reindex();
            }
            83..=84 if !verts.is_empty() => {
                let v = pick_v(&mut rng);
                #[cfg(feature = "rayon")]
                {
                    use rayon::iter::ParallelIterator;
                    g.par_iter_facets_mut()
                        .filter(|edge| *edge.id().get() % 11 == salt)
                        .for_each(|edge| {
                            edge.domain_mut().insert(v);
                        });
                }
                #[cfg(not(feature = "rayon"))]
                g.facets_mut()
                    .filter(|edge| *edge.id().get() % 11 == salt)
                    .for_each(|edge| {
                        edge.domain_mut().insert(v);
                    });
            }
            85..=86 if !edges.is_empty() => {
                g.set_edge_weight(&pick_e(&mut rng), Weight(step))?;
            }
            // node-map escape hatches: the index is derived from edge domains only, so these
            // leave it fresh, while liveness comes from the node map at read time
            87 if !verts.is_empty() => {
                g.nodes_mut().remove(&pick_v(&mut rng));
            }
            88 => {
                let dead = brute(&g)
                    .into_keys()
                    .filter(|v| !g.contains_node(v))
                    .collect::<Vec<_>>();
                if !dead.is_empty() {
                    let v = dead[rng.below(dead.len())];
                    g.node(v).or_insert(Node::new(v, 0));
                }
            }
            89 if rng.below(4) == 0 => {
                // SAFETY: only prunes nodes; edges naming them are exactly what is under test
                unsafe {
                    g.retain_nodes(|id, _| (*id.get() + salt) % 9 != 0);
                }
            }
            _ => {}
        }
        check(&g, &mut seen);
    }
    Ok(seen)
}

#[test]
fn model_undirected_matches_brute_force() -> Result<()> {
    for seed in 0..SEEDS {
        let seen = run_model(seed, UnHyperMap::<usize, usize>::undirected())?;
        // most checks must exercise the maintained index, not only the stale fallback
        assert!(seen.fresh > STEPS / 2, "seed {seed}: {} fresh", seen.fresh);
        assert!(seen.dead > 0, "seed {seed}: no dead vertex was checked");
    }
    Ok(())
}

#[test]
fn model_directed_matches_brute_force() -> Result<()> {
    for seed in 0..SEEDS {
        let seen = run_model(seed, DiHyperMap::<usize, usize>::directed())?;
        assert!(seen.fresh > STEPS / 2, "seed {seed}: {} fresh", seen.fresh);
        assert!(seen.dead > 0, "seed {seed}: no dead vertex was checked");
    }
    Ok(())
}

/// F2 (Silo: per-source replace): remove every edge of one source and add new ones; the old
/// ids never appear in any incidence list, before or after the index is rebuilt.
#[test]
fn per_source_replace_never_leaves_old_ids() -> Result<()> {
    for stale in [false, true] {
        let mut rng = SplitMix(0xF2);
        let mut g = UnHyperMap::<usize, usize>::undirected();
        let sources = (0..10)
            .map(|_| g.add_vertex())
            .collect::<Result<Vec<_>>>()?;
        let targets = (0..30)
            .map(|_| g.add_vertex())
            .collect::<Result<Vec<_>>>()?;
        let tag = g.add_vertex()?;
        for s in &sources {
            for _ in 0..5 {
                g.add_link([*s, targets[rng.below(30)]])?;
            }
            g.add_link([*s, tag])?;
        }
        let mut removed = BTreeSet::new();
        for round in 0..20 {
            if stale {
                g.edges_mut();
            }
            let s = sources[round % sources.len()];
            let old = g.incident_edges(&s)?.copied().collect::<Vec<_>>();
            for e in &old {
                g.remove_edge(e)?;
            }
            removed.extend(old);
            let mut new = (0..5)
                .map(|_| g.add_link([s, targets[rng.below(30)]]))
                .collect::<Result<Vec<_>>>()?;
            new.push(g.add_link([s, tag])?);
            assert_eq!(g.incident_edges(&s)?.copied().collect::<Vec<_>>(), new);
            for v in g.vertices() {
                assert!(g.incident_edges(v)?.all(|e| !removed.contains(e)));
            }
            check(&g, &mut Checked::default());
        }
    }
    Ok(())
}

/// F3 (Silo: parallel edges): six edges with the identical domain {Same, Index} coexist and
/// each appears exactly once, in ascending order.
#[test]
fn parallel_edges_each_appear_once() -> Result<()> {
    let mut g = UnHyperMap::<usize, usize>::undirected();
    let same = g.add_vertex()?;
    let index = g.add_vertex()?;
    let ids = (0..6)
        .map(|_| g.add_link([same, index]))
        .collect::<Result<Vec<_>>>()?;
    assert_eq!(g.incident_edges(&index)?.copied().collect::<Vec<_>>(), ids);
    assert_eq!(g.incident_edges(&same)?.copied().collect::<Vec<_>>(), ids);
    assert_eq!(g.get_node_degree(&index), 6);
    assert_eq!(
        g.find_node_neighbors(&index)?
            .into_iter()
            .collect::<Vec<_>>(),
        vec![same]
    );
    Ok(())
}

/// F4 (Silo: stable paging): with ≥ 300 edges, the incidence lists list each live edge once
/// and agree between two graphs built under different hashers.
#[test]
fn incidence_order_is_independent_of_the_hasher() -> Result<()> {
    fn build<S: BuildHasher + Default>() -> Result<UnHyperMap<usize, usize, usize, S>> {
        let mut rng = SplitMix(0xF4);
        let mut g = UnHyperMap::<usize, usize, usize, S>::undirected();
        let v = (0..60)
            .map(|_| g.add_vertex())
            .collect::<Result<Vec<_>>>()?;
        for _ in 0..320 {
            let k = 1 + rng.below(5);
            g.add_link((0..k).map(|_| v[rng.below(60)]).collect::<Vec<_>>())?;
        }
        for _ in 0..20 {
            let mut e = g.iter_edge_keys().copied().collect::<Vec<_>>();
            e.sort_unstable();
            g.remove_edge(&e[rng.below(e.len())])?;
        }
        Ok(g)
    }
    let a = build::<hashbrown::DefaultHashBuilder>()?;
    let b = build::<RevBuild>()?;
    // the stale scan and the rebuild walk the edge map in hash order before sorting
    let mut b_stale = b.clone();
    b_stale.edges_mut();
    let mut b_rebuilt = b_stale.clone();
    b_rebuilt.reindex();
    assert!(a.size() >= 300);
    let mut listed = 0;
    for v in sorted_vertices(&a) {
        let la = a.incident_edges(&v)?.copied().collect::<Vec<_>>();
        for other in [&b, &b_stale, &b_rebuilt] {
            let lb = other.incident_edges(&v)?.copied().collect::<Vec<_>>();
            assert_eq!(la, lb, "vertex {v}");
        }
        assert!(
            la.windows(2).all(|w| w[0] < w[1]),
            "vertex {v}: not ascending"
        );
        assert!(la.iter().all(|e| a.is_node_in_domain(e, &v)));
        listed += la.len();
    }
    assert_eq!(listed, a.facets().map(|e| e.len()).sum::<usize>());
    Ok(())
}

/// a checked insertion that reuses the id of a live edge (possible after raw history and
/// cursor edits) replaces it, and the index forgets the old edge's members
#[test]
fn replacing_a_live_edge_id_forgets_its_old_incidences() -> Result<()> {
    let mut g = UnHyperMap::<usize, usize>::undirected();
    let v = (0..3).map(|_| g.add_vertex()).collect::<Result<Vec<_>>>()?;
    let e0 = g.add_link([v[0], v[1]])?;
    g.add_link([v[1], v[2]])?;
    let cursor = *g.position();
    g.history_mut().edges_mut().retain(|id| *id != e0);
    g.set_position(IndexFrame::new(e0, *cursor.node()));
    let reused = g.add_link([v[2]])?;
    assert_eq!(reused, e0);
    g.set_position(cursor);
    check(&g, &mut Checked::default());
    assert_eq!(g.incident_edges(&v[0])?.count(), 0);
    assert_eq!(g.incident_edges(&v[2])?.count(), 2);
    Ok(())
}

#[test]
fn hyper_map_keeps_its_auto_traits() {
    fn assert_auto<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {}
    assert_auto::<HyperMap<(), ()>>();
}
