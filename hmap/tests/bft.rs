/*
    Appellation: search <test>
    Contrib: @FL03
*/
mod common;

use common::{RevBuild, SplitMix};
use core::hash::BuildHasher;
use rshyper_algo::Error;
use rshyper_core::idx::{EdgeId, VertexId};
use rshyper_core::{Result, Weight};
use rshyper_hmap::{HyperMap, UnHyperMap};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[test]
fn test_breadth_first_traversal() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create a simple hypergraph
    // 0 -- 1 -- 3
    //  \  /
    //   2 -- 4

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;

    let _e1 = graph.add_link(vec![v0, v1, v2]).unwrap();
    let _e2 = graph.add_link(vec![v1, v3]).unwrap();
    let _e3 = graph.add_link(vec![v2, v4]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // Verify traversal order - should be breadth-first
    // (note: exact order might vary for vertices at the same level)
    assert_eq!(path.len(), 5, "Should visit all 5 vertices");
    assert_eq!(path[0], v0, "Should start with v0");

    // All vertices should be visited
    let visited = bft.has_visited(&v0)
        && bft.has_visited(&v1)
        && bft.has_visited(&v2)
        && bft.has_visited(&v3)
        && bft.has_visited(&v4);
    assert!(visited, "All vertices should be visited");

    Ok(())
}

#[test]
fn test_bft_cyclic_graph() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create a cyclic hypergraph
    // 0 -- 1 -- 2
    // |         |
    // +----3----+

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;

    let _e1 = graph.add_link(vec![v0, v1]).unwrap();
    let _e2 = graph.add_link(vec![v1, v2]).unwrap();
    let _e3 = graph.add_link(vec![v2, v3]).unwrap();
    let _e4 = graph.add_link(vec![v3, v0]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // BFT should visit each vertex exactly once
    assert_eq!(path.len(), 4, "Should visit each vertex exactly once");

    // All vertices should be visited
    let visited = bft.has_visited(&v0)
        && bft.has_visited(&v1)
        && bft.has_visited(&v2)
        && bft.has_visited(&v3);
    assert!(visited, "All vertices should be visited");

    Ok(())
}

#[test]
fn test_bft_disconnected_graph() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create a disconnected hypergraph
    // 0 -- 1    2 -- 3

    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;

    let _e1 = graph.add_link(vec![v0, v1]).unwrap();
    let _e2 = graph.add_link(vec![v2, v3]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // BFT should only visit connected vertices (v0 and v1)
    assert_eq!(path.len(), 2, "Should only visit connected vertices");
    assert!(path.contains(&v0));
    assert!(path.contains(&v1));

    // v2 and v3 should not be visited
    let visited = bft.has_visited(&v2) || bft.has_visited(&v3);
    assert!(!visited, "v2 and v3 should not be visited");

    Ok(())
}

#[test]
fn test_bft_hyperedge_with_multiple_vertices() -> Result<()> {
    let mut graph = HyperMap::<usize, usize>::undirected();

    // Create a hypergraph with a large hyperedge
    // connecting multiple vertices
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;

    // Single hyperedge connecting all vertices
    let _e1 = graph.add_link(vec![v0, v1, v2, v3, v4]).unwrap();

    let mut bft = graph.bft();
    let path = bft.search(v0).unwrap();

    // All vertices should be in the path and at the same level
    assert_eq!(path.len(), 5, "All vertices should be visited");
    assert_eq!(path[0], v0, "Should start with v0");

    // The remaining vertices should all be at the same level
    // (order might vary but they should all be visited)
    let visited = bft.has_visited(&v0)
        && bft.has_visited(&v1)
        && bft.has_visited(&v2)
        && bft.has_visited(&v3)
        && bft.has_visited(&v4);

    assert!(visited, "All vertices should be visited");
    Ok(())
}

// #119: `search_bounded`

type V = VertexId<usize>;
type G<S> = UnHyperMap<usize, usize, usize, S>;
/// each edge's id and the members it was built from
type Domains = Vec<(EdgeId<usize>, Vec<V>)>;

