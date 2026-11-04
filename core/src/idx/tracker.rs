/*
    appellation: tracker <module>
    authors: @FL03
*/
use super::{EdgeId, IndexFrame, RawIndex, VertexId};
use crate::AddStep;
use crate::idx::error::{IndexError, IndexResult};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// the [`IndexTracker`] keeps a history of the created indices and the current cursor position.
///
/// Each history is mirrored by a count of every id it holds (duplicates included) so that
/// [`next_edge`](Self::next_edge) and
/// [`next_vertex`](Self::next_vertex) reject recorded ids in `O(log h)`. Every method keeps the
/// two in sync except the raw accessors ([`edges_mut`](Self::edges_mut),
/// [`nodes_mut`](Self::nodes_mut), [`edge_mut`](Self::edge_mut) and
/// [`node_mut`](Self::node_mut)), which bypass the mirror (#105).
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(
        bound(deserialize = "Ix: Clone + Default + Ord + serde::Deserialize<'de>"),
        from = "RawTracker<Ix>",
        rename_all = "snake_case"
    )
)]
pub struct IndexTracker<Ix = usize>
where
    Ix: RawIndex,
{
    pub(crate) cursor: IndexFrame<Ix>,
    pub(crate) edges: Vec<EdgeId<Ix>>,
    pub(crate) nodes: Vec<VertexId<Ix>>,
    /// counts each id in `edges` for membership checks
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) edge_counts: BTreeMap<Ix, usize>,
    /// counts each id in `nodes` for membership checks
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) node_counts: BTreeMap<Ix, usize>,
}

