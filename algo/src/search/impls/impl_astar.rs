/*
    Appellation: impl_search_a_star <module>
    Created At: 2026.01.06:18:59:39
    Contrib: @FL03
*/
use crate::search::astar::{AStarSearch, ScoreMap, SourceMap};

use crate::error::{Error, Result};
use crate::types::PriorityNode;
use crate::{Heuristic, PathFinder, Search, Traversal};
use alloc::collections::BinaryHeap;
use core::hash::{BuildHasher, Hash};
use hashbrown::HashSet;
use rshyper_core::edge::RawEdge;
use rshyper_core::idx::{HyperIndex, RawIndex, VertexId, VertexSet};
use rshyper_core::{GraphProps, GraphType, HyperGraph, HyperGraphIter, Undirected};

impl<'a, N, E, A, F, H, S, K, Idx> AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    H: HyperGraph<N, E, A>,
    F: Heuristic<Idx>,
    S: BuildHasher,
    K: GraphType,
    Idx: RawIndex,
{
    /// Create a new A* search instance with the given heuristic function
    pub fn new(graph: &'a H, heuristic: F) -> Self
    where
        S: Default,
    {
        Self {
            heuristic,
            graph,
            open_set: VertexSet::default(),
            closed_set: VertexSet::default(),
            came_from: SourceMap::default(),
            g_score: ScoreMap::default(),
            f_score: ScoreMap::default(),
            _marker: core::marker::PhantomData::<(N, E)>,
        }
    }
    /// consumes the current instance to create another from the given heuristic function;
    /// **note:** while the functions may be different, the output type of both must match.
    pub fn with_heuristic<G>(self, heuristic: G) -> AStarSearch<'a, N, E, A, G, H, S>
    where
        G: Heuristic<Idx, Output = F::Output>,
    {
        AStarSearch {
            heuristic,
            graph: self.graph,
            open_set: self.open_set,
            closed_set: self.closed_set,
            came_from: self.came_from,
            g_score: self.g_score,
            f_score: self.f_score,
            _marker: self._marker,
        }
    }

    pub const fn came_from(&self) -> &SourceMap<A::Ix, S> {
        &self.came_from
    }
    /// returns a mutable reference to the map of vertices that have been processed
    pub const fn came_from_mut(&mut self) -> &mut SourceMap<A::Ix, S> {
        &mut self.came_from
    }
    /// returns an immutable reference to the closed set of vertices
    pub const fn closed_set(&self) -> &VertexSet<A::Ix, S> {
        &self.closed_set
    }
    /// returns a mutable reference to the closed set of vertices
    pub const fn closed_set_mut(&mut self) -> &mut VertexSet<A::Ix, S> {
        &mut self.closed_set
    }
    /// returns an immutable reference to the f_score map
    pub const fn f_score(&self) -> &ScoreMap<A::Ix, F::Output, S> {
        &self.f_score
    }
    /// returns a mutable reference to the f_score map
    pub const fn f_score_mut(&mut self) -> &mut ScoreMap<A::Ix, F::Output, S> {
        &mut self.f_score
    }
    /// returns an immutable reference to the g_score map
    pub const fn g_score(&self) -> &ScoreMap<A::Ix, F::Output, S> {
        &self.g_score
    }
    /// returns a mutable reference to the g_score map
    pub const fn g_score_mut(&mut self) -> &mut ScoreMap<A::Ix, F::Output, S> {
        &mut self.g_score
    }
    /// returns an immutable reference to the heuristic function of the algorithm
    pub const fn heuristic(&self) -> &F {
        &self.heuristic
    }
    /// returns an immutable reference to the set of vertices that have been visited
    pub const fn open_set(&self) -> &VertexSet<A::Ix, S> {
        &self.open_set
    }
    /// returns amutable reference to the open set of vertices
    pub const fn open_set_mut(&mut self) -> &mut VertexSet<A::Ix, S> {
        &mut self.open_set
    }
    /// returns true if the given vertex has a f_score
    pub fn has_f_score<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.f_score().contains_key(vertex)
    }
    /// returns true if the given vertex has a g_score
    pub fn has_g_score<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.g_score().contains_key(vertex)
    }
    /// returns true if the given vertex has been visited
    pub fn has_visited<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.closed_set().contains(vertex)
    }
    /// returns true if the given vertex is in the open set
    pub fn in_open_set<Q>(&self, vertex: &Q) -> bool
    where
        Q: ?Sized + Eq + Hash,
        Idx: Eq + Hash,
        VertexId<Idx>: core::borrow::Borrow<Q>,
    {
        self.open_set().contains(vertex)
    }
    /// moves the vertex from the open set before inserting it into the closed set; this is
    /// useful for updating the state, marking a node as processed.
    pub fn move_open_to_closed(&mut self, vertex: &VertexId<Idx>)
    where
        Idx: Copy + Eq + Hash,
    {
        self.open_set_mut().remove(vertex);
        self.closed_set_mut().insert(*vertex);
    }
    /// reset the state
    pub fn reset(&mut self) -> &mut Self {
        self.open_set_mut().clear();
        self.closed_set_mut().clear();
        self.came_from_mut().clear();
        self.g_score_mut().clear();
        self.f_score_mut().clear();
        self
    }
    /// find a path between two nodes
    pub fn find_path(
        &mut self,
        start: VertexId<Idx>,
        goal: VertexId<Idx>,
    ) -> Result<<Self as PathFinder<Idx>>::Path>
    where
        Self: PathFinder<Idx>,
    {
        PathFinder::find_path(self, start, goal)
    }
    /// a convience method to perform a search
    pub fn search(
        &mut self,
        start: VertexId<Idx>,
    ) -> Result<<Self as Search<VertexId<Idx>>>::Output>
    where
        Self: Search<VertexId<Idx>>,
    {
        Search::search(self, start)
    }
}