/// a path 0 - 1 - 2 - 3 plus the triangle edge {0, 4, 5}; vertex `i` weighs `10 i` and the
/// edges weigh 1, 2, 3, 4 in that order
fn fixture<S: BuildHasher + Default>() -> Result<(G<S>, Vec<V>)> {
    let mut g = G::<S>::undirected();
    let v = (0..6)
        .map(|i| g.add_node(Weight(10 * i)))
        .collect::<Result<Vec<_>>>()?;
    g.add_edge([v[0], v[1]], Weight(1))?;
    g.add_edge([v[1], v[2]], Weight(2))?;
    g.add_edge([v[2], v[3]], Weight(3))?;
    g.add_edge([v[0], v[4], v[5]], Weight(4))?;
    Ok((g, v))
}

#[test]
fn bounded_depth_zero_returns_the_start() -> Result<()> {
    let (g, v) = fixture::<RevBuild>()?;
    let got = g
        .bft()
        .search_bounded(v[1], 0, usize::MAX, |_, _| true, |_, _| true)?;
    assert_eq!(got, [v[1]]);
    Ok(())
}

#[test]
fn bounded_depth_one_returns_only_co_members() -> Result<()> {
    let (g, v) = fixture::<RevBuild>()?;
    let got = g
        .bft()
        .search_bounded(v[0], 1, usize::MAX, |_, _| true, |_, _| true)?;
    assert_eq!(got, [v[0], v[1], v[4], v[5]]);
    let got = g
        .bft()
        .search_bounded(v[0], 2, usize::MAX, |_, _| true, |_, _| true)?;
    assert_eq!(got, [v[0], v[1], v[4], v[5], v[2]]);
    Ok(())
}

#[test]
fn bounded_limit_truncates_the_nearest_first_order() -> Result<()> {
    let (g, v) = fixture::<RevBuild>()?;
    let mut bft = g.bft();
    assert!(
        bft.search_bounded(v[0], 9, 0, |_, _| true, |_, _| true)?
            .is_empty()
    );
    assert_eq!(
        bft.search_bounded(v[0], 9, 1, |_, _| true, |_, _| true)?,
        [v[0]]
    );
    assert_eq!(
        bft.search_bounded(v[0], 9, 3, |_, _| true, |_, _| true)?,
        [v[0], v[1], v[4]]
    );
    let all = bft.search_bounded(v[0], 9, usize::MAX, |_, _| true, |_, _| true)?;
    assert_eq!(all, [v[0], v[1], v[4], v[5], v[2], v[3]]);
    Ok(())
}

#[test]
fn bounded_filtered_edge_disconnects() -> Result<()> {
    let (g, v) = fixture::<RevBuild>()?;
    let mut calls = BTreeMap::<EdgeId<usize>, usize>::new();
    // weight 2 is the bridge {1, 2}
    let got = g.bft().search_bounded(
        v[0],
        usize::MAX,
        usize::MAX,
        |e, w| {
            *calls.entry(*e).or_default() += 1;
            *w != 2
        },
        |_, _| true,
    )?;
    assert_eq!(got, [v[0], v[1], v[4], v[5]]);
    assert!(
        calls.values().all(|&n| n == 1),
        "edge filter ran twice: {calls:?}"
    );
    Ok(())
}

#[test]
fn bounded_filtered_vertex_is_neither_returned_nor_expanded() -> Result<()> {
    let (g, v) = fixture::<RevBuild>()?;
    let mut calls = BTreeMap::<V, usize>::new();
    let got = g.bft().search_bounded(
        v[0],
        usize::MAX,
        usize::MAX,
        |_, _| true,
        |u, w| {
            *calls.entry(*u).or_default() += 1;
            *w != 10
        },
    )?;
    // 2 and 3 are reachable only through 1, and the filter never judges the start
    assert_eq!(got, [v[0], v[4], v[5]]);
    assert!(
        calls.values().all(|&n| n == 1),
        "vertex filter ran twice: {calls:?}"
    );
    assert!(!calls.contains_key(&v[0]));
    let got = g
        .bft()
        .search_bounded(v[0], 9, 9, |_, _| true, |_, _| false)?;
    assert_eq!(got, [v[0]]);
    Ok(())
}

