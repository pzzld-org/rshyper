/*
    Appellation: hgraph <module>
    Contrib: @FL03
*/
use crate::edge::{BinaryEdge, RawEdge, RawSurface};
use crate::error::Result;
use crate::idx::{EdgeId, VertexId};
use crate::node::RawNode;
use crate::{GraphProps, Weight};

/// [`RawHyperGraph`] is a trait that defines the basic operations for a hypergraph data
/// structure.
pub trait RawHyperGraph<A>
where
    A: GraphProps,
{
    type Edge<E>: RawSurface<E, Index = A::Ix, Kind = A::Kind>;
    type Node<N>: RawNode<N, Key = A::Ix>;
}
/// The [`HyperGraph`] trait directly extends the [`RawHyperGraph`] trait to provide additional
/// utilities and constructors for implementors while establishing a more robust interface for
/// hypergraphs. This trait is designed to abstract the basic behaviour of hypergraphs,
/// enabling the generalization of implements algorithms and operators.
pub trait HyperGraph<N, E, A>: RawHyperGraph<A>
where
    A: GraphProps,
{
    /// given an iterable of vertex indices, add an edge to the graph and return its index
    fn add_edge<I>(&mut self, iter: I) -> Result<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>,
        E: Default,
    {
        self.add_surface(iter, Default::default())
    }
    /// given an iterable of vertex indices and a weight, add an edge to the graph and return its index
    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> Result<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>;
    /// add a new node to the graph with the given weight and return its index
    fn add_node(&mut self, weight: Weight<N>) -> Result<VertexId<A::Ix>>;
    /// add a new default node to the graph and return its index
    fn add_vertex(&mut self) -> Result<VertexId<A::Ix>>
    where
        N: Default,
    {
        self.add_node(Default::default())
    }
    /// returns the vertices of the edge with the given index
    fn get_edge_domain(&self, index: &EdgeId<A::Ix>) -> Result<&<Self::Edge<E> as RawEdge>::Store>;
    /// returns a mutable reference to the vertices of the edge with the given index
    fn get_edge_domain_mut(
        &mut self,
        index: &EdgeId<A::Ix>,
    ) -> Result<&mut <Self::Edge<E> as RawEdge>::Store>;
    /// returns a reference to the weight of the edge with the given index
    fn get_edge_weight(&self, index: &EdgeId<A::Ix>) -> Result<&Weight<E>> {
        self.get_edge(index).map(|edge| edge.weight())
    }
    /// returns a mutable reference to the weight of the edge with the given index
    fn get_edge_weight_mut(&mut self, index: &EdgeId<A::Ix>) -> Result<&mut Weight<E>> {
        self.get_edge_mut(index).map(|edge| edge.weight_mut())
    }
    /// returns an immutable reference to the edge with the given index
    fn get_edge(&self, index: &EdgeId<A::Ix>) -> Result<&Self::Edge<E>>;
    /// returns a mutable reference to the edge with the given index
    fn get_edge_mut(&mut self, index: &EdgeId<A::Ix>) -> Result<&mut Self::Edge<E>>;
    /// returns a reference to the node with the given index
    fn get_node(&self, index: &VertexId<A::Ix>) -> Result<&Self::Node<N>>;
    /// returns a mutable reference to the node with the given index
    fn get_node_mut(&mut self, index: &VertexId<A::Ix>) -> Result<&mut Self::Node<N>>;
    /// returns the weight of the node with the given index
    fn get_node_weight(&self, index: &VertexId<A::Ix>) -> Result<&Weight<N>>;
    /// returns a mutable reference to the weight of the node with the given index
    fn get_node_weight_mut(&mut self, index: &VertexId<A::Ix>) -> Result<&mut Weight<N>>;
    /// returns true if the graph contains the edge with the given index
    fn contains_edge(&self, index: &EdgeId<A::Ix>) -> bool;
    /// returns true if the graph contains the node with the given index
    fn contains_node(&self, index: &VertexId<A::Ix>) -> bool;
    /// returns an iterator over all edges that contain the given node
    fn find_edges_with_node(&self, index: &VertexId<A::Ix>)
    -> impl Iterator<Item = &EdgeId<A::Ix>>;
}

/// The [`HyperGraphIterNode`] trait extends the [`HyperGraph`] trait to provide iterators over
/// the nodes in the hypergraph.
pub trait HyperGraphIterNode<N, E, A>: HyperGraph<N, E, A>
where
    A: GraphProps,
{
    type Nodes<'a>: Iterator<Item = (&'a VertexId<A::Ix>, &'a Self::Node<N>)>
    where
        Self: 'a,
        <Self as RawHyperGraph<A>>::Node<N>: 'a;
    type Verts<'a>: Iterator<Item = &'a VertexId<A::Ix>>
    where
        Self: 'a;
    /// returns an iterator over the nodes of the graph
    fn iter_nodes(&self) -> Self::Nodes<'_>;
    /// returns an iterators over the indices of the nodes within the graph
    fn vertices(&self) -> Self::Verts<'_>;
}
/// The [`HyperGraphIterEdge`] trait extends the [`HyperGraph`] trait to provide iterators over
/// the edges in the hypergraph.
pub trait HyperGraphIterEdge<N, E, A>: HyperGraph<N, E, A>
where
    A: GraphProps,
{
    type Surfaces<'a>: Iterator<Item = (&'a EdgeId<A::Ix>, &'a Self::Edge<E>)>
    where
        Self: 'a,
        <Self as RawHyperGraph<A>>::Edge<E>: 'a;
    type Edges<'a>: Iterator<Item = &'a EdgeId<A::Ix>>
    where
        Self: 'a;
    /// returns an iterator over the edges of the graph
    fn iter_surfaces(&self) -> Self::Surfaces<'_>;
    /// returns an iterator over the indices of the edges within the graph
    fn edges(&self) -> Self::Edges<'_>;
}
/// The [`HyperGraphIter`] trait combines the [`HyperGraphIterNode`] and [`HyperGraphIterEdge`]
/// traits to provide a unified interface for iterating over both nodes
pub trait HyperGraphIter<N, E, A>:
    HyperGraphIterNode<N, E, A> + HyperGraphIterEdge<N, E, A>
where
    A: GraphProps,
{
    private!();
}
/// The [`StdGraph`] is used to denotes instances in-which the hypergraph contains binary edges
/// meaning that each edge is composed of exactly two vertices.
///
/// **note:** the trait is automatically implemented for all hypergraphs that leverage a
/// so-called [`BinaryEdge`] representation
pub trait StdGraph<N, E, A>: RawHyperGraph<A>
where
    A: GraphProps,
    Self::Edge<E>: BinaryEdge,
{
    private!();
}

/*
 ************* Implementations *************
*/

impl<N, E, A, H> StdGraph<N, E, A> for H
where
    A: GraphProps,
    N: Default,
    E: Default,
    H: HyperGraph<N, E, A>,
    H::Edge<E>: BinaryEdge,
{
    seal!();
}

impl<H, N, E, A> HyperGraphIter<N, E, A> for H
where
    A: GraphProps,
    H: HyperGraphIterNode<N, E, A> + HyperGraphIterEdge<N, E, A>,
{
    seal!();
}
