/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module implements the [`Error`] type for algorithms and operators for hypergraphs in
//! the [`rshyper`](https://docs.rs/rshyper) crate.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use rshyper::error::Error as CoreError;
use rshyper::idx::RawIndex;
/// a type alias for a [Result] with the crate-specific error type [`AlgoError`]
pub type Result<T = ()> = core::result::Result<T, Error>;

/// The [`Error`] type enumerates the various errors encountered by algorithms and operators on
/// hypergraphs
#[derive(Debug, strum::EnumIs, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error("Not Found: {0}")]
    NotFound(Box<dyn RawIndex>),
    #[error("No path found between the two points")]
    PathNotFound,
    /// the search supports only undirected graphs until directed semantics are decided
    /// (ADR-DIR, #118)
    #[error("unsupported graph kind `{0}`: only undirected graphs are supported")]
    UnsupportedGraphKind(&'static str),
    /// an edge weight is negative, NaN or not finite
    #[error("edge weights must be finite and non-negative")]
    InvalidWeight,
    /// a heuristic value is negative, NaN or not finite
    #[error("heuristic values must be finite and non-negative")]
    InvalidHeuristic,
    /// a path cost exceeded the largest value of the weight type
    #[error("the path cost overflowed the weight type")]
    CostOverflow,
    #[error(transparent)]
    CoreError(#[from] CoreError),
}

#[cfg(feature = "alloc")]
impl From<Error> for CoreError {
    fn from(e: Error) -> Self {
        match e {
            Error::CoreError(e) => e,
            _ => CoreError::BoxError(Box::new(e)),
        }
    }
}
