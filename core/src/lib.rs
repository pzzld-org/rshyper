//! This crate provides the core functionality for the rshyper library, implementing various
//! primitives and utilities for working with hypergraphs.
//!
//! ## Components
//!
//! - [`attrs`]: Contains the [`Attrs`] and [`GraphProps`] types for managing graph attributes.
//! - [`edge`]: implements the [`HyperEdge`] and [`Link`] types for representing hyperedges
//! - [`node`]: provides the [`Node`] implementation for representing hypernodes
//! - [`weight`]: gives the [`Weight`] type for representing weights in a hypergraph
//!
#![crate_name = "rshyper_core"]
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
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
// compile check
#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! { "either the `alloc` or `std` feature must be enabled for the rshyper crate" }
// extenral crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod index;
    #[macro_use]
    pub(crate) mod hypergraph;
    #[macro_use]
    pub(crate) mod seal;
}
// modules
pub mod attrs;
pub mod edge;
pub mod error;
pub mod idx;
pub mod node;
pub mod weight;

pub mod traits {
    //! this module contains various traits used throughout to establish common interfaces and
    //! behaviors
    #[doc(inline)]
    pub use self::{
        contains::*, domain::*, hyper_graph::*, merge::*, step::*, transform::*, weighted::*,
    };
    /// the [`Contains`] trait provides a way to check if a graph contains a specific component
    mod contains;
    /// this module implements the [`RawDomain`] trait for defining the type of collection used
    /// to compose the hyperedge
    mod domain;
    /// the [`HyperGraph`] trait defines the core interface for hypergraphs, enabling the
    /// generalization of algorithms constructors, and graphical operators.
    #[cfg(feature = "alloc")]
    mod hyper_graph;
    /// the [`Merge`] trait provides a way to combine two graphs into one
    mod merge;
    /// this module defines sequential step generators
    mod step;
    /// traits for transformative operations on hypergraphs, such as mapping, are implemented
    /// within this module
    mod transform;
    /// the [`Weighted`] trait provides common behaviors for types that have an associated weight
    mod weighted;
}

pub mod types {
    //! this module provides various types used throughout the library
    #[doc(inline)]
    pub use self::graph_kind::*;
    /// this module defines the two types of graph kinds: [`Directed`] and [`Undirected`]
    mod graph_kind;
}
// re-exports
#[doc(inline)]
pub use self::{
    attrs::{Attrs, GraphProps},
    edge::{HyperEdge, Link, RawEdge},
    error::{Error, Result},
    idx::prelude::*,
    node::Node,
    traits::*,
    types::*,
    weight::{IntoWeight, Weight, Weightless},
};
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::{hyperedge, hypergraph, hypernode};

    pub use crate::attrs::prelude::*;
    pub use crate::edge::prelude::*;
    pub use crate::idx::prelude::*;
    pub use crate::node::prelude::*;
    pub use crate::traits::*;
    pub use crate::types::*;
    pub use crate::weight::*;
}
