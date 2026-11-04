/*
    Appellation: serde <test>
    Contrib: @FL03
*/
//! #126: JSON round trips, the validated load, and hasher-independent bytes.
#![cfg(feature = "json")]
mod common;

use common::RevBuild;
use core::hash::BuildHasher;
use rshyper_core::idx::VertexId;
use rshyper_core::{Result, Weight};
use rshyper_hmap::{DiHyperMap, UnHyperMap};
use serde_json::{Value, json};

type Un<S = hashbrown::DefaultHashBuilder> = UnHyperMap<usize, usize, usize, S>;
type Di = DiHyperMap<usize, usize>;

/// weights, parallel edges, a removed id followed by fresh allocations, and an isolated vertex
macro_rules! fixture {
    ($g:expr) => {{
        let mut g = $g;
        let v = (0..6)
            .map(|i| g.add_node(Weight(10 * i)))
            .collect::<Result<Vec<_>>>()?;
        g.add_edge([v[0], v[1]], Weight(1))?;
        g.add_edge([v[0], v[1]], Weight(2))?; // parallel
        g.add_edge([v[1], v[2], v[3]], Weight(3))?;
        g.add_edge([v[2], v[4]], Weight(4))?;
        g.remove_node(&v[2])?; // cascades edges 2 and 3
        let w = g.add_node(Weight(60))?; // id 6, never 2 again
        g.add_edge([v[3], w], Weight(5))?;
        let e = g.add_edge([v[0], v[4]], Weight(6))?;
        g.remove_edge(&e)?;
        g.add_link([v[4]])?; // a one-member edge; v[5] stays isolated
        g
    }};
}

/// everything the issue asks a round trip to preserve, plus the incidence index
macro_rules! assert_same {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert_eq!((a.order(), a.size()), (b.order(), b.size()));
        assert!(a.nodes() == b.nodes(), "nodes differ");
        assert!(a.edges() == b.edges(), "edges or domains differ");
        assert_eq!(a.history(), b.history());
        assert_eq!(a.position(), b.position());
        assert!(!b.is_incidence_stale());
        for v in a.vertices() {
            let ea = a.incident_edges(v)?.copied().collect::<Vec<_>>();
            let eb = b.incident_edges(v)?.copied().collect::<Vec<_>>();
            assert_eq!(ea, eb, "incidence of {v}");
        }
    }};
}

#[test]
fn undirected_graph_round_trips() -> Result<()> {
    let mut g = fixture!(Un::undirected());
    assert_eq!((g.order(), g.size()), (6, 4));
    let json = serde_json::to_string(&g).unwrap();
    let mut back: Un = serde_json::from_str(&json).unwrap();
    assert_same!(g, back);
    // and through a reader, whose keys are never borrowed
    let read: Un = serde_json::from_reader(json.as_bytes()).unwrap();
    assert_same!(g, read);
    // fresh ids continue exactly where the original's would
    let (v, w) = (back.add_vertex()?, g.add_vertex()?);
    assert_eq!(v, w);
    assert_eq!(back.add_link([v])?, g.add_link([w])?);
    Ok(())
}

#[test]
fn directed_graph_round_trips() -> Result<()> {
    let g = fixture!(Di::directed());
    let json = serde_json::to_string(&g).unwrap();
    let back: Di = serde_json::from_str(&json).unwrap();
    assert_same!(g, back);
    let read: Di = serde_json::from_reader(json.as_bytes()).unwrap();
    assert_same!(g, read);
    Ok(())
}

#[test]
fn empty_graph_round_trips() {
    let g = Un::<hashbrown::DefaultHashBuilder>::undirected();
    let back: Un = serde_json::from_str(&serde_json::to_string(&g).unwrap()).unwrap();
    assert_eq!((back.order(), back.size()), (0, 0));
}

