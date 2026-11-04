/*
    Appellation: algo <test>
    Contrib: @FL03
*/
//! #121: the facade's `algo` feature must enable the `HyperMap` search methods.
use rshyper::HyperMap;

type Result<T = ()> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn facade_algo_enables_hypermap_searches() -> Result {
    let mut g = HyperMap::<(), usize>::undirected();
    let a = g.add_vertex()?;
    let b = g.add_vertex()?;
    let c = g.add_vertex()?;
    g.add_link([a, b])?;
    g.add_link([b, c])?;

    assert_eq!(g.bft().search(a)?.len(), 3);
    assert_eq!(g.dft().search(a)?.len(), 3);
    assert_eq!(g.dijkstra().find_path(a, c)?, vec![a, b, c]);
    assert_eq!(g.astar(|_, _| 0.0).find_path(a, c)?, vec![a, b, c]);
    Ok(())
}
