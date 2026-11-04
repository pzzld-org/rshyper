/*
    appellation: macros <module>
    authors: @FL03
*/
use rshyper::HyperMap;

#[test]
fn test_hypergraph_on_eg() -> rshyper::Result<()> {
    // initialize a new, undirected hypergraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // use the `hypergraph!` macro to define nodes and edges
    rshyper::hypergraph! {
        graph {
            nodes: {
                let v0;
                let v1 = 10;
                let v2 = 20;
                let v3 = 30;

            };
            edges: {
                let e1: [v0, v1, v2];
                let e2: [v1, v2, v3];
            };
        }
    }
    // verify the nodes
    assert_eq! { graph.get_node_weight(&v0)?, &<usize>::default() }
    assert_eq! { graph.get_node_weight(&v1)?, &10 }
    assert_eq! { graph.get_node_weight(&v2)?, &20 }
    assert_eq! { graph.get_node_weight(&v3)?, &30 }
    // verify the edges
    assert_eq! { graph.get_edge_order(&e1)?, 3 }
    assert_eq! { graph.get_edge_order(&e2)?, 3 }
    // finish
    Ok(())
}

#[test]
fn test_hypernode_on_eg() -> rshyper::Result<()> {
    // initialize a new, undirected hypergraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // use the `hypernode!` macro to define nodes
    rshyper::hypernode! {
        graph {
            let v0;
            let v1 = 10;
            let v2 = 20;
            let v3 = 30;
        }
    }
    // verify the graph composition
    assert_eq!(graph.size(), 0); // no edges defined
    assert_eq!(graph.order(), 4); // 4 vertices
    // verify the weights of each node
    for (id, exp) in [(v0, 0), (v1, 10), (v2, 20), (v3, 30)] {
        assert_eq!(graph.get_node_weight(&id)?, &exp);
    }
    // finish
    Ok(())
}

#[test]
fn test_hyperedge_on_hg() -> rshyper::Result<()> {
    // initialize a new, undirected hypergraph
    let mut graph = HyperMap::<usize, usize>::undirected();
    // manually add some vertices
    let v0 = graph.add_vertex()?;
    let v1 = graph.add_vertex()?;
    let v2 = graph.add_vertex()?;
    let v3 = graph.add_vertex()?;
    // use the `hyperedge!` macro to define edges
    rshyper::hyperedge! {
        graph {
            let e1: [v0, v1, v2];
            let e2: [v1, v2, v3];
        }
    }
    // verify the graph composition
    assert_eq!(graph.size(), 2); // 2 edges
    assert_eq!(graph.get_edge_order(&e1)?, graph.get_edge_order(&e2)?); // both edges have 3 vertices
    // finish
    Ok(())
}

/// #125: each documented `# Panics` case of the macros
mod panics {
    use rshyper::HyperMap;
    use rshyper::idx::VertexId;

    #[test]
    #[should_panic(expected = "Failed to insert edge")]
    fn hyperedge_with_an_unknown_vertex() {
        let mut graph = HyperMap::<usize, usize>::undirected();
        let v0 = graph.add_vertex().unwrap();
        let ghost = VertexId::new(99);
        rshyper::hyperedge! { graph { let _e: [v0, ghost]; } }
    }

    #[test]
    #[should_panic(expected = "Failed to insert edge")]
    fn hyperedge_with_no_members() {
        let mut graph = HyperMap::<usize, usize>::undirected();
        rshyper::hyperedge! { graph { let _e: [] = 1; } }
    }

    #[test]
    #[should_panic(expected = "Failed to insert edge")]
    fn hypergraph_with_an_unknown_vertex() {
        let ghost = VertexId::new(99);
        rshyper::hypergraph! {
            let mut graph: HyperMap<usize, usize> {
                nodes: { let v0; };
                edges: { let _e: [v0, ghost]; };
            }
        }
    }

    #[test]
    #[should_panic(expected = "Failed to insert edge")]
    fn hyperedge_on_an_exhausted_index() {
        let mut graph = rshyper::UnHyperMap::<usize, usize, u8>::undirected();
        let v0 = graph.add_vertex().unwrap();
        // fill the u8 edge-id space; bounded, so a missing error fails instead of hanging
        let n = (0..=256)
            .take_while(|_| graph.add_link([v0]).is_ok())
            .count();
        assert!(n <= 256, "{n}");
        rshyper::hyperedge! { graph { let _e: [v0]; } }
    }

    #[test]
    #[should_panic(expected = "Failed to insert node")]
    fn hypernode_on_an_exhausted_index() {
        let mut graph = rshyper::UnHyperMap::<usize, usize, u8>::undirected();
        // fill the u8 id space; bounded, so a missing error fails instead of hanging
        let n = (0..=256).take_while(|_| graph.add_vertex().is_ok()).count();
        assert!(n <= 256, "{n}");
        rshyper::hypernode! { graph { let _v; } }
    }
}
