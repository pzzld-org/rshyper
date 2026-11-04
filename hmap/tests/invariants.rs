/*
    Appellation: invariants <tests>
    Contrib: @FL03
*/
//! RED witnesses for known `rshyper-hmap` invariant defects (#104 ordered iterators,
//! #105 invariant-safe mutations). Each test asserts the CORRECT behaviour and is expected to
//! fail on the current code; it should pass once the defect is fixed.
use core::hash::{BuildHasher, Hasher};
use rshyper_core::idx::{EdgeId, IndexError, IndexFrame, VertexId};
use rshyper_core::{Error, Result, Weight};
use rshyper_hmap::UnHyperMap;

type Graph = UnHyperMap<usize, usize, usize, RevBuild>;

/// deterministic hasher: hashbrown buckets by low hash bits, so the raw iteration order of the
/// maps becomes the reverse of the id order, exposing any reliance on the raw order.
#[derive(Clone, Copy, Default)]
struct RevBuild;

#[derive(Default)]
struct RevHasher(u64);

impl Hasher for RevHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(u64::from(b));
        }
    }
    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64;
    }
    fn finish(&self) -> u64 {
        !self.0
    }
}

impl BuildHasher for RevBuild {
    type Hasher = RevHasher;

    fn build_hasher(&self) -> RevHasher {
        RevHasher::default()
    }
}

fn raw_node_ids(g: &Graph) -> Vec<VertexId<usize>> {
    g.iter_nodes().map(|(id, _)| *id).collect()
}

fn raw_edge_ids(g: &Graph) -> Vec<EdgeId<usize>> {
    g.iter_edges().map(|(id, _)| *id).collect()
}

/// #104: sequential node iterators must yield every live node, in insertion order.
#[test]
fn seq_nodes_yield_all_live_in_insertion_order() -> Result<()> {
    let mut g = Graph::undirected();
    for _ in 0..8 {
        g.add_vertex()?;
    }
    let expected = g.history().nodes().clone();
    assert_ne!(
        raw_node_ids(&g),
        expected,
        "precondition: raw hash order must differ from insertion order"
    );
    let by_keys: Vec<_> = g.seq_iter_nodes().map(|(id, _)| *id).collect();
    assert_eq!(
        by_keys, expected,
        "seq_iter_nodes must yield all live nodes in insertion order"
    );
    let by_points: Vec<_> = g.seq_iter_points().map(|n| *n.id()).collect();
    assert_eq!(
        by_points, expected,
        "seq_iter_points must yield all live nodes in insertion order"
    );
    Ok(())
}

/// #104: sequential edge iterators must yield every live edge, in insertion order.
#[test]
fn seq_edges_yield_all_live_in_insertion_order() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let v2 = g.add_vertex()?;
    let domains = [
        vec![v0, v1],
        vec![v1, v2],
        vec![v0, v2],
        vec![v0],
        vec![v1],
        vec![v2],
    ];
    for domain in domains {
        g.add_link(domain)?;
    }
    let expected = g.history().edges().clone();
    assert_ne!(
        raw_edge_ids(&g),
        expected,
        "precondition: raw hash order must differ from insertion order"
    );
    let by_keys: Vec<_> = g.seq_iter_edges().map(|(id, _)| *id).collect();
    assert_eq!(
        by_keys, expected,
        "seq_iter_edges must yield all live edges in insertion order"
    );
    let by_facets: Vec<_> = g.seq_iter_facets().map(|e| *e.id()).collect();
    assert_eq!(
        by_facets, expected,
        "seq_iter_facets must yield all live edges in insertion order"
    );
    assert_eq!(by_facets.len(), g.size());
    Ok(())
}

