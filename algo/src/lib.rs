//! Algorithms and operators for hypergraphs designed to work with the `rshyper` framewor.
//!
//! ## Features
//!
//! - [`astar`]: the A* search algorithm for hypergraphs
//! - [`breadth_first`]: the breadth-first search algorithm for hypergraphs
//! - [`depth_first`]: the depth-first search algorithm for hypergraphs
//! - [`dijkstra`]: Dijkstra's algorithm for finding the shortest path in hypergraphs
//!
#![crate_name = "rshyper_algo"]
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/FL03/rshyper/main/.artifacts/assets/logo.svg"
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::non_canonical_clone_impl,
    clippy::non_canonical_partial_ord_impl,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
// compile check
#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! { "either the `alloc` or `std` feature must be enabled for the rshyper crate" }
// extenral crates
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate rshyper_core as rshyper;
// macros
#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}
// modules
pub mod error;
#[cfg(feature = "alloc")]
pub mod search;

mod traits {
    #[doc(inline)]
    pub use self::{operators::*, path::*, traverse::*};

    mod operators;
    mod path;
    mod traverse;
}

mod types {
    #[doc(inline)]
    pub use self::{priority_node::*, queue_node::*};

    mod priority_node;
    mod queue_node;
}
// re-exports
#[cfg(feature = "alloc")]
pub use self::search::{
    AStarSearch, BreadthFirstTraversal, DepthFirstTraversal, Dijkstra, Heuristic, PathLimits,
    Search, Walk, Walks,
};
#[doc(inline)]
pub use self::{error::*, traits::*, types::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::traits::*;
    pub use crate::types::*;

    #[cfg(feature = "alloc")]
    pub use crate::search::prelude::*;
}