impl<Ix> IndexTracker<Ix>
where
    Ix: RawIndex,
{
    /// creates a new instance with an empty history and a default cursor.
    pub fn new() -> Self
    where
        Ix: Default,
    {
        Self::from_cursor(IndexFrame::default())
    }
    /// creates a new instance with an empty history and the given cursor.
    pub fn from_cursor(cursor: IndexFrame<Ix>) -> Self {
        Self {
            cursor,
            edges: Vec::new(),
            nodes: Vec::new(),
            edge_counts: BTreeMap::new(),
            node_counts: BTreeMap::new(),
        }
    }
    /// create a new history with the cursor initialized to `1`
    pub fn one() -> Self
    where
        Ix: num_traits::One,
    {
        Self::from_cursor(IndexFrame::one())
    }
    /// create a new history with the cursor initialized to `0`
    pub fn zero() -> Self
    where
        Ix: num_traits::Zero,
    {
        Self::from_cursor(IndexFrame::zero())
    }
    /// returns a reference to the current cursor.
    pub const fn cursor(&self) -> &IndexFrame<Ix> {
        &self.cursor
    }
    /// returns a mutable reference to the current cursor.
    pub const fn cursor_mut(&mut self) -> &mut IndexFrame<Ix> {
        &mut self.cursor
    }
    /// returns an immutable reference to the edge history
    pub const fn edges(&self) -> &Vec<EdgeId<Ix>> {
        &self.edges
    }
    /// returns a mutable reference to the edge history; writes through it bypass the membership
    /// mirror (#105)
    pub const fn edges_mut(&mut self) -> &mut Vec<EdgeId<Ix>> {
        &mut self.edges
    }
    /// returns an immutable reference to the node history
    pub const fn nodes(&self) -> &Vec<VertexId<Ix>> {
        &self.nodes
    }
    /// returns a mutable reference to the history of created nodes; writes through it bypass the
    /// membership mirror (#105)
    pub const fn nodes_mut(&mut self) -> &mut Vec<VertexId<Ix>> {
        &mut self.nodes
    }
    /// set the current position and return a mutable reference to the tracker
    #[inline]
    pub fn set_cursor(&mut self, cursor: IndexFrame<Ix>) -> &mut Self {
        *self.cursor_mut() = cursor;
        self
    }
    /// overwrite the edge history and return a mutable reference to the tracker
    #[inline]
    pub fn set_edges(&mut self, edges: Vec<EdgeId<Ix>>) -> &mut Self
    where
        Ix: Clone + Ord,
    {
        self.edge_counts = tally(edges.iter().map(|id| id.get()));
        self.edges = edges;
        self
    }
    /// overwrite the node history and return a mutable reference to the tracker
    #[inline]
    pub fn set_nodes(&mut self, nodes: Vec<VertexId<Ix>>) -> &mut Self
    where
        Ix: Clone + Ord,
    {
        self.node_counts = tally(nodes.iter().map(|id| id.get()));
        self.nodes = nodes;
        self
    }
    /// consumes the current instance to create another with the given position
    #[inline]
    pub fn with_cursor(self, cursor: IndexFrame<Ix>) -> Self {
        Self { cursor, ..self }
    }
    /// consumes the current instance to create another with the given edge history
    #[inline]
    pub fn with_edges(mut self, edges: Vec<EdgeId<Ix>>) -> Self
    where
        Ix: Clone + Ord,
    {
        self.set_edges(edges);
        self
    }
    /// consumes the current instance to create another with the given node history
    #[inline]
    pub fn with_nodes(mut self, nodes: Vec<VertexId<Ix>>) -> Self
    where
        Ix: Clone + Ord,
    {
        self.set_nodes(nodes);
        self
    }
    /// add a new edge index to the history.
    #[inline]
    pub fn add_edge(&mut self, index: EdgeId<Ix>) -> &mut Self
    where
        Ix: Clone + Ord,
    {
        *self.edge_counts.entry(index.get().clone()).or_insert(0) += 1;
        self.edges_mut().push(index);
        self
    }
    /// add a new node index to the history.
    #[inline]
    pub fn add_node(&mut self, index: VertexId<Ix>) -> &mut Self
    where
        Ix: Clone + Ord,
    {
        *self.node_counts.entry(index.get().clone()).or_insert(0) += 1;
        self.nodes_mut().push(index);
        self
    }
    /// clears the recorded edges and nodes before resetting the cursor to the initial position
    pub fn clear(&mut self) -> &mut Self
    where
        Ix: Default,
    {
        // clear the nodes and edges
        self.edges_mut().clear();
        self.nodes_mut().clear();
        self.edge_counts.clear();
        self.node_counts.clear();
        // reset the cursor to the initial position
        self.cursor_mut().reset();
        // return a mutable reference
        self
    }

    /// returns true if the element is in the edge history
    #[inline]
    pub fn contains_edge(&self, index: &EdgeId<Ix>) -> bool
    where
        Ix: PartialEq,
    {
        self.edges().contains(index)
    }
    /// returns true if the element is in the node history
    #[inline]
    pub fn contains_node(&self, index: &VertexId<Ix>) -> bool
    where
        Ix: PartialEq,
    {
        self.nodes().contains(index)
    }
    /// returns a reference to the edge if at the given index
    #[inline]
    pub fn edge<I>(&self, index: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[EdgeId<Ix>]>,
    {
        self.edges().get(index)
    }
    /// returns a mutable reference to the edge id; writes through it bypass the membership
    /// mirror (#105)
    #[inline]
    pub fn edge_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: core::slice::SliceIndex<[EdgeId<Ix>]>,
    {
        self.edges_mut().get_mut(index)
    }
    /// returns a reference to the node if at the given index
    #[inline]
    pub fn node<I>(&self, index: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[VertexId<Ix>]>,
    {
        self.nodes().get(index)
    }
    /// returns a mutable reference to the node id; writes through it bypass the membership
    /// mirror (#105)
    #[inline]
    pub fn node_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: core::slice::SliceIndex<[VertexId<Ix>]>,
    {
        self.nodes_mut().get_mut(index)
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn remove_edge(&mut self, index: &Ix) -> &mut Self
    where
        Ix: Ord,
    {
        self.retain_edges(|i| i != index);
        self
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn remove_node(&mut self, index: &Ix) -> &mut Self
    where
        Ix: Ord,
    {
        self.retain_nodes(|i| i != index);
        self
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn retain_edges<F>(&mut self, mut f: F)
    where
        F: FnMut(&EdgeId<Ix>) -> bool,
        Ix: Ord,
    {
        let Self {
            edges, edge_counts, ..
        } = self;
        edges.retain(|id| {
            let keep = f(id);
            if !keep {
                untally(edge_counts, id.get());
            }
            keep
        });
    }
    /// retains only nodes satisfying the predicate `f`.
    #[inline]
    pub fn retain_nodes<F>(&mut self, mut f: F)
    where
        F: FnMut(&VertexId<Ix>) -> bool,
        Ix: Ord,
    {
        let Self {
            nodes, node_counts, ..
        } = self;
        nodes.retain(|id| {
            let keep = f(id);
            if !keep {
                untally(node_counts, id.get());
            }
            keep
        });
    }
    /// steps the edge index forward before storing and returning the previous index; if that
    /// index is already in the history, the tracker is left unchanged and
    /// [`DuplicateIndex`](IndexError::DuplicateIndex) is returned.
    #[inline]
    pub fn next_edge(&mut self) -> IndexResult<EdgeId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Clone + Ord,
    {
        // check the on-deck index before stepping so a rejection leaves the cursor untouched
        if self.edge_counts.contains_key(self.cursor().edge().get()) {
            return Err(IndexError::DuplicateIndex);
        }
        // step the current edge cursor forward before replacing and returning
        // the previous index
        let prev = self.cursor_mut().next_edge()?;
        // add the previous edge index to the history
        self.add_edge(prev.clone());
        // return the previous edge index
        Ok(prev)
    }
    /// steps the node index forward before storing and returning the previous index; if that
    /// index is already in the history, the tracker is left unchanged and
    /// [`DuplicateIndex`](IndexError::DuplicateIndex) is returned.
    #[inline]
    pub fn next_vertex(&mut self) -> IndexResult<VertexId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Clone + Ord,
    {
        // check the on-deck index before stepping so a rejection leaves the cursor untouched
        if self.node_counts.contains_key(self.cursor().node().get()) {
            return Err(IndexError::DuplicateIndex);
        }
        // step the current node cursor forward before replacing and returning
        // the previous index
        let prev = self.cursor_mut().next_node()?;
        // add the previous node index to the history
        self.add_node(prev.clone());
        // return the previous node index
        Ok(prev)
    }
    /// returns the total number of edges within the history
    pub fn size(&self) -> usize {
        self.edges().len()
    }
    /// returns the total number of nodes within the history
    pub fn order(&self) -> usize {
        self.nodes().len()
    }
}

#[allow(deprecated)]
#[doc(hidden)]
impl<Ix> IndexTracker<Ix>
where
    Ix: RawIndex,
{
    #[deprecated(since = "0.1.2", note = "use `add_edge` instead")]
    pub fn add_edge_index(&mut self, index: EdgeId<Ix>) -> &mut Self
    where
        Ix: Clone + Ord,
    {
        self.add_edge(index)
    }
    #[deprecated(since = "0.1.2", note = "use `add_node` instead")]
    pub fn add_vertex_index(&mut self, index: VertexId<Ix>) -> &mut Self
    where
        Ix: Clone + Ord,
    {
        self.add_node(index)
    }
    #[deprecated(since = "0.1.2", note = "use `remove_node` instead")]
    pub fn remove_vertex(&mut self, index: &Ix) -> &mut Self
    where
        Ix: Ord,
    {
        self.remove_node(index)
    }
    #[deprecated(since = "0.1.2", note = "use `next_edge` instead")]
    pub fn next_edge_index(&mut self) -> Result<EdgeId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + Ord,
    {
        self.next_edge()
    }
    #[deprecated(since = "0.1.2", note = "use `next_point` instead")]
    pub fn next_vertex_index(&mut self) -> Result<VertexId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + Ord,
    {
        self.next_vertex()
    }
}

/// counts each id, duplicates included
fn tally<'a, Ix>(ids: impl IntoIterator<Item = &'a Ix>) -> BTreeMap<Ix, usize>
where
    Ix: 'a + Clone + Ord,
{
    let mut counts = BTreeMap::new();
    ids.into_iter()
        .for_each(|id| *counts.entry(id.clone()).or_insert(0) += 1);
    counts
}

/// drops one occurrence of `id`, removing it once none remain
fn untally<Ix: Ord>(counts: &mut BTreeMap<Ix, usize>, id: &Ix) {
    if let Some(n) = counts.get_mut(id) {
        *n -= 1;
        if *n == 0 {
            counts.remove(id);
        }
    }
}

/// the serialized form of an [`IndexTracker`]; deserializing through it rebuilds the mirrors
#[cfg(feature = "serde")]
#[derive(Default, serde::Deserialize)]
#[serde(default, rename_all = "snake_case")]
struct RawTracker<Ix>
where
    Ix: RawIndex,
{
    cursor: IndexFrame<Ix>,
    edges: Vec<EdgeId<Ix>>,
    nodes: Vec<VertexId<Ix>>,
}

#[cfg(feature = "serde")]
impl<Ix> From<RawTracker<Ix>> for IndexTracker<Ix>
where
    Ix: RawIndex + Clone + Ord,
{
    fn from(
        RawTracker {
            cursor,
            edges,
            nodes,
        }: RawTracker<Ix>,
    ) -> Self {
        Self::from_cursor(cursor)
            .with_edges(edges)
            .with_nodes(nodes)
    }
}
