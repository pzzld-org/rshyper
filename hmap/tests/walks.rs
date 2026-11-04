/*
    Appellation: walks <test>
    Contrib: @FL03
*/
//! #124: bounded shortest-hop walk enumeration
mod common;

use common::{RevBuild, SplitMix};
use core::hash::BuildHasher;
use rshyper_algo::{Error, PathLimits, Walk, Walks};
use rshyper_core::idx::{EdgeId, VertexId};
use rshyper_core::{Result, Weight};
use rshyper_hmap::UnHyperMap;
use std::collections::{BTreeMap, BTreeSet};

type V = VertexId<usize>;
type G<S> = UnHyperMap<&'static str, &'static str, usize, S>;

const ALL: PathLimits = PathLimits {
    max_depth: usize::MAX,
    max_expansions: usize::MAX,
    max_paths: usize::MAX,
};

fn limits(max_depth: usize, max_paths: usize) -> PathLimits {
    PathLimits {
        max_depth,
        max_paths,
        ..ALL
    }
}

/// the walks as `name -kind-> name …` strings
fn show<S: BuildHasher + Default>(g: &G<S>, found: &Walks<usize>) -> Vec<String> {
    found
        .walks
        .iter()
        .map(|w| {
            let mut s = g
                .get_node(&w.vertices[0])
                .unwrap()
                .weight()
                .get()
                .to_string();
            for (e, v) in w.edges.iter().zip(&w.vertices[1..]) {
                let kind = g.get_edge_weight(e).unwrap();
                let name = g.get_node(v).unwrap().weight();
                s += &format!(" -{}#{}-> {}", kind.get(), **e, name.get());
            }
            s
        })
        .collect()
}

/// F1: a reduced port of Silo's `graph_paths_are_shortest_bounded_and_survive_cycles`, edges
/// inserted in Silo-id order so `EdgeId` order matches
fn silo_fixture<S: BuildHasher + Default>() -> Result<(G<S>, [V; 5])> {
    let mut g = G::<S>::undirected();
    let [a, c, e, h, i] = ["A", "C", "E", "H", "I"].map(|n| g.add_node(Weight(n)).unwrap());
    for (kind, members) in [
        ("link", &[c, i][..]),      // 0: 2a0aa5
        ("parent", &[c, i]),        // 1: 2fa1d4
        ("link", &[i, a]),          // 2: 306396
        ("link", &[i, a]),          // 3: 37385e
        ("tag", &[i, h]),           // 4: 663472
        ("link", &[i, e]),          // 5: 6a94a2
        ("link", &[c, i]),          // 6: 966cf9
        ("depends_on", &[a, c, i]), // 7: ced23c
        ("link", &[i, a]),          // 8: da5072
    ] {
        g.add_edge(members.iter().copied(), Weight(kind))?;
    }
    Ok((g, [a, c, e, h, i]))
}

#[test]
fn f1_silo_paths_fixture() -> Result<()> {
    let (g, [a, c, e, h, i]) = silo_fixture::<RevBuild>()?;
    let any = |_: &EdgeId<usize>, _: &&str| true;
    let kinds = |ks: &'static [&'static str]| move |_: &EdgeId<usize>, k: &&str| ks.contains(k);
    let yes = |_: &V, _: &&str| true;
    let mut bft = g.bft();

    let got = bft.shortest_walks(c, e, limits(usize::MAX, 3), any, yes)?;
    assert_eq!(
        show(&g, &got),
        [
            "C -link#0-> I -link#5-> E",
            "C -parent#1-> I -link#5-> E",
            "C -link#6-> I -link#5-> E",
        ]
    );
    assert!(got.truncated && got.complete);

    let got = bft.shortest_walks(c, e, limits(usize::MAX, 10), any, yes)?;
    assert_eq!(got.walks.len(), 4);
    assert_eq!(show(&g, &got)[3], "C -depends_on#7-> I -link#5-> E");
    assert!(!got.truncated && got.complete);

    let got = bft.shortest_walks(c, e, limits(1, 10), any, yes)?;
    assert!(got.walks.is_empty() && got.complete && !got.truncated);

    // start == goal needs no expansion, so even a zero cap finds the one walk
    for max_expansions in [usize::MAX, 0] {
        let lim = PathLimits {
            max_expansions,
            ..ALL
        };
        let got = bft.shortest_walks(i, i, lim, any, yes)?;
        assert_eq!(
            got.walks,
            [Walk {
                vertices: vec![i],
                edges: vec![]
            }]
        );
        assert!(got.complete && !got.truncated);
        assert_eq!(got.expansions, 0);
    }

    let got = bft.shortest_walks(h, i, ALL, any, yes)?;
    assert_eq!(show(&g, &got), ["H -tag#4-> I"]);

    let got = bft.shortest_walks(h, a, limits(usize::MAX, 3), kinds(&["tag", "link"]), yes)?;
    assert_eq!(
        show(&g, &got),
        [
            "H -tag#4-> I -link#2-> A",
            "H -tag#4-> I -link#3-> A",
            "H -tag#4-> I -link#8-> A",
        ]
    );
    assert!(!got.truncated && got.complete);

    let got = bft.shortest_walks(
        h,
        a,
        limits(usize::MAX, 3),
        kinds(&["tag", "link", "depends_on"]),
        yes,
    )?;
    assert_eq!(
        show(&g, &got),
        [
            "H -tag#4-> I -link#2-> A",
            "H -tag#4-> I -link#3-> A",
            "H -tag#4-> I -depends_on#7-> A",
        ]
    );
    assert!(got.truncated && got.complete);
    Ok(())
}

