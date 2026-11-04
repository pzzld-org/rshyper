/*
    appellation: queue_node <module>
    authors: @FL03
*/
use core::cmp::Ordering;
use rshyper::idx::{RawIndex, VertexId};

/// A node in the priority queue for Dijkstra's algorithm.
///
/// Nodes compare by cost in *reverse* (both `Ord` and `PartialOrd`), so a
/// [`BinaryHeap`](alloc::collections::BinaryHeap) pops the cheapest node first.
#[derive(Copy, Clone, Debug, Default)]
pub struct QueueNode<Idx, T>
where
    Idx: RawIndex,
{
    pub(crate) cost: T,
    pub(crate) vertex: VertexId<Idx>,
}

impl<Idx, T> QueueNode<Idx, T>
where
    Idx: RawIndex,
{
    /// Creates a new [`QueueNode`] with the given cost and vertex.
    pub const fn new(cost: T, vertex: VertexId<Idx>) -> Self {
        Self { cost, vertex }
    }
    /// create a new node with the given cost and a default vertex
    pub fn from_cost(cost: T) -> Self
    where
        Idx: Default,
    {
        Self::new(cost, Default::default())
    }
    /// create a new node with the given vertex and a default cost
    pub fn from_vertex(vertex: VertexId<Idx>) -> Self
    where
        T: Default,
    {
        Self::new(Default::default(), vertex)
    }
    /// returns an immutable reference to the cost of the node
    pub const fn cost(&self) -> &T {
        &self.cost
    }
    /// returns a mutable reference to the cost of the node
    pub const fn cost_mut(&mut self) -> &mut T {
        &mut self.cost
    }
    /// returns an immutable reference to the vertex of the node
    pub const fn vertex(&self) -> &VertexId<Idx> {
        &self.vertex
    }
    /// returns a mutable reference to the vertex of the node
    pub const fn vertex_mut(&mut self) -> &mut VertexId<Idx> {
        &mut self.vertex
    }
    /// update the current cost and return a mutable reference to the node
    pub fn set_cost(&mut self, cost: T) -> &mut Self {
        *self.cost_mut() = cost;
        self
    }
    /// update the current vertex and return a mutable reference to the node
    pub fn set_vertex(&mut self, vertex: VertexId<Idx>) -> &mut Self {
        *self.vertex_mut() = vertex;
        self
    }
    /// consumes the current instance to create another with the given cost
    pub fn with_cost(self, cost: T) -> Self {
        Self { cost, ..self }
    }
    /// consumes the current instance to create another with the given vertex
    pub fn with_vertex(self, vertex: VertexId<Idx>) -> Self {
        Self { vertex, ..self }
    }
}

impl<Idx, T> Eq for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq,
{
}

impl<Idx, T> core::hash::Hash for QueueNode<Idx, T>
where
    Idx: RawIndex + Eq + core::hash::Hash,
    T: Eq + core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.cost().hash(state);
        self.vertex().hash(state);
    }
}

impl<Idx, T> Ord for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialOrd + PartialEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap
        other
            .cost()
            .partial_cmp(self.cost())
            .unwrap_or(Ordering::Equal)
    }
}

impl<Idx, T> PartialEq<QueueNode<Idx, T>> for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq,
{
    fn eq(&self, other: &QueueNode<Idx, T>) -> bool {
        self.cost() == other.cost() && self.vertex() == other.vertex()
    }
}

impl<'a, Idx, T> PartialEq<&'a QueueNode<Idx, T>> for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq,
{
    fn eq(&self, other: &&'a QueueNode<Idx, T>) -> bool {
        self.cost() == other.cost() && self.vertex() == other.vertex()
    }
}

impl<'a, Idx, T> PartialEq<&'a mut QueueNode<Idx, T>> for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq,
{
    fn eq(&self, other: &&'a mut QueueNode<Idx, T>) -> bool {
        self.cost() == other.cost() && self.vertex() == other.vertex()
    }
}

impl<Idx, T> PartialEq<QueueNode<Idx, T>> for &QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq,
{
    fn eq(&self, other: &QueueNode<Idx, T>) -> bool {
        self.cost() == other.cost() && self.vertex() == other.vertex()
    }
}

impl<Idx, T> PartialEq<QueueNode<Idx, T>> for &mut QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq,
{
    fn eq(&self, other: &QueueNode<Idx, T>) -> bool {
        self.cost() == other.cost() && self.vertex() == other.vertex()
    }
}

impl<Idx, T> PartialOrd<QueueNode<Idx, T>> for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &QueueNode<Idx, T>) -> Option<Ordering> {
        // must agree with `Ord`: `BinaryHeap` sifts with these operators
        Some(self.cmp(other))
    }
}

impl<'a, Idx, T> PartialOrd<&'a QueueNode<Idx, T>> for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &&'a QueueNode<Idx, T>) -> Option<Ordering> {
        other.cost().partial_cmp(self.cost())
    }
}

impl<'a, Idx, T> PartialOrd<&'a mut QueueNode<Idx, T>> for QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut QueueNode<Idx, T>) -> Option<Ordering> {
        other.cost().partial_cmp(self.cost())
    }
}

impl<Idx, T> PartialOrd<QueueNode<Idx, T>> for &QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &QueueNode<Idx, T>) -> Option<Ordering> {
        other.cost().partial_cmp(self.cost())
    }
}

impl<Idx, T> PartialOrd<QueueNode<Idx, T>> for &mut QueueNode<Idx, T>
where
    Idx: RawIndex + PartialEq,
    T: PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &QueueNode<Idx, T>) -> Option<Ordering> {
        other.cost().partial_cmp(self.cost())
    }
}
