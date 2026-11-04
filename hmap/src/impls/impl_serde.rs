/*
    appellation: impl_serde <module>
    authors: @FL03
*/
//! The wire format (#126): ids are raw values, so the node and edge maps are plain JSON maps;
//! both maps are written in ascending id order and each edge's domain sorted, so equal graphs
//! give equal bytes under any hasher; the graph kind replaces the old `attrs` field (which
//! wrote only nulls) and is checked on load; and a document is validated before it is exposed
//! as a graph.
use crate::HyperMap;
use crate::types::incidence::Incidence;
use alloc::vec::Vec;
use core::hash::{BuildHasher, Hash};
use hashbrown::HashSet;
use rshyper::edge::HashEdge;
use rshyper::idx::{EdgeId, RawIndex, VertexId};
use rshyper::{GraphProps, GraphType, Mode, Weight};
use serde::de::{Deserialize, DeserializeOwned, Error, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

const FIELDS: &[&str] = &["edges", "history", "kind", "nodes"];

/// the kind a graph with attributes `A` is written with: `None` for a `Mode`-kinded graph,
/// which fixes no direction at the type level
fn kind_of<A: GraphProps>(attrs: &A) -> Option<Mode> {
    if attrs.is_directed() {
        Some(Mode::Directed)
    } else if attrs.is_undirected() {
        Some(Mode::Undirected)
    } else {
        None
    }
}

/// a map written in ascending key order
struct Ordered<K, V>(Vec<(K, V)>);

impl<K: Serialize, V: Serialize> Serialize for Ordered<K, V> {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
        serializer.collect_map(self.0.iter().map(|(k, v)| (k, v)))
    }
}

/// a [`HashEdge`] with its domain sorted, in the shape its derived `Deserialize` reads
#[derive(serde_derive::Serialize)]
#[serde(rename = "HyperEdge")]
struct SortedEdge<'a, E, Ix> {
    link: SortedLink<'a, Ix>,
    weight: &'a Weight<E>,
}

#[derive(serde_derive::Serialize)]
#[serde(rename = "Link")]
struct SortedLink<'a, Ix> {
    id: &'a EdgeId<Ix>,
    domain: Vec<&'a VertexId<Ix>>,
    _kind: core::marker::PhantomData<()>,
}

impl<'a, E, Ix> SortedEdge<'a, E, Ix>
where
    Ix: RawIndex + Eq + Hash + Ord,
{
    fn new<K: GraphType, S: BuildHasher>(edge: &'a HashEdge<E, K, Ix, S>) -> Self {
        let mut domain = edge.domain().iter().collect::<Vec<_>>();
        domain.sort_unstable();
        SortedEdge {
            link: SortedLink {
                id: edge.id(),
                domain,
                _kind: core::marker::PhantomData,
            },
            weight: edge.weight(),
        }
    }
}

impl<N, E, A, S> Serialize for HyperMap<N, E, A, S>
where
    A: GraphProps,
    E: Serialize,
    N: Serialize,
    S: BuildHasher + Default,
    A::Ix: Default + Eq + Hash + Ord + Serialize,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        let mut nodes = self.nodes().iter().collect::<Vec<_>>();
        nodes.sort_unstable_by(|a, b| a.0.cmp(b.0));
        let mut edges = self
            .edges()
            .iter()
            .map(|(id, edge)| (id, SortedEdge::new(edge)))
            .collect::<Vec<_>>();
        edges.sort_unstable_by(|a, b| a.0.cmp(b.0));
        let mut state = serializer.serialize_struct("HashGraph", 4)?;
        state.serialize_field("edges", &Ordered(edges))?;
        state.serialize_field("history", self.history())?;
        state.serialize_field("kind", &kind_of(&self.attrs()))?;
        state.serialize_field("nodes", &Ordered(nodes))?;
        state.end()
    }
}

impl<'a, N, E, A, S> Deserialize<'a> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    E: DeserializeOwned,
    N: DeserializeOwned,
    S: BuildHasher + Default,
    A::Ix: Clone + Default + Eq + Hash + Ord + DeserializeOwned,
    A::Kind: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        deserializer.deserialize_struct(
            "HashGraph",
            FIELDS,
            HashGraphVisitor {
                _marker: core::marker::PhantomData,
            },
        )
    }
}

/// the field names, read owned or borrowed, so `from_reader` and escaped keys work
#[derive(serde_derive::Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    Edges,
    History,
    Kind,
    Nodes,
}

struct HashGraphVisitor<N, E, A, S> {
    _marker: core::marker::PhantomData<(N, E, A, S)>,
}

