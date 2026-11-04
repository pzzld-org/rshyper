/*
    appellation: impl_ops <module>
    authors: @FL03
*/
use crate::graph::HyperMap;

use core::hash::BuildHasher;
use core::ops::{Add, Index, IndexMut};
use rshyper_core::edge::HashEdge;
use rshyper_core::idx::{EdgeId, HashIndex, HyperIndex, VertexId};
use rshyper_core::{Combine, GraphProps, Node, Result};

impl<N, E, A, S, Ix> Combine<EdgeId<Ix>, EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher + Default,
    Ix: HyperIndex,
    for<'a> &'a E: Add<Output = E>,
{
    type Output = EdgeId<Ix>;

    fn combine(&mut self, src: EdgeId<Ix>, tgt: EdgeId<Ix>) -> Result<Self::Output> {
        self.merge_edges(&src, &tgt)
    }
}

impl<'a, N, E, A, S, Ix> Combine<&'a EdgeId<Ix>, &'a EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher + Default,
    Ix: HyperIndex,
    for<'b> &'b E: Add<Output = E>,
{
    type Output = EdgeId<Ix>;

    fn combine(&mut self, src: &'a EdgeId<Ix>, tgt: &'a EdgeId<Ix>) -> Result<Self::Output> {
        self.merge_edges(src, tgt)
    }
}

impl<N, E, A, S, Ix> Index<&EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    type Output = HashEdge<E, A::Kind, Ix, S>;

    /// # Panics
    ///
    /// Panics if the graph has no edge with this id, as indexing does in `std`; use
    /// [`get_edge`](HyperMap::get_edge) to get an error instead.
    fn index(&self, index: &EdgeId<Ix>) -> &Self::Output {
        self.get_edge(index).expect("Edge not found")
    }
}

impl<N, E, A, S, Ix> IndexMut<&EdgeId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    /// # Panics
    ///
    /// Panics if the graph has no edge with this id, as indexing does in `std`; use
    /// [`get_edge_mut`](HyperMap::get_edge_mut) to get an error instead.
    fn index_mut(&mut self, index: &EdgeId<Ix>) -> &mut Self::Output {
        self.get_edge_mut(index).expect("Edge not found")
    }
}

impl<N, E, A, S, Ix> Index<&VertexId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    type Output = Node<N, Ix>;

    /// # Panics
    ///
    /// Panics if the graph has no node with this id, as indexing does in `std`; use
    /// [`get_node`](HyperMap::get_node) to get an error instead.
    fn index(&self, index: &VertexId<Ix>) -> &Self::Output {
        self.get_node(index).expect("Node not found")
    }
}

impl<N, E, A, S, Ix> IndexMut<&VertexId<Ix>> for HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
{
    /// # Panics
    ///
    /// Panics if the graph has no node with this id, as indexing does in `std`; use
    /// [`get_node_mut`](HyperMap::get_node_mut) to get an error instead.
    fn index_mut(&mut self, index: &VertexId<Ix>) -> &mut Self::Output {
        self.get_node_mut(index).expect("Node not found")
    }
}
