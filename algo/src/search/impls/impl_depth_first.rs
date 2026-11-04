/*
    Appellation: impl_depth_first <module>
    Created At: 2026.01.06:19:19:58
    Contrib: @FL03
*/
use crate::search::depth_first::DepthFirstTraversal;

use crate::error::Result;
use crate::{Search, Traversal};
use core::hash::{BuildHasher, Hash};
use hashbrown::HashSet;
use rshyper_core::idx::{HashIndex, HyperIndex, VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph, RawEdge};

impl<'a, N, E, H, A, S> DepthFirstTraversal<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
{
    /// Create a new DepthFirstTraversal instance
    pub fn new(graph: &'a H) -> Self
    where
        S: Default,
    {
        Self {
            graph,
            stack: Vec::new(),
            visited: VertexSet::default(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// returns an immutable reference to the stack
    pub const fn stack(&self) -> &Vec<VertexId<A::Ix>> {
        &self.stack
    }
    /// returns a mutable reference to the stack
    pub const fn stack_mut(&mut self) -> &mut Vec<VertexId<A::Ix>> {
        &mut self.stack
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
    /// reset the traversal state
    pub fn reset(&mut self) -> &mut Self {
        #[cfg(feature = "tracing")]
        tracing::debug!("resetting the depth-first traversal operator state...");
        // clear the stack
        self.stack_mut().clear();
        // clear the visited set
        self.visited_mut().clear();
        self
    }
    /// a convience method to perform a search
    pub fn search(
        &mut self,
        start: VertexId<A::Ix>,
    ) -> Result<<Self as Search<VertexId<A::Ix>>>::Output>
    where
        Self: Search<VertexId<A::Ix>>,
        A::Ix: HyperIndex,
    {
        Search::search(self, start)
    }
    /// include the given index in both the stack and visited stores
    pub(crate) fn register_vertex(&mut self, index: VertexId<A::Ix>) -> &mut Self
    where
        A::Ix: Copy + Eq + Hash,
    {
        self.stack_mut().push(index);
        self.visited_mut().insert(index);
        self
    }
}

impl<'a, N, E, A, H, S> Traversal<VertexId<A::Ix>> for DepthFirstTraversal<'a, N, E, A, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: HashIndex,
{
    type Store<I2> = HashSet<I2, S>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        &self.visited
    }
}

impl<'a, N, E, A, H, S> Search<VertexId<A::Ix>> for DepthFirstTraversal<'a, N, E, A, H, S>
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
            return Err(rshyper::Error::NodeNotFound.into());
        }

        // Add start vertex to stack and mark as visited
        self.register_vertex(start);

        // Path to return (traversal order)
        let mut path = Vec::new();

        // DFT algorithm
        while let Some(current) = self.stack.pop() {
            path.push(current);

            // For each hyperedge, visit all vertices that haven't been visited yet
            for edge_id in self.graph.find_edges_with_node(&current) {
                let vertices = self.graph.get_edge_domain(edge_id)?;

                // Add vertices in reverse order to maintain expected DFS behavior
                let mut new_vertices = vertices
                    .into_iter()
                    .filter(|&v| !self.has_visited(v))
                    .copied()
                    .collect::<Vec<_>>();

                // Sort in reverse order (arbitrary but consistent)
                new_vertices.sort_by(|&a, &b| b.cmp(a));

                for v in new_vertices {
                    // add the index to the stack and indicate it has been viewed
                    self.register_vertex(v);
                }
            }
        }

        Ok(path)
    }
}
