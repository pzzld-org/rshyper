/*
    appellation: dijkstra <module>
    authors: @FL03
*/
#![allow(unused_variables)]
use rshyper_core::Result;
use rshyper_hmap::HyperMap;

#[test]
fn test_dijkstra_direct_edge() -> Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some nodes
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    // add a direct edge from v0 to v2
    graph.add_link([v0, v1, v2])?;
    // find the shortest path from v0 to v2
    let path = graph.dijkstra().find_path(v0, v2)?;
    // verify the results
    assert_eq!(path, [v0, v2], "Direct edge should return direct path");
    // return
    Ok(())
}

#[test]
fn test_dijkstra_shortest_path() -> Result<()> {
    // Initialize a new graph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add vertices to the graph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    let v5 = graph.add_vertex()?;
    // Add edges to the graph
    graph.add_link([v0, v1])?;
    graph.add_link([v1, v2])?;
    graph.add_link([v2, v5])?;
    graph.add_link([v5, v4])?;
    graph.add_link([v0, v4])?;
    graph.add_link([v4, v3])?;
    graph.add_link([v0, v5, v3])?;

    // Use Dijkstra's algorithm to find the shortest path from v0 to v3
    let path = graph.dijkstra().find_path(v0, v3)?;
    // Dijkstra should find the shortest path (v0 -> v4 -> v3)
    assert_eq!(
        path,
        [v0, v3],
        "Path should start with v0, go through v4 and end with v3"
    );

    Ok(())
}

#[test]
fn test_dijkstra_no_path() -> Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add vertices to the graph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    // Add edges to the graph
    let e0 = graph.add_link([v0, v1])?;
    // v2 and v3 are not connected to v0 or v1
    let e1 = graph.add_link([v0, v2])?;
    // v2 and v3 are disconnected from v0 and v1
    // find the shortest path between v0 and v3
    let path = graph.dijkstra().find_path(v0, v3);
    // verify that there is no path between v0 and v3
    assert!(
        path.is_err() || path.as_ref().unwrap().is_empty(),
        "Should return error or empty path when no path exists"
    );

    Ok(())
}

#[test]
fn test_dijkstra_same_start_end() -> Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some vertices
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    // add an edge between v0 and v1
    let _e0 = graph.add_link([v0, v1])?;
    // compute a self-loop from v0 -> v0
    let path = graph.dijkstra().find_path(v0, v0)?;
    // verify the path has one item, which is the node itself
    assert_eq!(
        path,
        [v0],
        "Path from a node to itself should be just that node"
    );

    Ok(())
}

#[test]
fn test_dijkstra_multiple_paths() -> Result<()> {
    // initializea new undirected hashgraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // add some nodes to the graph
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    let v4 = graph.add_vertex()?;
    // Add edges to the graph
    let e0 = graph.add_link([v0, v1, v3])?;
    let e1 = graph.add_link([v0, v2])?;
    let e2 = graph.add_link([v2, v4])?;
    let e3 = graph.add_link([v3, v4])?;
    // find the shortest path from v0 to v4
    let path = graph.dijkstra().find_path(v0, v4)?;

    // Both [v0, v2, v4] and [v0, v3, v4] are valid shortest paths
    assert!(
        path == [v0, v2, v4] || path == [v0, v3, v4],
        "Path should be one of the two shortest paths"
    );

    Ok(())
}

/// #122: the queue used to pop the most expensive vertex first, returning a cost-11 path.
#[test]
fn test_dijkstra_returns_the_cheapest_path() -> Result<()> {
    use rshyper_core::Weight;
    let mut graph = HyperMap::<(), usize>::undirected();
    let [s, a, b, t] = [(); 4].map(|_| graph.add_vertex().unwrap());
    graph.add_edge([s, a], Weight(1))?;
    graph.add_edge([s, b], Weight(10))?;
    graph.add_edge([a, b], Weight(1))?;
    graph.add_edge([b, t], Weight(1))?;
    assert_eq!(graph.dijkstra().find_path(s, t)?, [s, a, b, t]);
    Ok(())
}

/// #122: zero-weight edges are valid and cost nothing.
#[test]
fn test_dijkstra_zero_weights() -> Result<()> {
    use rshyper_core::Weight;
    let mut graph = HyperMap::<(), usize>::undirected();
    let [s, a, t] = [(); 3].map(|_| graph.add_vertex().unwrap());
    graph.add_edge([s, t], Weight(1))?;
    graph.add_edge([s, a], Weight(0))?;
    graph.add_edge([a, t], Weight(0))?;
    assert_eq!(graph.dijkstra().find_path(s, t)?, [s, a, t]);
    Ok(())
}

/// #122: negative and NaN weights are rejected before the search, even when the cheap path
/// would reach the goal without touching them.
#[test]
fn test_dijkstra_rejects_invalid_weights() -> Result<()> {
    use rshyper_algo::Error;
    use rshyper_core::Weight;

    let mut graph = HyperMap::<(), i64>::undirected();
    let [s, a, t] = [(); 3].map(|_| graph.add_vertex().unwrap());
    graph.add_edge([s, t], Weight(1))?;
    graph.add_edge([s, a], Weight(2))?;
    graph.add_edge([a, t], Weight(-10))?;
    assert!(matches!(
        graph.dijkstra().find_path(s, t),
        Err(Error::InvalidWeight)
    ));

    let mut graph = HyperMap::<(), f64>::undirected();
    let [s, t] = [(); 2].map(|_| graph.add_vertex().unwrap());
    graph.add_edge([s, t], Weight(f64::NAN))?;
    assert!(matches!(
        graph.dijkstra().find_path(s, t),
        Err(Error::InvalidWeight)
    ));
    Ok(())
}

/// #122 (P-9): a path cost above `E::MAX` is an error, not an overflow panic or a wrap.
#[test]
fn test_dijkstra_cost_overflow_is_an_error() -> Result<()> {
    use rshyper_algo::Error;
    use rshyper_core::Weight;
    let mut graph = HyperMap::<(), u8>::undirected();
    let [s, a, t] = [(); 3].map(|_| graph.add_vertex().unwrap());
    graph.add_edge([s, a], Weight(200))?;
    graph.add_edge([a, t], Weight(200))?;
    assert!(matches!(
        graph.dijkstra().find_path(s, t),
        Err(Error::CostOverflow)
    ));
    Ok(())
}

/// #122: directed graphs are rejected until ADR-DIR (#118) defines their semantics.
#[test]
fn test_dijkstra_rejects_directed_graphs() -> Result<()> {
    use rshyper_algo::Error;
    let mut graph = rshyper_hmap::DiHyperMap::<(), usize>::directed();
    let [s, t] = [(); 2].map(|_| graph.add_vertex().unwrap());
    graph.add_link([s, t])?;
    assert!(matches!(
        graph.dijkstra().find_path(s, t),
        Err(Error::UnsupportedGraphKind(_))
    ));
    Ok(())
}