/// The heap key for an f-score: for finite `f >= 0` the IEEE-754 bit pattern orders exactly
/// like the value, so this is order-preserving with no truncation (`+ 0.0` maps `-0.0` to `0.0`).
fn f_key(f: f64) -> i64 {
    (f + 0.0).to_bits() as i64
}

/// the heuristic's estimate, rejected unless it is finite and non-negative
fn checked_h(h: f64) -> Result<f64> {
    if h.is_finite() && h >= 0.0 {
        Ok(h)
    } else {
        Err(Error::InvalidHeuristic)
    }
}

/// Shortest paths by hop count over the undirected co-member 2-section: a step goes from a
/// vertex to any other member of an incident hyperedge and costs `1.0`; edge weights are
/// ignored.
///
/// The heuristic must be *consistent* (`h(u) <= 1 + h(v)` for every step `u → v`, and
/// `h(goal) = 0`): closed vertices are never reopened, so an inconsistent heuristic can return
/// a longer path.
///
/// # Errors
///
/// - [`Error::UnsupportedGraphKind`] unless the graph is `Undirected` (#118).
/// - [`Error::InvalidHeuristic`] if the heuristic returns a negative, NaN or infinite value.
/// - [`Error::PathNotFound`] if `goal` is unreachable.
impl<'a, N, E, F, A, H, S> PathFinder<A::Ix> for AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    F: Heuristic<A::Ix, Output = f64>,
    S: BuildHasher,
    A::Ix: HyperIndex,
    for<'b> &'b <H::Edge<E> as RawEdge>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
{
    type Path = Vec<VertexId<A::Ix>>;
    /// Find the shortest path between start and goal vertices
    fn find_path(&mut self, start: VertexId<A::Ix>, goal: VertexId<A::Ix>) -> Result<Self::Path> {
        // Check if both vertices exist
        if !self.graph.contains_node(&start) {
            return Err(rshyper::Error::NodeNotFound.into());
        }
        if !self.graph.contains_node(&goal) {
            return Err(rshyper::Error::NodeNotFound.into());
        }
        if core::any::TypeId::of::<A::Kind>() != core::any::TypeId::of::<Undirected>() {
            return Err(Error::UnsupportedGraphKind(
                core::any::type_name::<A::Kind>(),
            ));
        }

        // reset state
        self.reset();
        // initialize g_score for start node (0) and infinity for all other nodes
        self.g_score_mut().insert(start, 0.0);

        // initialize f_score for start node (heuristic only since g=0)
        let initial_fscore = checked_h(self.heuristic().compute(start, goal))?;
        self.f_score_mut().insert(start, initial_fscore);
        // add start node to the open set
        self.open_set_mut().insert(start);
        // initialize priority queue
        let mut priority_queue = BinaryHeap::new();
        // push the start node with its f_score
        priority_queue.push(PriorityNode {
            vertex: start,
            priority: f_key(initial_fscore),
        });
        // track processed nodes to avoid duplicate processing
        let mut processed = HashSet::new();
        // process nodes until the queue is empty or we attain the goal
        while let Some(PriorityNode {
            vertex: current, ..
        }) = priority_queue.pop()
        {
            // Skip if we've already processed this vertex with a better path
            // or it's no longer in the open set
            if processed.contains(&current) || !self.in_open_set(&current) {
                continue;
            }
            // add the current vertex to the processed set
            processed.insert(current);

            // If we've reached the goal, construct and return the path
            if current == goal {
                return Ok(self.reconstruct_path(goal));
            }

            // Move from open to closed set
            self.move_open_to_closed(&current);

            // Get all hyperedges containing the current vertex
            let graph = self.graph;
            for edge_id in graph.find_edges_with_node(&current) {
                // Get all vertices in this hyperedge
                let vertices = graph.get_edge_domain(edge_id)?;

                // Process each vertex in this hyperedge
                for &neighbor in vertices {
                    // Skip if this is the current vertex or already evaluated
                    if neighbor == current || self.has_visited(&neighbor) {
                        continue;
                    }

                    // Cost to reach neighbor through current vertex
                    let tentative_g_score = self.g_score[&current] + 1.0;

                    // Check if this path is better than any previous path
                    let is_better_path =
                        !self.has_g_score(&neighbor) || tentative_g_score < self.g_score[&neighbor];

                    if is_better_path {
                        // Update path info
                        self.came_from_mut().insert(neighbor, current);
                        self.g_score_mut().insert(neighbor, tentative_g_score);

                        // Update f_score (g_score + heuristic)
                        let f_score = tentative_g_score
                            + checked_h(self.heuristic().compute(neighbor, goal))?;
                        self.f_score_mut().insert(neighbor, f_score);

                        // Add to open set if not already there
                        if !self.in_open_set(&neighbor) {
                            self.open_set_mut().insert(neighbor);
                        }

                        // `PriorityNode` is already a min-heap node, so the key is not negated
                        priority_queue.push(PriorityNode {
                            vertex: neighbor,
                            priority: f_key(f_score),
                        });
                    }
                }
            }
        }

        // No path found
        Err(Error::PathNotFound)
    }

    // Reconstruct path from came_from map
    fn reconstruct_path(&self, goal: VertexId<A::Ix>) -> Self::Path {
        let mut path = vec![goal];
        let mut current = goal;

        while let Some(&prev) = self.came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.reverse();
        path
    }
}

