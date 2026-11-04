/*
    Appellation: serde_incidence <test>
    Contrib: @FL03
*/
//! the incidence index is not serialized, so deserializing a graph must rebuild it
#![cfg(feature = "json")]
use rshyper_core::idx::{EdgeId, VertexId};
use rshyper_hmap::HyperMap;
use serde::Deserialize;
use serde::de::value::{BorrowedStrDeserializer, MapDeserializer};
use serde_json::{Value, json};

// the graph's maps are keyed by struct ids, which JSON text cannot encode, so feed serde directly
type Map = MapDeserializer<'static, std::vec::IntoIter<(Value, Value)>, serde_json::Error>;

#[test]
fn deserialize_rebuilds_incidence() {
    let id = |v: usize| json!({ "value": v, "_type": null });
    let node = |v| (id(v), json!({ "id": id(v), "weight": 0 }));
    let link =
        json!({ "link": { "id": id(0), "domain": [id(0), id(1)], "_kind": null }, "weight": 0 });
    let map = |items: Vec<(Value, Value)>| Map::new(items.into_iter());
    let fields = [
        ("attrs", map(vec![])),
        ("edges", map(vec![(id(0), link)])),
        ("history", map(vec![])),
        ("nodes", map(vec![node(0), node(1)])),
    ]
    .into_iter()
    .map(|(k, v)| (BorrowedStrDeserializer::new(k), v));
    let g = HyperMap::<usize, usize>::deserialize(MapDeserializer::new(fields)).unwrap();
    assert_eq!(g.size(), 1);
    assert_eq!(g.get_node_degree(&VertexId::new(0)), 1);
    let hits = g
        .find_edges_with_node(&VertexId::new(1))
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(hits, [EdgeId::new(0)]);
}
