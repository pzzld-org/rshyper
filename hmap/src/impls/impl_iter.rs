/*
    appellation: impl_iter <module>
    authors: @FL03
*/
use crate::HyperMap;
use crate::iter::*;

use core::hash::BuildHasher;
use rshyper::{GraphProps, GraphType, HashIndex};

/// implements various iterators for the [`HyperMap`]
impl<N, E, A, S, K, Idx> HyperMap<N, E, A, S>
where
    S: BuildHasher,
    A: GraphProps<Kind = K, Ix = Idx>,
    K: GraphType,
    Idx: HashIndex,
{
    /// returns an iterator over the node entries within the hypergraph, yielding a 2-tuple
    /// consisting of:
    ///
    /// - `0`: a reference to the [`VertexId`](rshyper::VertexId)
    /// - `1`: a reference to the corresponding [`Node`](rshyper::Node).
    pub fn iter_nodes(&self) -> NodeIter<'_, N, Idx> {
        NodeIter {
            iter: self.nodes().iter(),
        }
    }
    /// returns a mutable iterator over the node entries within the hypergraph,yielding a
    /// 2-tuple consisting of:
    ///
    /// - `0`: a reference to the [`VertexId`](rshyper::VertexId)
    /// - `1mutable reference to the corresponding [`Node`](rshyper::Node).
    pub fn iter_nodes_mut(&mut self) -> NodeIterMut<'_, N, Idx> {
        NodeIterMut {
            iter: self.nodes_mut().iter_mut(),
        }
    }
    /// returns an iterator over the edge entries of the hypergraph, yielding a 2-tuple
    /// consisting of:
    ///
    /// - `0`: a reference to the [`EdgeId`](rshyper::EdgeId)
    /// - `1`: a reference to the corresponding [`Edge`](rshyper::HyperEdge).
    pub fn iter_edges(&self) -> EdgeIter<'_, E, K, Idx, S> {
        EdgeIter {
            iter: self.edges().iter(),
        }
    }
    /// returns a mutable iterator over the edge entries of the hypergraph, yielding a 2-tuple
    /// consisting of:
    ///
    ///  - `0`: a reference to the [`EdgeId`](rshyper::EdgeId)
    ///  - `1`: a mutable reference to the corresponding [`Edge`](rshyper::HyperEdge).
    pub fn iter_edges_mut(&mut self) -> EdgeIterMut<'_, E, K, Idx, S> {
        EdgeIterMut {
            iter: self.edges_mut().iter_mut(),
        }
    }
    /// returns an immutable iterator over each of the associated identifiers of the edges
    /// within the graph.
    pub fn iter_edge_keys(&self) -> EdgeKeys<'_, E, K, Idx, S> {
        EdgeKeys {
            iter: self.edges().keys(),
        }
    }
    /// returns an iterator over each of the [`Edge`](rshyper::HyperEdge) values within the graph.
    pub fn facets(&self) -> EdgeValues<'_, E, K, Idx, S> {
        EdgeValues {
            iter: self.edges().values(),
        }
    }
    /// returns a mutable iterator over each of the [`Edge`](rshyper::HyperEdge) values within the
    /// graph.
    pub fn facets_mut(&mut self) -> EdgeValuesMut<'_, E, K, Idx, S> {
        EdgeValuesMut {
            iter: self.edges_mut().values_mut(),
        }
    }
    /// returns an iterator over the keys of the nodes, yielding the indices of the entries.
    pub fn vertices(&self) -> NodeKeys<'_, N, Idx> {
        NodeKeys {
            iter: self.nodes().keys(),
        }
    }
    /// returns an iterator over all the nodes of the hypergraph, producing [`Node`](rshyper::Node)
    /// until exhausted.
    pub fn points(&self) -> NodeValues<'_, N, Idx> {
        NodeValues {
            iter: self.nodes().values(),
        }
    }
    /// returns a mutable iterator over all the nodes of the hypergraph, producing mutable
    /// references to [`Node`](rshyper::Node) until exhausted.
    pub fn points_mut(&mut self) -> NodeValuesMut<'_, N, Idx> {
        NodeValuesMut {
            iter: self.nodes_mut().values_mut(),
        }
    }
    /// returns a sequential iterator over the live edge entries of the hypergraph, in the
    /// order they were created
    pub fn seq_iter_edges(&self) -> SeqEdgeIter<'_, E, K, Idx, S> {
        SeqEdgeIter {
            keys: self.history().edges().iter(),
            edges: self.edges(),
        }
    }
    /// returns a sequential iterator over the indices of the live edges within the hypergraph
    pub fn seq_iter_edge_keys(&self) -> SeqEdgeKeys<'_, E, K, Idx, S> {
        SeqEdgeKeys {
            keys: self.history().edges().iter(),
            edges: self.edges(),
        }
    }
    /// returns a sequential iterator over the live edges of the hypergraph
    pub fn seq_iter_facets(&self) -> SeqEdgeValues<'_, E, K, Idx, S> {
        SeqEdgeValues {
            keys: self.history().edges().iter(),
            edges: self.edges(),
        }
    }
    /// returns a sequential iterator over the nodes of the hypergraph producing references to
    /// the live nodes in the order they were inserted.
    pub fn seq_iter_nodes(&self) -> SeqNodeIter<'_, N, Idx, S> {
        SeqNodeIter {
            keys: self.history().nodes().iter(),
            nodes: self.nodes(),
        }
    }
    /// returns a sequential iterator over the nodes of the hypergraph producing items of type
    /// [`Node`](rshyper::Node) in the order they were inserted.
    pub fn seq_iter_points(&self) -> SeqNodeValues<'_, N, Idx, S> {
        SeqNodeValues {
            keys: self.history().nodes().iter(),
            nodes: self.nodes(),
        }
    }
    /// returns a sequential iterator over _keys_, or vertices, of the hypergraph
    pub fn seq_iter_vertices(&self) -> SeqNodeKeys<'_, N, Idx, S> {
        SeqNodeKeys {
            keys: self.history().nodes().iter(),
            nodes: self.nodes(),
        }
    }
}

