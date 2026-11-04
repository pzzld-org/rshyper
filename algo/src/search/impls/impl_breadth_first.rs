/*
    Appellation: impl_breadth_first <module>
    Created At: 2026.01.06:19:19:55
    Contrib: @FL03
*/
use crate::search::breadth_first::BreadthFirstTraversal;

use crate::error::{Error, Result};
use crate::search::Search;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::any::{TypeId, type_name};
use core::hash::{BuildHasher, Hash};
use hashbrown::HashSet;
use rshyper_core::idx::{EdgeId, HyperIndex, VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph, RawEdge, Undirected};

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
    /// **Unstable:** the signature may change until the traversal taxonomy (#49) and the path
    /// semantics (ADR-PATH) are settled.
    ///
    /// A bounded breadth-first search over the co-membership relation (the 2-section) of the
    /// sub-hypergraph that `edge_filter` keeps, undirected: `u` and `v` are adjacent when some
    /// kept hyperedge contains both. It returns the vertices reached from `start`, nearest layer
    /// first and each layer in ascending [`VertexId`] order, so the result and any truncation
    /// of it are the same whatever the hasher.
    ///
    /// - `max_depth` counts hyperedge hops (`v → e → u` is one hop); depth 0 returns `[start]`.
    /// - `limit` caps the number of vertices returned; pass `usize::MAX` for no cap.
    /// - `edge_filter(id, weight)`: a rejected edge connects nothing. It runs at most once per
    ///   edge.
    /// - `vertex_filter(id, weight)`: a rejected vertex is neither returned nor expanded. It runs
    ///   at most once per vertex and is never asked about `start`.
    ///
    /// The filters should be pure: the order they are called in follows the graph's edge order.
    /// Every kept edge's domain is read once, so the search costs one `find_edges_with_node` per
    /// expanded vertex, plus O(Σ |e|) over the kept edges it reaches, plus sorting each layer.
    ///
    /// # Errors
    ///
    /// - [`Error::NotFound`] if `start` is not in the graph.
    /// - [`Error::UnsupportedGraphKind`] unless the graph is `Undirected` (directed semantics
    ///   are not decided yet, ADR-DIR, #118).
    /// - the graph's error if a reached edge member has no node entry (a broken graph).
    pub fn search_bounded<F, G>(
        &mut self,
        start: VertexId<A::Ix>,
        max_depth: usize,
        limit: usize,
        mut edge_filter: F,
        mut vertex_filter: G,
    ) -> Result<Vec<VertexId<A::Ix>>>
    where
        A::Ix: HyperIndex,
        F: FnMut(&EdgeId<A::Ix>, &E) -> bool,
        G: FnMut(&VertexId<A::Ix>, &N) -> bool,
        for<'b> &'b <H::Edge<E> as RawEdge>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
    {
        self.reset();
        if !self.graph.contains_node(&start) {
            return Err(Error::NotFound(Box::new(start.value())));
        }
        if TypeId::of::<A::Kind>() != TypeId::of::<Undirected>() {
            return Err(Error::UnsupportedGraphKind(type_name::<A::Kind>()));
        }
        if limit == 0 {
            return Ok(Vec::new());
        }
        // `visited` holds `start` and every vertex `vertex_filter` has judged, kept or not
        self.visited.insert(start);
        let mut seen_edges = HashSet::<EdgeId<A::Ix>>::new();
        let mut path = alloc::vec![start];
        let mut layer = alloc::vec![start];
        for _ in 0..max_depth {
            if layer.is_empty() || path.len() >= limit {
                break;
            }
            let mut next = Vec::new();
            for v in &layer {
                for e in self.graph.find_edges_with_node(v) {
                    if !seen_edges.insert(*e)
                        || !edge_filter(e, self.graph.get_edge_weight(e)?.get())
                    {
                        continue;
                    }
                    for u in self.graph.get_edge_domain(e)? {
                        if self.visited.insert(*u)
                            && vertex_filter(u, self.graph.get_node_weight(u)?.get())
                        {
                            next.push(*u);
                        }
                    }
                }
            }
            next.sort_unstable();
            next.truncate(limit - path.len());
            path.extend_from_slice(&next);
            layer = next;
        }
        Ok(path)
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
                for vertex in self.graph.get_edge_domain(edge_id).expect("Edge is empty") {
                    self.register(*vertex);
                }
            }
        }

        Ok(path)
    }
}
