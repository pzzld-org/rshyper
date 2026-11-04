/*
    Appellation: dft <module>
    Contrib: @FL03
*/
//! this module implements a Depth-First Traversal algorithm for hypergraphs
use hashbrown::DefaultHashBuilder;
use rshyper_core::idx::{VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph};

/// Depth-First Traversal algorithm for hypergraphs
pub struct DepthFirstTraversal<'a, N, E, A, H, S = DefaultHashBuilder>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) stack: Vec<VertexId<A::Ix>>,
    pub(crate) visited: VertexSet<A::Ix, S>,
    pub(crate) _marker: core::marker::PhantomData<(N, E)>,
}
