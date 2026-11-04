/*
    appellation: impl_serde <module>
    authors: @FL03
*/
use crate::HyperMap;
use crate::types::incidence::Incidence;
use core::hash::{BuildHasher, Hash};
use rshyper::GraphProps;
use serde::de::{Deserialize, DeserializeOwned, MapAccess, Visitor};
use serde::ser::Serialize;

const FIELDS: &[&str] = &["attrs", "edges", "history", "nodes"];

impl<'a, N, E, A, S> Deserialize<'a> for HyperMap<N, E, A, S>
where
    A: GraphProps + DeserializeOwned,
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

impl<N, E, A, S> Serialize for HyperMap<N, E, A, S>
where
    A: GraphProps + Serialize,
    E: Serialize,
    N: Serialize,
    S: BuildHasher + Default,
    A::Ix: Default + Eq + Hash + Serialize,
    A::Kind: Serialize,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("HashGraph", 4)?;
        state.serialize_field("attrs", &self.attrs())?;
        state.serialize_field("edges", self.edges())?;
        state.serialize_field("history", self.history())?;
        state.serialize_field("nodes", self.nodes())?;
        state.end()
    }
}

struct HashGraphVisitor<N, E, A, S> {
    _marker: core::marker::PhantomData<(N, E, A, S)>,
}

impl<'de, N, E, A, S> Visitor<'de> for HashGraphVisitor<N, E, A, S>
where
    A: GraphProps + DeserializeOwned,
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
        let mut position = None;
        let mut attrs = None;

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "attrs" => {
                    if attrs.is_some() {
                        return Err(serde::de::Error::duplicate_field("attrs"));
                    }
                    attrs = Some(map.next_value()?);
                }
                "edges" => {
                    if edges.is_some() {
                        return Err(serde::de::Error::duplicate_field("edges"));
                    }
                    edges = Some(map.next_value()?);
                }
                "history" => {
                    if position.is_some() {
                        return Err(serde::de::Error::duplicate_field("position"));
                    }
                    position = Some(map.next_value()?);
                }
                "nodes" => {
                    if nodes.is_some() {
                        return Err(serde::de::Error::duplicate_field("nodes"));
                    }
                    nodes = Some(map.next_value()?);
                }
                _ => return Err(serde::de::Error::unknown_field(key, FIELDS)),
            }
        }

        let attrs = attrs.ok_or_else(|| serde::de::Error::missing_field("attrs"))?;
        let edges = edges.ok_or_else(|| serde::de::Error::missing_field("edges"))?;
        let history = position.ok_or_else(|| serde::de::Error::missing_field("history"))?;
        let nodes = nodes.ok_or_else(|| serde::de::Error::missing_field("nodes"))?;

        let mut graph = HyperMap {
            nodes,
            edges,
            history,
            attrs,
            incidence: Incidence::default(),
        };
        graph.reindex();
        Ok(graph)
    }
}