impl<'de, N, E, A, S> Visitor<'de> for HashGraphVisitor<N, E, A, S>
where
    A: GraphProps,
    E: DeserializeOwned,
    N: DeserializeOwned,
    S: BuildHasher + Default,
    A::Ix: Clone + Default + Eq + Hash + Ord + DeserializeOwned,
    A::Kind: DeserializeOwned,
{
    type Value = HyperMap<N, E, A, S>;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a HashGraph")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut nodes = None;
        let mut edges = None;
        let mut history = None;
        let mut kind: Option<Option<Mode>> = None;

        while let Some(key) = map.next_key::<Field>()? {
            match key {
                Field::Edges if edges.is_some() => return Err(M::Error::duplicate_field("edges")),
                Field::Edges => edges = Some(map.next_value()?),
                Field::History if history.is_some() => {
                    return Err(M::Error::duplicate_field("history"));
                }
                Field::History => history = Some(map.next_value()?),
                Field::Kind if kind.is_some() => return Err(M::Error::duplicate_field("kind")),
                Field::Kind => kind = Some(map.next_value()?),
                Field::Nodes if nodes.is_some() => return Err(M::Error::duplicate_field("nodes")),
                Field::Nodes => nodes = Some(map.next_value()?),
            }
        }

        let mut graph = HyperMap {
            attrs: A::new(),
            edges: edges.ok_or_else(|| M::Error::missing_field("edges"))?,
            history: history.ok_or_else(|| M::Error::missing_field("history"))?,
            nodes: nodes.ok_or_else(|| M::Error::missing_field("nodes"))?,
            incidence: Incidence::default(),
        };
        validate(&graph, kind.ok_or_else(|| M::Error::missing_field("kind"))?)?;
        graph.reindex();
        Ok(graph)
    }
}

/// reject a document the in-memory graph could not hold: a graph of another kind, a map key
/// that differs from the stored id, an empty edge or one naming a missing vertex, and a history
/// with a duplicate, an id at or above the cursor, or a live id it does not record. A recorded
/// id with no live element is accepted, as in memory: `next_vertex_id` and `next_edge_id`
/// leave one, and the sequential iterators skip it.
fn validate<N, E, A, S, Err>(g: &HyperMap<N, E, A, S>, kind: Option<Mode>) -> Result<(), Err>
where
    A: GraphProps,
    S: BuildHasher + Default,
    A::Ix: Eq + Hash + Ord,
    Err: Error,
{
    let target = kind_of(&g.attrs());
    if kind != target {
        return Err(Err::custom(format_args!(
            "graph kind mismatch: the document is {kind:?}, the target type is {target:?}"
        )));
    }
    for (key, node) in g.nodes() {
        if key != node.id() {
            return Err(Err::custom(format_args!(
                "node key {key} holds the node {}",
                node.id()
            )));
        }
    }
    for (key, edge) in g.edges() {
        if key != edge.id() {
            return Err(Err::custom(format_args!(
                "edge key {key} holds the edge {}",
                edge.id()
            )));
        }
        if edge.domain().is_empty() {
            return Err(Err::custom(format_args!("edge {key} is empty")));
        }
        if let Some(v) = edge.domain().iter().find(|v| !g.nodes().contains_key(*v)) {
            return Err(Err::custom(format_args!(
                "edge {key} names the vertex {v}, which is not in the graph"
            )));
        }
    }
    let cursor = g.history().cursor();
    check_history::<_, S, Err>(
        g.history().nodes(),
        cursor.node(),
        g.nodes().keys(),
        "vertex",
    )?;
    check_history::<_, S, Err>(g.history().edges(), cursor.edge(), g.edges().keys(), "edge")
}

/// `history` must name each id once, only ids below the cursor, and every `live` id; so every
/// live id is below the cursor too, and no allocation can hand one out again
fn check_history<'a, T, S, Err>(
    history: &'a [T],
    cursor: &T,
    mut live: impl Iterator<Item = &'a T>,
    what: &str,
) -> Result<(), Err>
where
    T: 'a + core::fmt::Display + Eq + Hash + Ord,
    S: BuildHasher + Default,
    Err: Error,
{
    let mut seen = HashSet::with_capacity_and_hasher(history.len(), S::default());
    for id in history {
        if !seen.insert(id) {
            return Err(Err::custom(format_args!(
                "the {what} {id} is in the history twice"
            )));
        }
        if id >= cursor {
            return Err(Err::custom(format_args!(
                "the cursor {cursor} would issue the recorded {what} {id} again"
            )));
        }
    }
    match live.find(|id| !seen.contains(id)) {
        Some(id) => Err(Err::custom(format_args!(
            "the {what} {id} is missing from the history"
        ))),
        None => Ok(()),
    }
}