/// not from Silo: a tag vertex is a waypoint only when `expand_ok` allows it
#[test]
fn f1_tag_waypoint_needs_expand_ok() -> Result<()> {
    let mut g = G::<RevBuild>::undirected();
    let [p, t, q] = ["P", "#t", "Q"].map(|n| g.add_node(Weight(n)).unwrap());
    g.add_edge([p, t], Weight("tag"))?;
    g.add_edge([t, q], Weight("tag"))?;
    let any = |_: &EdgeId<usize>, _: &&str| true;
    let mut bft = g.bft();
    let no_tags = |_: &V, n: &&str| !n.starts_with('#');
    let got = bft.shortest_walks(p, q, ALL, any, no_tags)?;
    assert!(got.walks.is_empty() && got.complete);
    // a rejected vertex is not an expansion: only P counts, so a cap of 1 still completes
    assert_eq!(got.expansions, 1);
    let one = PathLimits {
        max_expansions: 1,
        ..ALL
    };
    let got = bft.shortest_walks(p, q, one, any, no_tags)?;
    assert!(got.walks.is_empty() && got.complete, "{got:?}");
    assert_eq!(got.expansions, 1);
    let got = bft.shortest_walks(p, q, ALL, any, |_, _| true)?;
    assert_eq!(show(&g, &got), ["P -tag#0-> #t -tag#1-> Q"]);
    // the goal and the start are never judged
    let got = bft.shortest_walks(t, q, ALL, any, |_, _| false)?;
    assert_eq!(show(&g, &got), ["#t -tag#1-> Q"]);
    Ok(())
}

/// acceptance 2: S joined to A1..A5, each Ai joined to G
#[test]
fn expansion_cap_stops_incomplete() -> Result<()> {
    let mut g = G::<RevBuild>::undirected();
    let s = g.add_node(Weight("S"))?;
    let a = (1..=5)
        .map(|_| g.add_node(Weight("A")))
        .collect::<Result<Vec<_>>>()?;
    let goal = g.add_node(Weight("G"))?;
    for ai in &a {
        g.add_edge([s, *ai], Weight("link"))?;
    }
    for ai in &a {
        g.add_edge([*ai, goal], Weight("link"))?;
    }
    let mut bft = g.bft();
    let capped = |max_expansions| PathLimits {
        max_expansions,
        ..ALL
    };
    let got = bft.shortest_walks(s, goal, capped(3), |_, _| true, |_, _| true)?;
    assert!(!got.complete && !got.truncated);
    assert_eq!(got.expansions, 3);
    let via = got
        .walks
        .iter()
        .map(|w| w.vertices.clone())
        .collect::<Vec<_>>();
    assert_eq!(via, [vec![s, a[0], goal], vec![s, a[1], goal]]);
    let got = bft.shortest_walks(s, goal, capped(6), |_, _| true, |_, _| true)?;
    assert!(got.complete && !got.truncated);
    assert_eq!((got.walks.len(), got.expansions), (5, 6));
    let got = bft.shortest_walks(s, goal, capped(0), |_, _| true, |_, _| true)?;
    assert!(!got.complete && got.walks.is_empty());
    Ok(())
}