/// a rejected edge or vertex reachable from two expanded vertices is still judged once
#[test]
fn bounded_filters_judge_each_edge_and_vertex_once() -> Result<()> {
    // a triangle whose edge {1, 2} is rejected, reached from both 1 and 2
    let mut g = G::<RevBuild>::undirected();
    let v = (0..3).map(|_| g.add_vertex()).collect::<Result<Vec<_>>>()?;
    g.add_edge([v[0], v[1]], Weight(1))?;
    g.add_edge([v[0], v[2]], Weight(1))?;
    g.add_edge([v[1], v[2]], Weight(0))?;
    let mut calls = BTreeMap::<EdgeId<usize>, usize>::new();
    let got = g.bft().search_bounded(
        v[0],
        usize::MAX,
        usize::MAX,
        |e, w| {
            *calls.entry(*e).or_default() += 1;
            *w != 0
        },
        |_, _| true,
    )?;
    assert_eq!(got, [v[0], v[1], v[2]]);
    assert_eq!(calls.values().copied().collect::<Vec<_>>(), [1, 1, 1]);
    // a 4-cycle 0-1-3-2-0 whose vertex 3 is rejected, reached from both 1 and 2
    let mut g = G::<RevBuild>::undirected();
    let v = (0..4)
        .map(|i| g.add_node(Weight(i)))
        .collect::<Result<Vec<_>>>()?;
    for [a, b] in [[0, 1], [0, 2], [1, 3], [2, 3]] {
        g.add_link([v[a], v[b]])?;
    }
    let mut calls = BTreeMap::<V, usize>::new();
    let got = g.bft().search_bounded(
        v[0],
        usize::MAX,
        usize::MAX,
        |_, _| true,
        |u, w| {
            *calls.entry(*u).or_default() += 1;
            *w != 3
        },
    )?;
    assert_eq!(got, [v[0], v[1], v[2]]);
    assert_eq!(calls, BTreeMap::from([(v[1], 1), (v[2], 1), (v[3], 1)]));
    Ok(())
}

#[test]
fn bounded_missing_start_is_not_found() -> Result<()> {
    let (g, _) = fixture::<RevBuild>()?;
    // the checks come before the depth and limit shortcuts
    for (depth, limit) in [(1, 1), (0, 9), (9, 0), (0, 0)] {
        let got = g
            .bft()
            .search_bounded(VertexId::new(99), depth, limit, |_, _| true, |_, _| true);
        assert!(matches!(got, Err(Error::NotFound(_))), "{got:?}");
    }
    Ok(())
}

#[test]
fn bounded_rejects_directed_graphs() -> Result<()> {
    let mut g = rshyper_hmap::DiHyperMap::<(), ()>::directed();
    let [s, t] = [(); 2].map(|_| g.add_vertex().unwrap());
    g.add_link([s, t])?;
    for (depth, limit) in [(1, 9), (0, 9), (9, 0), (0, 0)] {
        let got = g
            .bft()
            .search_bounded(s, depth, limit, |_, _| true, |_, _| true);
        assert!(
            matches!(got, Err(Error::UnsupportedGraphKind(_))),
            "{got:?}"
        );
    }
    Ok(())
}

#[test]
fn bounded_search_leaves_plain_search_unchanged() -> Result<()> {
    let (g, v) = fixture::<RevBuild>()?;
    let mut bft = g.bft();
    let bounded = bft.search_bounded(v[0], 1, 2, |_, _| true, |_, _| true)?;
    assert_eq!(bounded, [v[0], v[1]]);
    let mut plain = bft.search(v[0])?;
    plain.sort();
    assert_eq!(plain, v);
    Ok(())
}