impl<'a, N, E, F, A, H, S> Traversal<VertexId<A::Ix>> for AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps,
    F: Heuristic<A::Ix, Output = f64>,
    H: HyperGraph<N, E, A>,
    S: BuildHasher,
    A::Ix: Eq + Hash,
{
    type Store<U> = HashSet<U, S>;

    fn has_visited(&self, vertex: &VertexId<A::Ix>) -> bool {
        self.visited().contains(vertex)
    }

    fn visited(&self) -> &Self::Store<VertexId<A::Ix>> {
        self.closed_set()
    }
}

impl<'a, N, E, F, A, H, S> Search<VertexId<A::Ix>> for AStarSearch<'a, N, E, A, F, H, S>
where
    A: GraphProps,
    F: Heuristic<A::Ix, Output = f64>,
    H: HyperGraphIter<N, E, A>,
    S: BuildHasher,
    A::Ix: HyperIndex,
    for<'b> &'b <H::Edge<E> as RawEdge>::Store: IntoIterator<Item = &'b VertexId<A::Ix>>,
{
    type Output = Vec<VertexId<A::Ix>>;

    fn search(&mut self, start: VertexId<A::Ix>) -> Result<Self::Output> {
        // For A*, we need a goal vertex to compute the heuristic
        // This implementation of search will explore the graph and return
        // all reachable vertices ordered by their distance from start
        self.reset();

        if !self.graph.contains_node(&start) {
            return Err(rshyper::Error::NodeNotFound.into());
        }

        // Using the vertex with the largest ID as a pseudo-goal
        // This is a hack to make A* behave more like a general search
        let max_vertex_id = match self.graph.vertices().max() {
            Some(&id) => id,
            None => return Ok(vec![]),
        };

        self.find_path(start, max_vertex_id)
    }
}
