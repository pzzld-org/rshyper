/*
    appellation: hyper_edge <module>
    authors: @FL03
*/
use crate::idx::{EdgeId, VertexId};
use crate::traits::RawDomain;

/// Here, a [`Link`] essentially defines the layout of an edge within a hypergraph. The
/// implementation is generic over the type of domain it contains, which can be a set of
/// vertices or any other structure that implements the [`RawDomain`] trait.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Link<S, K, Ix = usize>
where
    S: RawDomain<Key = VertexId<Ix>>,
{
    pub(crate) id: EdgeId<Ix>,
    pub(crate) domain: S,
    pub(crate) _kind: core::marker::PhantomData<K>,
}
