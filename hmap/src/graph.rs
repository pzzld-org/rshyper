/*
    Appellation: hyper_map <module>
    Contrib: @FL03
*/
use crate::types::prelude::*;

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use hashbrown::DefaultHashBuilder;
use rshyper_core::attrs::{DiAttrs, GraphProps, UnAttrs};
use rshyper_core::idx::{self, EdgeId, HashIndex, IndexTracker, RawIndex, Udx, VertexId};
use rshyper_core::{AddStep, GraphType, Mode};

/// a type alias for a [directed](rshyper_core::Directed) [`HyperMap`]
pub type DiHyperMap<N, E, Idx = Udx, S = DefaultHashBuilder> = HyperMap<N, E, DiAttrs<Idx>, S>;
/// a type alias for an [undirected](rshyper_core::Undirected) [`HyperMap`]
pub type UnHyperMap<N, E, Idx = Udx, S = DefaultHashBuilder> = HyperMap<N, E, UnAttrs<Idx>, S>;

/// The [`HyperMap`] is a map-based implementation of a hypergraph that provides a flexible and
/// efficient way to store and manipulate hypergraphs. It is designed to be generic over the
/// types of nodes N, edges E, attributes A, and the hasher S used for hashing the nodes and
/// edges. This design allows for a wide range of applications, from simple hypergraphs to more
/// complex structures with custom attributes and hashing strategies.
///
/// ## Overview
///
/// This implementation focuses on establishing a solid foundation for a hypergraph, relying on
/// various traits to help shape its behavior and properties. Additionally, the internal
/// history imbues the instance with a native sense of order enabling the design of sequential
/// iteraotrs that respect the order in which the respective component was created in.
#[derive(Clone, Default)]
pub struct HyperMap<N = (), E = (), A = UnAttrs<Udx>, S = DefaultHashBuilder>
where
    A: GraphProps,
    S: BuildHasher,
{
    /// `edges` represent the hyperedges of the hypergraph, each identified by an `EdgeId`
    pub(crate) edges: EdgeMap<E, A::Kind, A::Ix, S>,
    /// the `nodes` of a hypergraph are the vertices, each identified by a `VertexId` and
    /// associated with a weight of type `N`.
    pub(crate) nodes: NodeMap<N, A::Ix, S>,
    /// the attributes of a graph define its _kind_ and the type of index used
    pub(crate) attrs: A,
    /// tracks the current position of the hypergraph, which is used to determine the next
    /// available indices for edges and vertices.
    pub(crate) history: IndexTracker<A::Ix>,
}