/// #104: removing a middle node and edge must not stop or corrupt the sequential iterators.
#[test]
fn seq_iter_after_remove_skips_removed_continues() -> Result<()> {
    let mut g = Graph::undirected();
    let v: Vec<VertexId<usize>> = (0..5).map(|_| g.add_vertex()).collect::<Result<_>>()?;
    let e0 = g.add_link([v[0], v[1]])?;
    let e1 = g.add_link([v[0], v[4]])?;
    let e2 = g.add_link([v[3], v[4]])?;
    g.remove_edge(&e1)?;
    // v[2] is isolated, so removing it cascades to no edges
    g.remove_node(&v[2])?;
    let live_nodes = vec![v[0], v[1], v[3], v[4]];
    let live_edges = vec![e0, e2];
    assert_ne!(
        raw_node_ids(&g),
        live_nodes,
        "precondition: raw hash order must differ from insertion order"
    );
    let nodes: Vec<_> = g.seq_iter_nodes().map(|(id, _)| *id).collect();
    assert_eq!(
        nodes, live_nodes,
        "seq_iter_nodes must skip the removed node"
    );
    assert_eq!(nodes.len(), g.order());
    let points: Vec<_> = g.seq_iter_points().map(|n| *n.id()).collect();
    assert_eq!(
        points, live_nodes,
        "seq_iter_points must skip the removed node"
    );
    let edges: Vec<_> = g.seq_iter_edges().map(|(id, _)| *id).collect();
    assert_eq!(
        edges, live_edges,
        "seq_iter_edges must skip the removed edge"
    );
    assert_eq!(edges.len(), g.size());
    Ok(())
}

/// #105: removing a node must prune the history of every edge it cascades away.
#[test]
fn remove_node_prunes_cascaded_edge_history() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let v2 = g.add_vertex()?;
    let e0 = g.add_link([v0, v1])?;
    let _e1 = g.add_link([v1, v2])?;
    let e2 = g.add_link([v0, v2])?;
    g.remove_node(&v1)?;
    assert!(
        !g.contains_edge(&e0),
        "edge containing the removed node must be gone"
    );
    assert_eq!(g.size(), 1);
    assert_eq!(
        g.history().edges().len(),
        g.size(),
        "history must not retain edges removed by the cascade"
    );
    assert_eq!(g.seq_iter_edge_keys().count(), g.size());
    assert!(g.seq_iter_edge_keys().all(|k| g.contains_edge(k)));
    assert_eq!(g.history().edges(), &vec![e2]);
    Ok(())
}

/// #105: adding an empty edge must fail atomically, leaving history and size untouched.
#[test]
fn add_edge_empty_is_error_atomic() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    g.add_link([v0])?;
    let before = g.history().clone();
    let size = g.size();
    assert!(g.add_link(Vec::<VertexId<usize>>::new()).is_err());
    assert_eq!(
        g.history(),
        &before,
        "a failed add must not mutate the history"
    );
    assert_eq!(g.size(), size);
    Ok(())
}

/// #105: an edge whose domain has only nonexistent vertices must fail atomically.
#[test]
fn add_edge_all_missing_vertices_is_error_atomic() -> Result<()> {
    let mut g = Graph::undirected();
    g.add_vertex()?;
    let before = g.history().clone();
    let size = g.size();
    assert!(g.add_link([VertexId::new(999)]).is_err());
    assert_eq!(
        g.history(),
        &before,
        "a failed add must not mutate the history"
    );
    assert_eq!(g.size(), size);
    Ok(())
}

/// #105: an edge referencing a missing vertex must be rejected, not silently truncated.
#[test]
fn add_edge_with_missing_vertex_is_rejected() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let before = g.history().clone();
    let size = g.size();
    assert!(
        g.add_link([v0, VertexId::new(999)]).is_err(),
        "a domain containing a missing vertex must be rejected"
    );
    assert_eq!(
        g.history(),
        &before,
        "a rejected add must not mutate the history"
    );
    assert_eq!(g.size(), size);
    Ok(())
}

/// #105: merging an edge with itself must fail without consuming the edge.
#[test]
fn merge_edge_with_itself_is_error_atomic() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let e = g.add_edge([v0, v1], Weight(3))?;
    let before = g.history().clone();
    let size = g.size();
    assert!(g.merge_edges(&e, &e).is_err());
    assert!(
        g.contains_edge(&e),
        "a failed merge must not remove the edge"
    );
    assert_eq!(g.size(), size);
    assert_eq!(
        g.history(),
        &before,
        "a failed merge must not mutate the history"
    );
    Ok(())
}

/// #105: merging with a missing edge must fail without consuming the present edge.
#[test]
fn merge_with_missing_edge_is_error_atomic() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let e = g.add_edge([v0, v1], Weight(3))?;
    let before = g.history().clone();
    let size = g.size();
    assert!(g.merge_edges(&e, &EdgeId::new(999)).is_err());
    assert!(
        g.contains_edge(&e),
        "a failed merge must not remove the present edge"
    );
    assert_eq!(g.size(), size);
    assert_eq!(
        g.history(),
        &before,
        "a failed merge must not mutate the history"
    );
    Ok(())
}

