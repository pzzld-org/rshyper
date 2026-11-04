/*
    Appellation: bft <module>
    Contrib: @FL03
*/
//! this module implements the breadth-first search algorithm as an operator on the hypergraph.

use crate::traits::Traversal;
use alloc::collections::VecDeque;
use core::hash::BuildHasher;
use hashbrown::{DefaultHashBuilder, HashSet};
use rshyper_core::idx::{HyperIndex, VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph};

/// Breadth-First Traversal algorithm for hypergraphs
pub struct BreadthFirstTraversal<'a, N, E, A, H, S = DefaultHashBuilder>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) queue: VecDeque<VertexId<A::Ix>>,
    pub(crate) visited: VertexSet<A::Ix, S>,
    pub(crate) _marker: core::marker::PhantomData<(N, E)>,
}

impl<'a, N, E, A, H, S> Traversal<VertexId<A::Ix>> for BreadthFirstTraversal<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: HyperIndex,
{
    type Store<I2> = HashSet<I2, S>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        self.visited()
    }
}