impl<N, E, A, K, Ix, S> HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Ix, Kind = K>,
    S: BuildHasher,
    K: GraphType,
    Ix: RawIndex,
{
    /// returns a new, empty instance of the [`HyperMap`] using the default [`BuildHasher`] an
    /// for the indices.
    pub fn new() -> Self
    where
        Ix: Default,
        S: Default,
    {
        Self {
            edges: EdgeMap::default(),
            nodes: NodeMap::default(),
            attrs: A::new(),
            history: Default::default(),
        }
    }
    /// creates a new instance of the hypergraph with the given capacity for edges and nodes
    pub fn with_capacity(edges: usize, nodes: usize) -> Self
    where
        Ix: Default,
        S: Default,
    {
        Self {
            edges: EdgeMap::with_capacity_and_hasher(edges, Default::default()),
            nodes: NodeMap::with_capacity_and_hasher(nodes, Default::default()),
            attrs: A::new(),
            history: Default::default(),
        }
    }
    #[doc(hidden)]
    /// initializes a new instance of the [`HyperMap`] with the given hasher and capacity for
    /// nodes and edges.
    pub fn with_capacity_and_hasher(edges: usize, nodes: usize, hash_builder: S) -> Self
    where
        Ix: Default,
        S: Clone,
    {
        HyperMap {
            edges: EdgeMap::with_capacity_and_hasher(edges, hash_builder.clone()),
            nodes: NodeMap::with_capacity_and_hasher(nodes, hash_builder),
            attrs: A::new(),
            history: Default::default(),
        }
    }
    #[doc(hidden)]
    /// initialize a new instance of the [`HyperMap`] configured with the given [`BuildHasher`]
    pub fn with_hasher(hash_builder: S) -> Self
    where
        Ix: Default,
        S: Clone,
    {
        HyperMap {
            edges: EdgeMap::with_hasher(hash_builder.clone()),
            nodes: NodeMap::with_hasher(hash_builder),
            attrs: A::new(),
            history: Default::default(),
        }
    }
    #[doc(hidden)]
    /// returns a copy of the graph attributes; almost never used, however, it is useful for
    /// extracting certain truths about the hypergraph.
    pub(crate) const fn attrs(&self) -> A {
        self.attrs
    }
    /// returns the [`Mode`] of the hypergraph
    pub fn mode(&self) -> Mode {
        self.attrs().mode()
    }
    /// returns am immutable reference to the nodes
    pub const fn nodes(&self) -> &NodeMap<N, Ix, S> {
        &self.nodes
    }
    /// returns a mutable reference to the nodes of the hypergraph
    pub const fn nodes_mut(&mut self) -> &mut NodeMap<N, Ix, S> {
        &mut self.nodes
    }
    /// returns an immutable reference to the history of the hypergraph, which is used to track
    /// the indices of edges and vertices that have been created so far.
    pub const fn history(&self) -> &IndexTracker<Ix> {
        &self.history
    }
    /// returns a mutable reference to the history of the hypergraph, which is used to track
    /// the indices of edges and vertices that have been created so far.
    pub const fn history_mut(&mut self) -> &mut IndexTracker<Ix> {
        &mut self.history
    }
    /// returns a copy of the position of the hypergraph; here, the [`position`](idx::IndexFrame) is
    /// used to track the indices (edge & vertex) and define which ones are next to be used
    /// when inserting new hyperedges or vertices
    pub const fn position(&self) -> &idx::IndexFrame<Ix> {
        self.history().cursor()
    }
    /// returns a mutable reference to the current position of the hypergraph;
    pub const fn position_mut(&mut self) -> &mut idx::IndexFrame<Ix> {
        self.history_mut().cursor_mut()
    }
    /// returns an immutable reference to the surfaces of the hypergraph
    pub const fn edges(&self) -> &EdgeMap<E, K, Ix, S> {
        &self.edges
    }
    /// returns a mutable reference to the surfaces of the hypergraph
    pub const fn edges_mut(&mut self) -> &mut EdgeMap<E, K, Ix, S> {
        &mut self.edges
    }
    #[inline]
    /// overrides the current surfaces and returns a mutable reference to the hypergraph
    pub fn set_edges(&mut self, edges: EdgeMap<E, K, Ix, S>) -> &mut Self {
        self.edges = edges;
        self
    }
    /// overrides the current nodes and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_nodes(&mut self, nodes: NodeMap<N, Ix, S>) -> &mut Self
    where
        Ix: Default,
    {
        self.nodes = nodes;
        self
    }
    /// overrides the current history and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_history(&mut self, history: IndexTracker<Ix>) -> &mut Self
    where
        Ix: Default,
    {
        *self.history_mut() = history;
        self
    }
    /// overrides the current position and returns a mutable reference to the hypergraph
    #[inline]
    pub fn set_position(&mut self, cursor: idx::IndexFrame<Ix>) -> &mut Self
    where
        Ix: Default,
    {
        self.history_mut().set_cursor(cursor);
        self
    }
    /// returns true if the hypergraph contains an edge with the given index;
    pub fn contains_edge<Q>(&self, index: &Q) -> bool
    where
        Ix: HashIndex,
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.edges().contains_key(index)
    }
    /// check if a vertex with the given id exists
    pub fn contains_node<Q>(&self, index: &Q) -> bool
    where
        Ix: HashIndex,
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.nodes().contains_key(index)
    }
    /// returns true if the vertex is contained in the hyperedge with the given id
    pub fn is_node_in_domain<Q, Q2>(&self, index: &Q, vertex: &Q2) -> bool
    where
        Ix: HashIndex,
        Q: ?Sized + Eq + Hash,
        Q2: Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
        VertexId<Ix>: Borrow<Q2>,
    {
        if let Some(surface) = self.edges().get(index) {
            return surface.contains(vertex);
        }
        false
    }
    /// returns true if the hypergraph is empty, meaning it has no edges nor any nodes.
    pub fn is_empty(&self) -> bool {
        self.edges().is_empty() && self.nodes().is_empty()
    }
    /// returns true if the hypergraph is directed
    pub fn is_directed(&self) -> bool {
        self.attrs().is_directed()
    }
    /// returns true if the hypergraph is undirected
    pub fn is_undirected(&self) -> bool {
        self.attrs().is_undirected()
    }
    /// returns an [`EdgeEntry`] for the edge associated with the given index, allowing for
    /// in-place modifications or insertions to the mapping
    pub fn edge(&mut self, index: EdgeId<Ix>) -> EdgeEntry<'_, E, K, Ix, S>
    where
        Ix: HashIndex,
    {
        self.edges_mut().entry(index)
    }
    /// returns an [`Entry`](std::collections::hash_map::Entry) for the node with the given
    /// index, allowing for modifications or insertions to the mapping
    pub fn node(&mut self, index: VertexId<Ix>) -> NodeEntry<'_, N, Ix, S>
    where
        Ix: HashIndex,
    {
        self.nodes_mut().entry(index)
    }
    /// allocates and records the next edge index; see [`IndexTracker::next_edge`].
    ///
    /// # Errors
    ///
    /// [`IndexError::DuplicateIndex`](idx::IndexError::DuplicateIndex) if the cursor's id is
    /// already in the history (for example after [`set_position`](Self::set_position) moved
    /// it back) and [`IndexError::IndexOutOfBounds`](idx::IndexError::IndexOutOfBounds) if
    /// the index space is exhausted. The graph is unchanged on error.
    pub fn next_edge_id(&mut self) -> rshyper_core::Result<EdgeId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Clone + PartialOrd,
    {
        Ok(self.history_mut().next_edge()?)
    }
    /// allocates and records the next node index; see [`next_edge_id`](Self::next_edge_id)
    /// for the errors.
    pub fn next_vertex_id(&mut self) -> rshyper_core::Result<VertexId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Clone + PartialOrd,
    {
        Ok(self.history_mut().next_vertex()?)
    }
    /// returns the order of the hypergraph, which is defined to be the number of nodes in `X`
    /// where `H=(X,E)`.
    pub fn order(&self) -> usize {
        self.nodes().len()
    }
    /// returns the size of the hypergraph, which is defined to be the number of edges in `E`
    /// where `H=(X,E)`.
    /// returns the total number of edges within the hypergraph
    pub fn size(&self) -> usize {
        self.edges().len()
    }
}

impl<N, E, A, S> core::fmt::Debug for HyperMap<N, E, A, S>
where
    A: GraphProps,
    E: core::fmt::Debug,
    N: core::fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HyperMap")
            .field("history", self.history())
            .field("nodes", self.nodes())
            .field("surfaces", self.edges())
            .finish()
    }
}

impl<N, E, A, S> core::fmt::Display for HyperMap<N, E, A, S>
where
    A: GraphProps,
    E: core::fmt::Debug,
    N: core::fmt::Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ history: {h:?}, edges: {e:?}, nodes: {n:?} }}",
            n = self.nodes(),
            e = self.edges(),
            h = self.history()
        )
    }
}
