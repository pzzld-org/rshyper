/*
    Appellation: error <module>
    Contrib: @FL03
*/
use super::{EdgeId, RawIndex, VertexId};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// A type alias for a [`Result`] with an error type of [`IndexError`]
pub type IndexResult<T = ()> = core::result::Result<T, IndexError>;

/// The error type for index-related operations
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("Duplicate index")]
    DuplicateIndex,
    #[error("the index is out of bounds")]
    IndexOutOfBounds,
    #[cfg(feature = "alloc")]
    #[error("invalid index: {0}")]
    InvalidIndex(Box<dyn RawIndex>),
    #[cfg(feature = "alloc")]
    #[error("No index found for {0}")]
    IndexNotFound(Box<dyn RawIndex>),
    #[cfg(feature = "alloc")]
    #[error("No path found from {from} to {to}")]
    NoPathFoundBetween {
        from: Box<dyn RawIndex>,
        to: Box<dyn RawIndex>,
    },
    #[cfg(feature = "alloc")]
    #[error("Hyperedge {0} does not exist")]
    EdgeDoesNotExist(Box<dyn RawIndex>),
    #[cfg(feature = "alloc")]
    #[error("Vertex {0} does not exist")]
    NodeDoesNotExist(Box<dyn RawIndex>),
}

impl IndexError {
    /// initialize a new [`DuplicateIndex`](IndexError::DuplicateIndex) error variant
    pub fn duplicate_index() -> Self {
        IndexError::DuplicateIndex
    }
    /// initialize a new [`IndexOutOfBounds`](IndexError::IndexOutOfBounds) error variant
    pub fn index_out_of_bounds() -> Self {
        IndexError::IndexOutOfBounds
    }
}

#[cfg(feature = "alloc")]
impl IndexError {
    /// initialize a new [`InvalidIndex`](IndexError::InvalidIndex) error variant
    pub fn invalid_index<Ix: RawIndex>(value: Ix) -> Self {
        IndexError::InvalidIndex(Box::new(value))
    }
    /// initialize a new [`IndexNotFound`](IndexError::IndexNotFound) error variant using the
    /// raw inner value of some index.
    pub fn index_not_found<Idx: RawIndex>(value: Idx) -> Self {
        IndexError::IndexNotFound(Box::new(value))
    }
    /// initialize a new [`NoPathFoundBetween`](IndexError::NoPathFoundBetween) error variant
    pub fn no_path_found_between<Idx: RawIndex>(from: Idx, to: Idx) -> Self {
        IndexError::NoPathFoundBetween {
            from: Box::new(from),
            to: Box::new(to),
        }
    }
    /// initialize a new [`EdgeDoesNotExist`](IndexError::EdgeDoesNotExist) error variant
    pub fn hyperedge_does_not_exist<Idx: RawIndex>(index: EdgeId<Idx>) -> Self {
        IndexError::EdgeDoesNotExist(Box::new(index.value()))
    }
    /// initialize a new [`NodeDoesNotExist`](IndexError::NodeDoesNotExist) error variant
    pub fn vertex_does_not_exist<Idx: RawIndex>(index: VertexId<Idx>) -> Self {
        IndexError::NodeDoesNotExist(index.into_boxed_value())
    }
}
