/*
    appellation: tracker <module>
    authors: @FL03
*/
use super::{EdgeId, IndexFrame, RawIndex, VertexId};
use crate::AddStep;
use crate::idx::error::{IndexError, IndexResult};
use alloc::vec::Vec;

/// the [`IndexTracker`] keeps a history of the created indices and the current cursor position.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct IndexTracker<Ix = usize>
where
    Ix: RawIndex,
{
    pub(crate) cursor: IndexFrame<Ix>,
    pub(crate) edges: Vec<EdgeId<Ix>>,
    pub(crate) nodes: Vec<VertexId<Ix>>,
    /// cached upper bounds of the histories; not part of equality, hashing or serde
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) marks: Marks<Ix>,
}

/// The largest id recorded in each history, so allocation can skip the history scan while
/// the cursor stays above it. A bound is an upper bound (removals keep it valid); `*_known`
/// is false after any raw history access, and the next allocation recomputes it once.
#[derive(Clone, Debug)]
pub(crate) struct Marks<Ix> {
    edge: Option<Ix>,
    node: Option<Ix>,
    edge_known: bool,
    node_known: bool,
}

impl<Ix> Marks<Ix> {
    const fn empty() -> Self {
        Self {
            edge: None,
            node: None,
            edge_known: true,
            node_known: true,
        }
    }
}

impl<Ix> Default for Marks<Ix> {
    /// unknown, so a deserialized or defaulted tracker recomputes before trusting it
    fn default() -> Self {
        Self {
            edge: None,
            node: None,
            edge_known: false,
            node_known: false,
        }
    }
}

impl<Ix> PartialEq for Marks<Ix> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<Ix> Eq for Marks<Ix> {}

impl<Ix> PartialOrd for Marks<Ix> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Ix> Ord for Marks<Ix> {
    fn cmp(&self, _: &Self) -> core::cmp::Ordering {
        core::cmp::Ordering::Equal
    }
}

impl<Ix> core::hash::Hash for Marks<Ix> {
    fn hash<H: core::hash::Hasher>(&self, _: &mut H) {}
}

