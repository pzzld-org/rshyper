/*
    appellation: sequential <edges>
    authors: @FL03
*/
//! this module implements sequential iterator over the edges of a [`HyperMap`](crate::HyperMap)
//!
//! Each iterator walks the edge history and looks every id up in the edge map, so it yields
//! exactly the live edges recorded in the history, in creation order; history ids without a
//! live edge are skipped. Traversal costs `O(h)` expected lookups for a history of length `h`.
//! Ordered _mutable_ iteration is not provided: it cannot be expressed over a hash map without
//! aliasing or `unsafe`.
use crate::types::EdgeMap;
use core::hash::BuildHasher;
use core::slice;
use rshyper_core::GraphType;
use rshyper_core::edge::HashEdge;
use rshyper_core::idx::{EdgeId, HashIndex, RawIndex};

/// [`SeqEdgeIter`] is an iterator producing references to the edge entries w.r.t. the order
/// in-which they were inserted.
pub struct SeqEdgeIter<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) edges: &'a EdgeMap<E, K, Ix, S>,
}
/// [`SeqEdgeKeys`] is a sequential iterator over the _keys_ of the edges within the
/// [`HyperMap`](crate::HyperMap) yielding the [`EdgeId`] of each edge
/// w.r.t. the order in-which they were inserted.
pub struct SeqEdgeKeys<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) edges: &'a EdgeMap<E, K, Ix, S>,
}
/// [`SeqEdgeValues`] is an iterator producing references to the edges of a hypergraph w.r.t. the
/// order in-which they were inserted.
pub struct SeqEdgeValues<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: RawIndex,
{
    pub(crate) keys: slice::Iter<'a, EdgeId<Ix>>,
    pub(crate) edges: &'a EdgeMap<E, K, Ix, S>,
}

/*
 ************* Implementations *************
*/
impl<'a, E, K, Ix, S> Iterator for SeqEdgeIter<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = (&'a EdgeId<Ix>, &'a HashEdge<E, K, Ix, S>);

    fn next(&mut self) -> Option<Self::Item> {
        let edges = self.edges;
        self.keys.find_map(|id| edges.get_key_value(id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.keys.len()))
    }
}

impl<'a, E, K, Ix, S> Iterator for SeqEdgeKeys<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = &'a EdgeId<Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        let edges = self.edges;
        self.keys.find(|id| edges.contains_key(*id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.keys.len()))
    }
}

impl<'a, E, K, Ix, S> Iterator for SeqEdgeValues<'a, E, K, Ix, S>
where
    S: BuildHasher + 'a,
    E: 'a,
    K: GraphType,
    Ix: HashIndex,
{
    type Item = &'a HashEdge<E, K, Ix, S>;

    fn next(&mut self) -> Option<Self::Item> {
        let edges = self.edges;
        self.keys.find_map(|id| edges.get(id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.keys.len()))
    }
}
