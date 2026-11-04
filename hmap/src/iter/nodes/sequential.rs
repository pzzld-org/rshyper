/*
    appellation: seq <module>
    authors: @FL03
*/
//! this module implements sequential iterator for the [`HyperMap`](crate::HyperMap)
//! which iterates over the nodes in the hypergraph in an ordered, sequential manner.
//!
//! Each iterator walks the node history and looks every id up in the node map, so it yields
//! exactly the live nodes recorded in the history, in creation order; history ids without a
//! live node are skipped. Traversal costs `O(h)` expected lookups for a history of length `h`.
//! Ordered _mutable_ iteration is not provided: it cannot be expressed over a hash map without
//! aliasing or `unsafe`.
use crate::types::NodeMap;
use core::hash::BuildHasher;
use core::slice;
use hashbrown::DefaultHashBuilder;
use rshyper_core::idx::{HashIndex, VertexId};
use rshyper_core::node::Node;

/// [`SeqNodeIter`] is an iterator producing references to the node entries w.r.t. the order
/// in-which they were inserted.
pub struct SeqNodeIter<'a, N, Ix, S = DefaultHashBuilder>
where
    N: 'a,
{
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) nodes: &'a NodeMap<N, Ix, S>,
}
/// [`SeqNodeKeys`] is a sequential iterator over the _keys_ of the nodes within the [`HyperMap`](crate::HyperMap)
/// yielding the [`VertexId`] of each node w.r.t. the order in-which they were inserted.
pub struct SeqNodeKeys<'a, N, Ix, S = DefaultHashBuilder> {
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) nodes: &'a NodeMap<N, Ix, S>,
}
/// [`SeqNodeValues`] is an iterator producing references to the nodes of a hypergraph w.r.t.
/// the order in-which they were inserted.
pub struct SeqNodeValues<'a, N, Ix, S = DefaultHashBuilder>
where
    N: 'a,
{
    pub(crate) keys: slice::Iter<'a, VertexId<Ix>>,
    pub(crate) nodes: &'a NodeMap<N, Ix, S>,
}

/*
 ************* Implementations *************
*/
impl<'a, N, Ix, S> Iterator for SeqNodeIter<'a, N, Ix, S>
where
    N: 'a,
    Ix: HashIndex,
    S: BuildHasher,
{
    type Item = (&'a VertexId<Ix>, &'a Node<N, Ix>);

    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes;
        self.keys.find_map(|id| nodes.get_key_value(id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.keys.len()))
    }
}

impl<'a, N, Ix, S> Iterator for SeqNodeKeys<'a, N, Ix, S>
where
    Ix: HashIndex,
    S: BuildHasher,
{
    type Item = &'a VertexId<Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes;
        self.keys.find(|id| nodes.contains_key(*id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.keys.len()))
    }
}

impl<'a, N, Ix, S> Iterator for SeqNodeValues<'a, N, Ix, S>
where
    N: 'a,
    Ix: HashIndex,
    S: BuildHasher,
{
    type Item = &'a Node<N, Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        let nodes = self.nodes;
        self.keys.find_map(|id| nodes.get(id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.keys.len()))
    }
}