/// allocate `cursor`'s current id, rejecting it if `history` already holds it. Nothing
/// changes on error. O(1) while the cursor is above the cached bound, O(history) otherwise.
fn allocate<Ix, K>(
    cursor: &mut crate::idx::IndexBase<Ix, K>,
    history: &mut Vec<crate::idx::IndexBase<Ix, K>>,
    bound: &mut Option<Ix>,
    known: &mut bool,
) -> IndexResult<crate::idx::IndexBase<Ix, K>>
where
    Ix: RawIndex + AddStep<Output = Ix> + Clone + PartialOrd,
    K: crate::idx::IndexType,
{
    if !*known {
        *bound = history
            .iter()
            .map(|id| id.get())
            .fold(None::<&Ix>, |acc, x| match acc {
                Some(m) if x > m => Some(x),
                Some(m) => Some(m),
                None => Some(x),
            })
            .cloned();
        *known = true;
    }
    let id = cursor.get();
    let fresh = match bound.as_ref() {
        Some(max) => id > max,
        None => true,
    };
    if !fresh && history.iter().any(|h| h.get() == id) {
        return Err(IndexError::DuplicateIndex);
    }
    // checked: an exhausted index space errors instead of wrapping onto old ids
    let prev = cursor.step()?;
    if fresh {
        *bound = Some(prev.get().clone());
    }
    history.push(prev.clone());
    Ok(prev)
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
            marks: Marks::empty(),
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
    /// returns a mutable reference to the edge history
    ///
    /// Edits made through it are not checked; the next allocation rescans the history once.
    pub const fn edges_mut(&mut self) -> &mut Vec<EdgeId<Ix>> {
        self.marks.edge_known = false;
        &mut self.edges
    }
    /// returns an immutable reference to the node history
    pub const fn nodes(&self) -> &Vec<VertexId<Ix>> {
        &self.nodes
    }
    /// returns a mutable reference to the history of created nodes
    ///
    /// Edits made through it are not checked; the next allocation rescans the history once.
    pub const fn nodes_mut(&mut self) -> &mut Vec<VertexId<Ix>> {
        self.marks.node_known = false;
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
    pub fn set_edges(&mut self, edges: Vec<EdgeId<Ix>>) -> &mut Self {
        *self.edges_mut() = edges;
        self
    }
    /// overwrite the node history and return a mutable reference to the tracker
    #[inline]
    pub fn set_nodes(&mut self, nodes: Vec<VertexId<Ix>>) -> &mut Self {
        *self.nodes_mut() = nodes;
        self
    }
    /// consumes the current instance to create another with the given position
    #[inline]
    pub fn with_cursor(self, cursor: IndexFrame<Ix>) -> Self {
        Self { cursor, ..self }
    }
    /// consumes the current instance to create another with the given edge history
    #[inline]
    pub fn with_edges(mut self, edges: Vec<EdgeId<Ix>>) -> Self {
        self.marks.edge_known = false;
        Self { edges, ..self }
    }
    /// consumes the current instance to create another with the given node history
    #[inline]
    pub fn with_nodes(mut self, nodes: Vec<VertexId<Ix>>) -> Self {
        self.marks.node_known = false;
        Self { nodes, ..self }
    }
    /// add a new edge index to the history.
    #[inline]
    pub fn add_edge(&mut self, index: EdgeId<Ix>) -> &mut Self {
        self.edges_mut().push(index);
        self
    }
    /// add a new node index to the history.
    #[inline]
    pub fn add_node(&mut self, index: VertexId<Ix>) -> &mut Self {
        self.nodes_mut().push(index);
        self
    }
    /// clears the recorded edges and nodes before resetting the cursor to the initial position
    pub fn clear(&mut self) -> &mut Self
    where
        Ix: Default,
    {
        // clear the nodes and edges
        self.edges.clear();
        self.nodes.clear();
        self.marks = Marks::empty();
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
    /// returns a mutable reference to the edge id
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
    /// returns a mutable reference to the node id
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
        Ix: PartialEq,
    {
        self.retain_edges(|i| i != index);
        self
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn remove_node(&mut self, index: &Ix) -> &mut Self
    where
        Ix: PartialEq,
    {
        self.retain_nodes(|i| i != index);
        self
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn retain_edges<F>(&mut self, f: F)
    where
        F: FnMut(&EdgeId<Ix>) -> bool,
    {
        // removal keeps the cached bound an upper bound
        self.edges.retain(f);
    }
    /// retains only nodes satisfying the predicate `f`.
    #[inline]
    pub fn retain_nodes<F>(&mut self, f: F)
    where
        F: FnMut(&VertexId<Ix>) -> bool,
    {
        // removal keeps the cached bound an upper bound
        self.nodes.retain(f);
    }
    /// allocates the cursor's edge id: records it in the history, steps the cursor and
    /// returns the id.
    ///
    /// Amortized O(1) while the cursor stays above every recorded id; after the cursor is
    /// moved back (or the history is edited directly) each call scans the history.
    ///
    /// # Errors
    ///
    /// - [`IndexError::DuplicateIndex`] if the history already holds the cursor's id.
    /// - [`IndexError::IndexOutOfBounds`] if stepping the cursor would overflow `Ix`.
    ///
    /// On error the cursor and the history are unchanged.
    #[inline]
    pub fn next_edge(&mut self) -> IndexResult<EdgeId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Clone + PartialOrd,
    {
        allocate(
            &mut self.cursor.edge,
            &mut self.edges,
            &mut self.marks.edge,
            &mut self.marks.edge_known,
        )
    }
    /// allocates the cursor's vertex id; see [`next_edge`](IndexTracker::next_edge) for the
    /// cost and the errors.
    #[inline]
    pub fn next_vertex(&mut self) -> IndexResult<VertexId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Clone + PartialOrd,
    {
        allocate(
            &mut self.cursor.node,
            &mut self.nodes,
            &mut self.marks.node,
            &mut self.marks.node_known,
        )
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
    pub fn add_edge_index(&mut self, index: EdgeId<Ix>) -> &mut Self {
        self.add_edge(index)
    }
    #[deprecated(since = "0.1.2", note = "use `add_node` instead")]
    pub fn add_vertex_index(&mut self, index: VertexId<Ix>) -> &mut Self {
        self.add_node(index)
    }
    #[deprecated(since = "0.1.2", note = "use `remove_node` instead")]
    pub fn remove_vertex(&mut self, index: &Ix) -> &mut Self
    where
        Ix: PartialEq,
    {
        self.remove_node(index)
    }
    #[deprecated(since = "0.1.2", note = "use `next_edge` instead")]
    pub fn next_edge_index(&mut self) -> Result<EdgeId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
    {
        self.next_edge()
    }
    #[deprecated(since = "0.1.2", note = "use `next_point` instead")]
    pub fn next_vertex_index(&mut self) -> Result<VertexId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
    {
        self.next_vertex()
    }
}
