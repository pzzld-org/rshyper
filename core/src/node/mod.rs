/*
    appellation: node <module>
    authors: @FL03
*/
//! this module provides the [`Node`] implmenetation alongisde the several traits such as
//! [`RawNode`] and [`RawPoint`] focused on establishing a common, well-defined interface for
//! weighted and unweighted "points" within a hypergraph.
#[doc(inline)]
pub use self::prelude::*;

/// this module defines the [`Node`] type
mod hyper_node;

mod impls {
    mod impl_hyper_node;
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::hyper_node::*;
    #[doc(inline)]
    pub use super::{HyperNode, HyperPoint, RawNode, RawPoint};
}

use crate::Weight;
use crate::idx::{RawIndex, VertexId};

/// [`RawNode`] is a trait that defines the behavior of a node in a hypergraph.
pub trait RawNode<T> {
    type Key: RawIndex;

    private!();
}
/// The [`HyperNode`] trait extends the [`RawNode`] trait to provide additional functionality
/// for nodes in a hypergraph, such as accessing the index and weight of the node.
pub trait HyperNode<T>: RawNode<T> {
    /// returns an immutable reference to the node index
    fn index(&self) -> &VertexId<Self::Key>;
    /// returns an immutable reference to the node data
    fn weight(&self) -> &Weight<T>;
    /// returns a mutable reference to the node data
    fn weight_mut(&mut self) -> &mut Weight<T>;
    /// [`replace`](core::mem::replace) the weight of the node with a new one, returning the
    /// previous value
    fn replace_weight(&mut self, weight: Weight<T>) -> Weight<T> {
        core::mem::replace(self.weight_mut(), weight)
    }
    /// overwrites the weight of the node with a new one and returns a mutable reference to
    /// the edge.
    fn set_weight(&mut self, weight: T) -> &mut Self {
        self.weight_mut().set(weight);
        self
    }
    /// [`swap`](core::mem::swap) the weight of the node with another weight
    fn swap_weight(&mut self, weight: &mut Weight<T>) {
        core::mem::swap(self.weight_mut(), weight)
    }
    /// [`take`](core::mem::take) the weight of the node, replacing it with a default value
    fn take_weight(&mut self) -> Weight<T>
    where
        T: Default,
    {
        core::mem::take(self.weight_mut())
    }
}
/// A [`RawPoint`] is used to defines the base representation of any given point within a
/// hypergraph.
pub trait RawPoint {
    type Key: RawIndex;

    private!();
}
/// [`HyperPoint`] is a trait that extends the [`RawPoint`] trait to provide additional
/// functionality for points in a hypergraph, such as accessing the index and raw index.
pub trait HyperPoint: RawPoint {
    /// returns the index of the point as a [`VertexId`].
    fn index(&self) -> &VertexId<Self::Key>;
    /// returns the raw index of the point as a reference to the underlying key type.
    fn raw_index(&self) -> &Self::Key {
        self.index().get()
    }
}

/*
 ************* Implementations *************
*/
use crate::idx::IndexBase;

impl<T, Idx> RawNode<T> for Node<T, Idx>
where
    Idx: RawIndex,
{
    type Key = Idx;

    seal!();
}

impl<T, Idx> HyperNode<T> for Node<T, Idx>
where
    Idx: RawIndex,
{
    fn index(&self) -> &VertexId<Idx> {
        &self.id
    }
    fn weight(&self) -> &Weight<T> {
        self.weight()
    }
    fn weight_mut(&mut self) -> &mut Weight<T> {
        self.weight_mut()
    }
}

impl<K, Id> RawPoint for IndexBase<Id, K>
where
    Id: RawIndex,
{
    type Key = Id;

    seal!();
}

impl<Id> HyperPoint for VertexId<Id>
where
    Id: RawIndex,
{
    fn index(&self) -> &VertexId<Self::Key> {
        self
    }
}

impl<T, Id> RawPoint for Node<T, Id>
where
    Id: RawIndex,
{
    type Key = Id;

    seal!();
}

impl<T, Id: RawIndex> HyperPoint for Node<T, Id>
where
    Id: RawIndex,
{
    fn index(&self) -> &VertexId<Self::Key> {
        self.id()
    }
}