/// acceptance 3: the tie {S, X, Y}, {X, Y, G} keeps [S, X, G] under any hasher
#[test]
fn ties_break_by_id_under_any_hasher() -> Result<()> {
    fn first<S: BuildHasher + Default>() -> Result<Vec<V>> {
        let mut g = G::<S>::undirected();
        let [s, x, y, goal] = ["S", "X", "Y", "G"].map(|n| g.add_node(Weight(n)).unwrap());
        g.add_edge([s, x, y], Weight("link"))?;
        g.add_edge([x, y, goal], Weight("link"))?;
        let got =
            g.bft()
                .shortest_walks(s, goal, limits(usize::MAX, 1), |_, _| true, |_, _| true)?;
        assert!(got.truncated);
        Ok(got.walks[0].vertices.clone())
    }
    let want = vec![V::new(0), V::new(1), V::new(3)];
    assert_eq!(first::<hashbrown::DefaultHashBuilder>()?, want);
    assert_eq!(first::<RevBuild>()?, want);
    Ok(())
}

#[test]
fn unknown_endpoints_and_directed_graphs_are_errors() -> Result<()> {
    let (g, [a, ..]) = silo_fixture::<RevBuild>()?;
    let mut bft = g.bft();
    for (s, t) in [(a, V::new(99)), (V::new(99), a), (V::new(99), V::new(99))] {
        let got = bft.shortest_walks(s, t, ALL, |_, _| true, |_, _| true);
        assert!(
            matches!(
                got,
                Err(Error::CoreError(rshyper_core::Error::NodeNotFound))
            ),
            "{got:?}"
        );
    }
    let mut d = rshyper_hmap::DiHyperMap::<(), ()>::directed();
    let [s, t] = [(); 2].map(|_| d.add_vertex().unwrap());
    d.add_link([s, t])?;
    // checked before the start == goal shortcut too
    for (s, t) in [(s, t), (s, s)] {
        let got = d.bft().shortest_walks(s, t, ALL, |_, _| true, |_, _| true);
        assert!(
            matches!(got, Err(Error::UnsupportedGraphKind(_))),
            "{got:?}"
        );
    }
    Ok(())
}

/// walk order compares the edge id before the vertex id at each step: here the two disagree
#[test]
fn walks_order_by_edge_then_vertex() -> Result<()> {
    fn walks<S: BuildHasher + Default>(max_paths: usize) -> Result<Vec<Vec<V>>> {
        let mut g = G::<S>::undirected();
        let [s, a, b, goal] = ["S", "A", "B", "G"].map(|n| g.add_node(Weight(n)).unwrap());
        for members in [[s, b], [s, a], [a, goal], [b, goal]] {
            g.add_edge(members, Weight("link"))?;
        }
        let got = g.bft().shortest_walks(
            s,
            goal,
            limits(usize::MAX, max_paths),
            |_, _| true,
            |_, _| true,
        )?;
        Ok(got.walks.into_iter().map(|w| w.vertices).collect())
    }
    let [s, a, b, goal] = [0, 1, 2, 3].map(V::new);
    // (e0, B, e3, G) < (e1, A, e2, G), although A < B
    let both = vec![vec![s, b, goal], vec![s, a, goal]];
    assert_eq!(walks::<hashbrown::DefaultHashBuilder>(9)?, both);
    assert_eq!(walks::<RevBuild>(9)?, both);
    assert_eq!(walks::<RevBuild>(1)?, both[..1]);
    Ok(())
}

/// `edge_ok` runs once per edge, even for an edge reached from both of its ends
#[test]
fn edge_ok_runs_once_per_edge() -> Result<()> {
    let (g, [_, c, e, ..]) = silo_fixture::<RevBuild>()?;
    let mut calls = BTreeMap::<EdgeId<usize>, usize>::new();
    // C -> E expands C, then I: the C-I edges are incident to both
    let got = g.bft().shortest_walks(
        c,
        e,
        ALL,
        |id, _| {
            *calls.entry(*id).or_default() += 1;
            true
        },
        |_, _| true,
    )?;
    assert_eq!(got.walks.len(), 4);
    assert!(calls.len() >= 9, "{calls:?}");
    assert!(calls.values().all(|&n| n == 1), "{calls:?}");
    Ok(())
}

type Domains = Vec<(EdgeId<usize>, BTreeSet<V>)>;