#[cfg(feature = "rayon")]
impl<N, E, A, S, K, Idx> HyperMap<N, E, A, S>
where
    S: BuildHasher,
    A: GraphProps<Kind = K, Ix = Idx>,
    K: GraphType,
    Idx: HashIndex,
{
    /// returns a parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper::EdgeId) and the corresponding [`Edge`](rshyper::HyperEdge).
    pub fn par_iter_facets(&self) -> ParEdgeValues<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        ParEdgeValues {
            iter: self.edges().par_values(),
        }
    }
    /// returns a mutable parallel iterator over the surfaces of the hypergraph, yielding pairs of
    /// [`EdgeId`](rshyper::EdgeId) and a mutable reference to the corresponding [`Edge`](rshyper::HyperEdge).
    pub fn par_iter_facets_mut(&mut self) -> ParEdgeValuesMut<'_, E, K, Idx, S>
    where
        E: Send + Sync,
        K: Send + Sync,
        Idx: Send + Sync,
        S: Send + Sync,
    {
        ParEdgeValuesMut {
            iter: self.edges_mut().par_values_mut(),
        }
    }

    /// returns a parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// [`VertexId`](rshyper::VertexId) and the corresponding [`Node`](rshyper::Node).
    pub fn par_iter_points(&self) -> ParNodeValues<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        ParNodeValues {
            iter: self.nodes().par_values(),
        }
    }
    /// returns a mutable parallel iterator over the nodes of the hypergraph, yielding pairs of
    /// references to the [`Node`](rshyper::Node) in the hypergraph.
    pub fn par_iter_points_mut(&mut self) -> ParNodeValuesMut<'_, N, Idx>
    where
        N: Send + Sync,
        Idx: Send + Sync,
    {
        ParNodeValuesMut {
            iter: self.nodes_mut().par_values_mut(),
        }
    }
}
