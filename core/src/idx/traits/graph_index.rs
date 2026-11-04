/*
    appellation: indexed <module>
    authors: @FL03
*/
use super::RawIndex;

/// [`GraphIndex`] describes a common interface for all types which are aware of some associated
/// index. The trait is generic over a type `T` which implements the [`RawIndex`] trait,
/// allowing for flexibility in the type of index used while ensuring that the index type is
/// compatible with the hypergraph's indexing system.
pub trait GraphIndex<T: RawIndex> {
    type Idx<I>;

    /// Returns the index of the node.
    fn index(&self) -> &Self::Idx<T>;
}

/*
 ************* Implementations *************
*/
use crate::idx::VertexId;

impl<T: RawIndex> GraphIndex<T> for VertexId<T> {
    type Idx<I> = VertexId<I>;

    fn index(&self) -> &Self::Idx<T> {
        self
    }
}

impl<T, Idx> GraphIndex<Idx> for crate::Node<T, Idx>
where
    Idx: RawIndex,
{
    type Idx<I> = VertexId<I>;

    fn index(&self) -> &Self::Idx<Idx> {
        &self.id
    }
}
