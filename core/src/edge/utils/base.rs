/*
    appellation: base <module>
    authors: @FL03
*/
use crate::edge::{HyperEdge, Link};
use crate::{GraphType, RawDomain, RawIndex, VertexId};

/// returns a new [`HyperEdge`] from the given iterator of vertex ids
pub fn edge<I, T, S, K, Idx>(iter: I) -> HyperEdge<T, S, K, Idx>
where
    I: IntoIterator<Item = S::Key>,
    S: RawDomain<Key = VertexId<Idx>>,
    T: Default,
    K: GraphType,
    Idx: RawIndex,
    Link<S, K, Idx>: FromIterator<S::Key>,
{
    let rel = Link::from_iter(iter);
    HyperEdge::from_link(rel)
}