/// #105: retaining edges must prune the edge history to the live ids, in original order.
#[test]
fn retain_edges_prunes_history() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let e0 = g.add_link([v0, v1])?;
    let e1 = g.add_link([v0])?;
    let e2 = g.add_link([v1])?;
    g.retain_edges(|id, _| *id != e1);
    assert_eq!(
        g.history().edges(),
        &vec![e0, e2],
        "history must contain exactly the live edge ids, in original order"
    );
    Ok(())
}

/// control: a valid merge of two distinct live edges keeps history in sync with the graph.
#[test]
fn merge_edges_valid_updates_history() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let v2 = g.add_vertex()?;
    let e1 = g.add_edge([v0, v1], Weight(10))?;
    let e2 = g.add_edge([v1, v2], Weight(20))?;
    let em = g.merge_edges(&e1, &e2)?;
    assert!(!g.contains_edge(&e1) && !g.contains_edge(&e2));
    let live: Vec<_> = g.iter_edges().map(|(id, _)| *id).collect();
    assert_eq!(live, vec![em]);
    assert_eq!(
        g.history().edges(),
        &vec![em],
        "history must contain exactly the live edge ids"
    );
    Ok(())
}

/// #104: history ids without a live entry (e.g. written through `history_mut`) are skipped by
/// every sequential iterator, including the key iterators.
#[test]
fn seq_iter_skips_dangling_history_ids() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let e0 = g.add_link([v0, v1])?;
    g.history_mut().nodes_mut().insert(0, VertexId::new(999));
    g.history_mut().edges_mut().insert(0, EdgeId::new(999));
    let vertices: Vec<_> = g.seq_iter_vertices().cloned().collect();
    assert_eq!(vertices, vec![v0, v1]);
    assert_eq!(g.seq_iter_points().count(), g.order());
    let edges: Vec<_> = g.seq_iter_edge_keys().cloned().collect();
    assert_eq!(edges, vec![e0]);
    assert_eq!(g.seq_iter_facets().count(), g.size());
    Ok(())
}

/// #114: allocating an id that is already in the history (here after rewinding the cursor) must
/// fail with `DuplicateIndex` and leave the graph, its history and its position untouched.
#[test]
fn rewound_allocation_is_error_atomic() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    g.add_link([v0, v1])?;
    g.set_position(IndexFrame::zero());
    let position = *g.position();
    let nodes = g.history().nodes().clone();
    let edges = g.history().edges().clone();
    let (order, size) = (g.order(), g.size());
    assert!(matches!(
        g.add_vertex(),
        Err(Error::IndexError(IndexError::DuplicateIndex))
    ));
    assert!(matches!(
        g.add_edge([v0, v1], Weight(0)),
        Err(Error::IndexError(IndexError::DuplicateIndex))
    ));
    assert_eq!(g.order(), order);
    assert_eq!(g.size(), size);
    assert_eq!(g.history().nodes(), &nodes);
    assert_eq!(g.history().edges(), &edges);
    assert_eq!(g.position(), &position);
    Ok(())
}

/// #114: a merge whose id allocation fails must not consume either operand.
#[test]
fn rewound_merge_is_error_atomic() -> Result<()> {
    let mut g = Graph::undirected();
    let v0 = g.add_vertex()?;
    let v1 = g.add_vertex()?;
    let v2 = g.add_vertex()?;
    g.add_edge([v0, v1], Weight(1))?;
    let e1 = g.add_edge([v1, v2], Weight(2))?;
    let e2 = g.add_edge([v0, v2], Weight(3))?;
    g.set_position(IndexFrame::zero());
    let position = *g.position();
    let nodes = g.history().nodes().clone();
    let edges = g.history().edges().clone();
    let size = g.size();
    assert!(matches!(
        g.merge_edges(&e1, &e2),
        Err(Error::IndexError(IndexError::DuplicateIndex))
    ));
    assert!(
        g.contains_edge(&e1) && g.contains_edge(&e2),
        "a failed merge must not remove its operands"
    );
    assert_eq!(g.size(), size);
    assert_eq!(g.history().nodes(), &nodes);
    assert_eq!(g.history().edges(), &edges);
    assert_eq!(g.position(), &position);
    Ok(())
}
