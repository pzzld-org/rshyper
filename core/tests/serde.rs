/*
    Appellation: serde <test>
    Contrib: @FL03
*/
//! #126: ids serialize as their raw value, inside every type that holds one
#![cfg(feature = "json")]
use rshyper_core::edge::HyperEdge;
use rshyper_core::idx::{EdgeId, IndexTracker, VertexId};
use rshyper_core::{Node, Undirected, Weight};
use serde_json::{from_str, to_string};

#[test]
fn ids_are_raw_values() {
    assert_eq!(to_string(&VertexId::<usize>::new(3)).unwrap(), "3");
    assert_eq!(to_string(&EdgeId::<u8>::new(4)).unwrap(), "4");
    assert_eq!(from_str::<VertexId<usize>>("3").unwrap(), VertexId::new(3));
    assert_eq!(from_str::<EdgeId<u8>>("4").unwrap(), EdgeId::new(4));
    // the old struct shape is no longer read
    assert!(from_str::<VertexId<usize>>(r#"{"value":3,"_type":null}"#).is_err());
}

#[test]
fn tracker_node_and_edge_hold_raw_ids() {
    let mut history = IndexTracker::<usize>::new();
    history.next_vertex().unwrap();
    history.next_vertex().unwrap();
    history.next_edge().unwrap();
    let json = r#"{"cursor":{"edge":1,"node":2},"edges":[0],"nodes":[0,1]}"#;
    assert_eq!(to_string(&history).unwrap(), json);
    assert_eq!(from_str::<IndexTracker<usize>>(json).unwrap(), history);

    let node = Node::new(VertexId::<usize>::new(3), 7usize);
    let json = r#"{"id":3,"weight":7}"#;
    assert_eq!(to_string(&node).unwrap(), json);
    assert_eq!(from_str::<Node<usize, usize>>(json).unwrap(), node);

    type Edge = HyperEdge<usize, [VertexId<usize>; 2], Undirected, usize>;
    let edge = Edge::from_parts(
        EdgeId::new(4),
        [VertexId::new(1), VertexId::new(2)],
        Weight(9),
    );
    let json = r#"{"link":{"id":4,"domain":[1,2],"_kind":null},"weight":9}"#;
    assert_eq!(to_string(&edge).unwrap(), json);
    assert_eq!(from_str::<Edge>(json).unwrap(), edge);
}
