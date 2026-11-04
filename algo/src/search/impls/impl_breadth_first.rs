/*
    Appellation: impl_breadth_first <module>
    Created At: 2026.01.06:19:19:55
    Contrib: @FL03
*/
use crate::search::breadth_first::BreadthFirstTraversal;

use crate::error::{Error, Result};
use crate::search::Search;
use alloc::collections::VecDeque;
use core::hash::{BuildHasher, Hash};
use rshyper_core::idx::{HyperIndex, VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph, RawEdge};

impl<'a, N, E, A, H, S> BreadthFirstTraversal<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
{
    /// create a new instance from a hypergraph
    pub fn new(graph: &'a H) -> Self
    where
        S: Default,
    {
        Self {
            graph,
            queue: Default::default(),
            visited: Default::default(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// returns an immutable reference to the queue
    pub const fn queue(&self) -> &VecDeque<VertexId<A::Ix>> {
        &self.queue
    }
    /// returns a mutable reference to the queue
    pub(crate) const fn queue_mut(&mut self) -> &mut VecDeque<VertexId<A::Ix>> {
        &mut self.queue
    }
    /// returns an immutable reference to the visited vertices
    pub const fn visited(&self) -> &VertexSet<A::Ix, S> {
        &self.visited
    }
    /// returns a mutable reference to the visited vertices
    pub const fn visited_mut(&mut self) -> &mut VertexSet<A::Ix, S> {
        &mut self.visited
    }
    /// returns true if the vertex has been visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        A::Ix: Eq + Hash,
        Q: ?Sized + Eq + Hash,
        VertexId<A::Ix>: core::borrow::Borrow<Q>,
    {
        self.visited().contains(vertex)
    }
    /// Reset the traversal state to allow reusing the instance
    pub fn reset(&mut self) -> &mut Self {
        self.queue_mut().clear();
        self.visited_mut().clear();
        self
    }
    /// a convience method to perform a search
    pub fn search(&mut self, start: VertexId<A::Ix>) -> Result<Vec<VertexId<A::Ix>>>
    where
        A::Ix: HyperIndex,
        for<'b> &'b <H::Edge<E> as RawEdge>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
    {
        Search::search(self, start)
    }
    /// if the vertex hans't been visited yet, push it to the back of the queue and mark it as
    /// visited by inserting it into the visited set.
    pub(crate) fn register(&mut self, vertex: VertexId<A::Ix>)
    where
        A::Ix: Copy + Eq + Hash,
    {
        if !self.has_visited(&vertex) {
            self.queue_mut().push_back(vertex);
            self.visited_mut().insert(vertex);
        }
    }
}

impl<'a, N, E, A, H, S> Search<VertexId<A::Ix>> for BreadthFirstTraversal<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: HyperIndex,
    for<'b> &'b <H::Edge<E> as RawEdge>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
{
    type Output = Vec<VertexId<A::Ix>>;

    fn search(&mut self, start: VertexId<A::Ix>) -> Result<Self::Output> {
        // Reset state
        self.reset();

        // Check if starting vertex exists
        if !self.graph.contains_node(&start) {
            return Err(Error::NotFound(Box::new(start.value())));
        }

        // Add start vertex to queue and mark as visited
        self.queue.push_back(start);
        self.visited.insert(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // BFT algorithm
        while let Some(current) = self.queue_mut().pop_front() {
            path.push(current);

            // visit all vertices within each edge that haven't been visited yet
            for edge_id in self.graph.find_edges_with_node(&current) {
                for vertex in self.graph.get_edge_domain(edge_id)? {
                    self.register(*vertex);
                }
            }
        }

        Ok(path)
    }
}