/// every shortest walk by brute force: shortest walks are vertex-simple, so enumerate the
/// simple walks from `start` and keep the shortest that end at `goal`
fn brute(
    edges: &Domains,
    start: V,
    goal: V,
    depth: usize,
    keep_edge: &dyn Fn(&EdgeId<usize>) -> bool,
    expand: &dyn Fn(&V) -> bool,
) -> Vec<Walk<usize>> {
    if start == goal {
        return vec![Walk {
            vertices: vec![start],
            edges: vec![],
        }];
    }
    fn go(
        edges: &Domains,
        goal: V,
        depth: usize,
        keep_edge: &dyn Fn(&EdgeId<usize>) -> bool,
        expand: &dyn Fn(&V) -> bool,
        walk: &mut Walk<usize>,
        out: &mut Vec<Walk<usize>>,
    ) {
        let v = *walk.vertices.last().unwrap();
        if walk.edges.len() == depth {
            return;
        }
        for (id, dom) in edges
            .iter()
            .filter(|(id, d)| keep_edge(id) && d.contains(&v))
        {
            for u in dom {
                if *u == v || walk.vertices.contains(u) {
                    continue;
                }
                walk.vertices.push(*u);
                walk.edges.push(*id);
                if *u == goal {
                    out.push(walk.clone());
                } else if expand(u) {
                    go(edges, goal, depth, keep_edge, expand, walk, out);
                }
                walk.vertices.pop();
                walk.edges.pop();
            }
        }
    }
    let mut out = Vec::new();
    let mut walk = Walk {
        vertices: vec![start],
        edges: vec![],
    };
    go(edges, goal, depth, keep_edge, expand, &mut walk, &mut out);
    let shortest = out.iter().map(|w| w.edges.len()).min();
    out.retain(|w| Some(w.edges.len()) == shortest);
    out.sort_by_key(key);
    out
}

/// the interleaved `(e1, v1, e2, v2, …)` ids
fn key(w: &Walk<usize>) -> Vec<usize> {
    w.edges
        .iter()
        .zip(&w.vertices[1..])
        .flat_map(|(e, v)| [**e, **v])
        .collect()
}

fn random_graph<S: BuildHasher + Default>(seed: u64) -> Result<(G<S>, Vec<V>, Domains)> {
    let mut rng = SplitMix(seed);
    let mut g = G::<S>::undirected();
    let n = 1 + rng.below(7);
    let v = (0..n)
        .map(|_| g.add_node(Weight("v")))
        .collect::<Result<Vec<_>>>()?;
    let mut edges = Vec::new();
    for _ in 0..rng.below(10) {
        let dom = (0..1 + rng.below(4))
            .map(|_| v[rng.below(n)])
            .collect::<BTreeSet<_>>();
        edges.push((g.add_edge(dom.iter().copied(), Weight("e"))?, dom));
    }
    Ok((g, v, edges))
}

/// acceptance 4: 2,000 fixed-seed small graphs against the brute force, under two hashers
#[test]
fn matches_brute_force_on_small_graphs() -> Result<()> {
    for seed in 0..2_000u64 {
        let (a, v, edges) = random_graph::<hashbrown::DefaultHashBuilder>(seed)?;
        let (b, _, _) = random_graph::<RevBuild>(seed)?;
        let mut rng = SplitMix(!seed);
        let (start, goal) = (v[rng.below(v.len())], v[rng.below(v.len())]);
        let max_depth = [0, 1, 2, 3, usize::MAX][rng.below(5)];
        let max_paths = [0, 1, 2, usize::MAX][rng.below(4)];
        let (em, vm) = (2 + rng.below(3), 2 + rng.below(3));
        let keep_edge = |e: &EdgeId<usize>| (**e + seed as usize) % em != 0;
        let expand = |u: &V| (**u + seed as usize) % vm != 0;
        let all = brute(&edges, start, goal, max_depth, &keep_edge, &expand);
        let lim = PathLimits {
            max_depth,
            max_paths,
            max_expansions: usize::MAX,
        };
        for (name, got) in [
            (
                "default",
                a.bft()
                    .shortest_walks(start, goal, lim, |e, _| keep_edge(e), |u, _| expand(u))?,
            ),
            (
                "RevBuild",
                b.bft()
                    .shortest_walks(start, goal, lim, |e, _| keep_edge(e), |u, _| expand(u))?,
            ),
        ] {
            let want = all.iter().take(max_paths).cloned().collect::<Vec<_>>();
            assert_eq!(got.walks, want, "seed {seed} ({name})");
            assert_eq!(got.truncated, all.len() > max_paths, "seed {seed} ({name})");
            assert!(got.complete, "seed {seed} ({name})");
        }
    }
    Ok(())
}
