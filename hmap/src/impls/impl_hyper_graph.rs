/*
    appellation: impl_hyper_graph <module>
    authors: @FL03
*/
use crate::{HyperMap, iter};
use core::hash::BuildHasher;
use rshyper_core::edge::HashEdge;
use rshyper_core::idx::{EdgeId, HyperIndex, VertexId};
use rshyper_core::traits::{HyperGraph, HyperGraphIterEdge, HyperGraphIterNode, RawHyperGraph};
use rshyper_core::{GraphProps, GraphType, Node, Result, VertexSet, Weight};

impl<N, E, A, S> RawHyperGraph<A> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    S: BuildHasher,
{
    type Node<_N> = Node<_N, A::Ix>;
    type Edge<_E> = HashEdge<_E, A::Kind, A::Ix, S>;
}

impl<N, E, A, S, K, Ix> HyperGraph<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphProps<Kind = K, Ix = Ix>,
    S: BuildHasher + Default,
    K: GraphType,
    Ix: HyperIndex,
{
    fn add_node(&mut self, weight: Weight<N>) -> Result<VertexId<A::Ix>> {
        self.add_node(weight)
    }

    fn add_surface<I>(&mut self, iter: I, weight: Weight<E>) -> Result<EdgeId<A::Ix>>
    where
        I: IntoIterator<Item = VertexId<A::Ix>>,
    {
        self.add_edge(iter, weight)
    }

    fn get_edge_domain(&self, index: &EdgeId<A::Ix>) -> Result<&VertexSet<A::Ix, S>> {
        self.get_domain(index)
    }

    fn get_edge_domain_mut(&mut self, index: &EdgeId<A::Ix>) -> Result<&mut VertexSet<A::Ix, S>> {
        self.get_domain_mut(index)
    }

    fn get_edge(&self, index: &EdgeId<A::Ix>) -> Result<&HashEdge<E, A::Kind, A::Ix, S>> {
        self.get_edge(index)
    }

    fn get_edge_mut(
        &mut self,
        index: &EdgeId<A::Ix>,
    ) -> Result<&mut HashEdge<E, A::Kind, A::Ix, S>> {
        self.get_edge_mut(index)
    }

    fn get_edge_weight(&self, index: &EdgeId<A::Ix>) -> Result<&Weight<E>> {
        self.get_edge_weight(index)
    }

    fn get_edge_weight_mut(&mut self, index: &EdgeId<A::Ix>) -> Result<&mut Weight<E>> {
        self.get_edge_weight_mut(index)
    }

    fn get_node(&self, index: &VertexId<A::Ix>) -> Result<&Node<N, A::Ix>> {
        self.get_node(index)
    }

    fn get_node_mut(&mut self, index: &VertexId<A::Ix>) -> Result<&mut Node<N, A::Ix>> {
        self.get_node_mut(index)
    }

    fn get_node_weight(&self, index: &VertexId<<A as GraphProps>::Ix>) -> Result<&Weight<N>> {
        self.get_node_weight(index)
    }

    fn get_node_weight_mut(&mut self, index: &VertexId<A::Ix>) -> Result<&mut Weight<N>> {
        self.get_node_weight_mut(index)
    }

    fn contains_edge(&self, index: &EdgeId<A::Ix>) -> bool {
        self.contains_edge(index)
    }

    fn contains_node(&self, index: &VertexId<A::Ix>) -> bool {
        self.contains_node(index)
    }

    fn find_edges_with_node(
        &self,
        index: &VertexId<A::Ix>,
    ) -> impl Iterator<Item = &EdgeId<A::Ix>> {
        self.find_edges_with_node(index)
    }
}

impl<N, E, A, S> HyperGraphIterNode<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    S: BuildHasher + Default,
    A::Ix: HyperIndex,
{
    type Nodes<'a>
        = iter::NodeIter<'a, N, A::Ix>
    where
        Self: 'a,
        Self::Node<N>: 'a;
    type Verts<'a>
        = iter::NodeKeys<'a, N, A::Ix>
    where
        Self: 'a;

    fn iter_nodes(&self) -> Self::Nodes<'_> {
        self.iter_nodes()
    }

    fn vertices(&self) -> Self::Verts<'_> {
        self.vertices()
    }
}

impl<N, E, A, S> HyperGraphIterEdge<N, E, A> for HyperMap<N, E, A, S>
where
    A: GraphProps,
    S: BuildHasher + Default,
    A::Ix: HyperIndex,
{
    type Surfaces<'a>
        = iter::EdgeIter<'a, E, A::Kind, A::Ix, S>
    where
        Self: 'a,
        Self::Edge<E>: 'a;

    type Edges<'a>
        = iter::EdgeKeys<'a, E, A::Kind, A::Ix, S>
    where
        Self: 'a,
        Self::Edge<E>: 'a;

    fn iter_surfaces(&self) -> Self::Surfaces<'_> {
        self.iter_edges()
    }

    fn edges(&self) -> Self::Edges<'_> {
        self.iter_edge_keys()
    }
}