/// the reference: plain BFS over an explicit adjacency of kept edges and kept vertices, then
/// (distance, id) order, truncated
fn oracle(
    edges: &[(EdgeId<usize>, Vec<V>)],
    start: V,
    depth: usize,
    limit: usize,
    keep_edge: &dyn Fn(&EdgeId<usize>) -> bool,
    keep_vertex: &dyn Fn(&V) -> bool,
) -> Vec<V> {
    let keep_vertex = |u: &V| *u == start || keep_vertex(u);
    let mut adj = BTreeMap::<V, BTreeSet<V>>::new();
    for (_, dom) in edges.iter().filter(|(id, _)| keep_edge(id)) {
        for a in dom.iter().filter(|a| keep_vertex(a)) {
            for b in dom.iter().filter(|b| keep_vertex(b)) {
                adj.entry(*a).or_default().insert(*b);
            }
        }
    }
    let mut dist = BTreeMap::from([(start, 0usize)]);
    let mut queue = VecDeque::from([start]);
    while let Some(u) = queue.pop_front() {
        let d = dist[&u];
        for w in adj.get(&u).into_iter().flatten() {
            if !dist.contains_key(w) {
                dist.insert(*w, d + 1);
                queue.push_back(*w);
            }
        }
    }
    let mut out = dist
        .into_iter()
        .filter(|&(_, d)| d <= depth)
        .map(|(u, d)| (d, u))
        .collect::<Vec<_>>();
    out.sort();
    out.into_iter().map(|(_, u)| u).take(limit).collect()
}

fn random_graph<S: BuildHasher + Default>(rng: &mut SplitMix) -> Result<(G<S>, Vec<V>, Domains)> {
    let mut g = G::<S>::undirected();
    let n = 1 + rng.below(40);
    let v = (0..n).map(|_| g.add_vertex()).collect::<Result<Vec<_>>>()?;
    let mut edges = Vec::new();
    for _ in 0..rng.below(2 * n + 1) {
        let dom = (0..1 + rng.below(4))
            .map(|_| v[rng.below(n)])
            .collect::<Vec<_>>();
        let id = g.add_link(dom.iter().copied())?;
        edges.push((id, dom));
    }
    Ok((g, v, edges))
}

/// random graphs, bounds and filters against the oracle, under two hashers that iterate in
/// different orders
#[test]
fn bounded_matches_the_oracle_under_both_hashers() -> Result<()> {
    for seed in 0..300u64 {
        let mut rng = SplitMix(seed);
        let (a, va, edges) = random_graph::<hashbrown::DefaultHashBuilder>(&mut SplitMix(seed))?;
        let (b, vb, _) = random_graph::<RevBuild>(&mut rng)?;
        assert_eq!(va, vb);
        let start = va[rng.below(va.len())];
        let depth = [0, 1, 2, 3, usize::MAX][rng.below(5)];
        let limit = [0, 1, 2, 5, usize::MAX][rng.below(5)];
        let (em, vm) = (1 + rng.below(4) as u64, 1 + rng.below(4) as u64);
        let keep_edge = |e: &EdgeId<usize>| (**e as u64 ^ seed) % em != 1;
        let keep_vertex = |u: &V| (**u as u64 ^ seed) % vm != 1;
        let want = oracle(&edges, start, depth, limit, &keep_edge, &keep_vertex);
        let (fe, fv) = (
            |e: &EdgeId<usize>, _: &usize| keep_edge(e),
            |u: &V, _: &usize| keep_vertex(u),
        );
        let got_a = a.bft().search_bounded(start, depth, limit, fe, fv)?;
        let got_b = b.bft().search_bounded(start, depth, limit, fe, fv)?;
        assert_eq!(got_a, want, "seed {seed} (default hasher)");
        assert_eq!(got_b, want, "seed {seed} (RevBuild)");
    }
    Ok(())
}