#[test]
fn bytes_do_not_depend_on_the_hasher() -> Result<()> {
    let a = fixture!(Un::<hashbrown::DefaultHashBuilder>::undirected());
    let b = fixture!(Un::<RevBuild>::undirected());
    let (ja, jb) = (
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap(),
    );
    assert_eq!(ja, jb);
    // and the order is visible: ascending ids, sorted domains
    let v: Value = serde_json::from_str(&ja).unwrap();
    let keys = v["nodes"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(keys, ["0", "1", "3", "4", "5", "6"]);
    assert_eq!(v["edges"]["4"]["link"]["domain"], json!([3, 6]));
    // a larger graph, where hashbrown's raw order is far from sorted
    fn build<S: BuildHasher + Default>() -> Result<Un<S>> {
        let mut g = Un::<S>::undirected();
        let v = (0..200)
            .map(|_| g.add_vertex())
            .collect::<Result<Vec<_>>>()?;
        for i in 0..400 {
            g.add_link([v[i % 200], v[(i * 7 + 3) % 200], v[(i * 13 + 5) % 200]])?;
        }
        Ok(g)
    }
    assert_eq!(
        serde_json::to_string(&build::<hashbrown::DefaultHashBuilder>()?).unwrap(),
        serde_json::to_string(&build::<RevBuild>()?).unwrap()
    );
    Ok(())
}

/// load `doc` as an undirected graph and return the error message; a panic fails the test
fn reject(doc: Value) -> String {
    match serde_json::from_str::<Un>(&doc.to_string()) {
        Ok(g) => panic!("accepted a graph of order {}", g.order()),
        Err(e) => e.to_string(),
    }
}

fn valid() -> Value {
    let g = (|| -> Result<Un> { Ok(fixture!(Un::undirected())) })().unwrap();
    serde_json::to_value(&g).unwrap()
}

#[test]
fn the_fixture_document_is_valid() {
    let _: Un = serde_json::from_value(valid()).unwrap();
}

#[test]
fn rejects_another_graph_kind() {
    // directed JSON read as an undirected graph, and the reverse
    let di = (|| -> Result<Di> { Ok(fixture!(Di::directed())) })().unwrap();
    let err = reject(serde_json::to_value(&di).unwrap());
    assert!(err.contains("graph kind mismatch"), "{err}");
    let un = serde_json::to_string(&valid()).unwrap();
    assert!(serde_json::from_str::<Di>(&un).is_err());
    let mut doc = valid();
    doc.as_object_mut().unwrap().remove("kind");
    assert!(reject(doc).contains("missing field `kind`"));
    let mut doc = valid();
    doc["kind"] = Value::Null;
    assert!(reject(doc).contains("graph kind mismatch"));
}

#[test]
fn rejects_a_key_that_differs_from_the_stored_id() {
    let mut doc = valid();
    doc["nodes"]["0"]["id"] = json!(1);
    assert!(reject(doc).contains("node key 0 holds the node 1"));
    let mut doc = valid();
    doc["edges"]["0"]["link"]["id"] = json!(1);
    assert!(reject(doc).contains("edge key 0 holds the edge 1"));
}

#[test]
fn rejects_an_edge_naming_a_missing_vertex() {
    let mut doc = valid();
    doc["edges"]["0"]["link"]["domain"] = json!([0, 2]); // 2 was removed
    let err = reject(doc);
    assert!(err.contains("edge 0 names the vertex 2"), "{err}");
}

#[test]
fn rejects_an_empty_edge() {
    let mut doc = valid();
    doc["edges"]["0"]["link"]["domain"] = json!([]);
    assert!(reject(doc).contains("edge 0 is empty"));
}

#[test]
fn rejects_a_history_with_a_duplicate_or_a_missing_live_id() {
    // a duplicate
    let mut doc = valid();
    doc["history"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(json!(0));
    assert!(reject(doc).contains("vertex 0 is in the history twice"));
    let mut doc = valid();
    doc["history"]["edges"]
        .as_array_mut()
        .unwrap()
        .push(json!(0));
    assert!(reject(doc).contains("edge 0 is in the history twice"));
    // a live id that is not recorded
    let mut doc = valid();
    doc["history"]["nodes"]
        .as_array_mut()
        .unwrap()
        .retain(|v| v != 5);
    assert!(reject(doc).contains("vertex 5 is missing from the history"));
    let mut doc = valid();
    doc["history"]["edges"] = json!([]);
    assert!(reject(doc).contains("is missing from the history"));
}

#[test]
fn rejects_a_cursor_that_would_reissue_a_recorded_id() {
    let mut doc = valid();
    doc["history"]["cursor"]["node"] = json!(6);
    let err = reject(doc);
    assert!(
        err.contains("cursor 6 would issue the recorded vertex 6"),
        "{err}"
    );
    let mut doc = valid();
    doc["history"]["cursor"]["edge"] = json!(0);
    assert!(reject(doc).contains("would issue the recorded edge"));
    // the issue's witness: empty maps with a recorded history used to panic on the next add
    let doc = json!({
        "edges": {},
        "history": {"cursor": {"edge": 0, "node": 0}, "edges": [], "nodes": [0, 1, 2]},
        "kind": "undirected",
        "nodes": {},
    });
    let err = reject(doc);
    assert!(
        err.contains("cursor 0 would issue the recorded vertex 0"),
        "{err}"
    );
}

/// a recorded id with no live element is legal in memory (the sequential iterators skip it),
/// and the public `next_vertex_id` / `next_edge_id` leave one, so it must load
#[test]
fn a_reserved_id_round_trips() -> Result<()> {
    let mut g = fixture!(Un::undirected());
    let (v, e) = (g.next_vertex_id()?, g.next_edge_id()?);
    let json = serde_json::to_string(&g).unwrap();
    let mut back: Un = serde_json::from_str(&json).unwrap();
    assert_same!(g, back);
    assert!(back.history().nodes().contains(&v) && !back.contains_node(&v));
    assert!(back.history().edges().contains(&e) && !back.contains_edge(&e));
    // and it is still never issued again
    assert_eq!(back.add_vertex()?, g.add_vertex()?);
    assert!(back.add_vertex()? > v);
    // a dead id spliced into the history below the cursor loads the same way
    let mut doc = valid();
    doc["history"]["nodes"]
        .as_array_mut()
        .unwrap()
        .push(json!(2));
    let g: Un = serde_json::from_value(doc).unwrap();
    assert_eq!(g.history().nodes().last(), Some(&VertexId::new(2)));
    Ok(())
}

#[test]
fn rejects_unknown_and_duplicate_fields() {
    let mut doc = valid();
    doc["attrs"] = json!({"index": null, "kind": null}); // the pre-#126 field
    assert!(reject(doc).contains("unknown field `attrs`"));
    let json = valid().to_string();
    let twice = json.replacen("\"kind\":", "\"kind\":\"undirected\",\"kind\":", 1);
    assert!(
        serde_json::from_str::<Un>(&twice)
            .unwrap_err()
            .to_string()
            .contains("duplicate field `kind`")
    );
}

#[test]
fn ids_are_plain_numbers_on_the_wire() {
    let v = &valid()["history"];
    assert_eq!(v["cursor"], json!({"edge": 7, "node": 7}));
    assert_eq!(v["nodes"], json!([0, 1, 3, 4, 5, 6]));
    assert_eq!(v["edges"], json!([0, 1, 4, 6]));
    assert_eq!(
        serde_json::to_string(&VertexId::<usize>::new(3)).unwrap(),
        "3"
    );
}
