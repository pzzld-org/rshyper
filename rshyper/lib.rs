#![crate_name = "rshyper"]
#![crate_type = "lib"]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pzzld-org/rshyper/main/.artifacts/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/pzzld-org/rshyper/main/.artifacts/assets/logo.svg"
)]
//! # rshyper
//!
//! [![crates.io](https://img.shields.io/crates/v/rshyper?style=for-the-badge&logo=rust)](https://crates.io/crates/rshyper)
//! [![docs.rs](https://img.shields.io/docsrs/rshyper?style=for-the-badge&logo=docs.rs)](https://docs.rs/rshyper)
//! [![GitHub License](https://img.shields.io/github/license/pzzld-org/rshyper?style=for-the-badge&logo=github)](https://github.com/pzzld-org/rshyper/blob/main/LICENSE)
//!
//! ***
//!
//! Welcome to the `rshyper` crate - a Rust package providing a comprehensive framework for
//! creating, manipulating, and analyzing hypergraphs using a myriad of mathematical and
//! algorithmic techniques. The crate is designed to be flexible and modular enabled via heavy
//! feature-gating throughout the framework.
//!
//! ## Background
//!
//! Before diving in to the technical side of things, let's start by defining several terms
//! commonly used in the definition and implementation of hypergraphs.
//!
//! ### Terms
//!
//! - **edge:** a hyperedge is a generalization of an edge in a graph, allowing it to connect
//!   any number of vertices.
//! - **link:** here, a link defines the _layout_ of an edge providing a way to connect
//!   vertices together.
//! - **node** a node is a complete _vertex_ in that it is considered to be weighted.
//! - **point:** here, a point is a synonym for a vertex, and is used to define the position of
//!   a vertex within the hypergraph.
//! - **surface:** a surface is a synonym for an edge, often used here to describe iterators
//!   directly yielding (mutable) references to the "Edge Values" of the hypergraph.
//! - **vertex:** a vertex is an _unweighted_ node defining a point within the hypergraph.
//!
//! ### Hypergraphs
//!
//! A hypergraph is an abstraction of a graph that allows edges to connect any number of
//! vertices. This flexible data-strcture is highly mathematical, yet, extremely useful in
//! many applications such as database design, network analysis, combinatorial optimization,
//! modeling topological spaces, and more.
//!
//! #### _Definition 1:_
//! Formally, a directed hypergraph is a pair `H = (V,E)` where `V` is a set of vertices and
//! `E` is a set of hyperedges. Each hyperedge `e ∈ E` is a subset of `V` that can contain
//!
//! ## Features
//!
//! - `hyper_map`: enables the [`HyperMap`] implementation, a hash-based hypergraph structure
//! - `macros`: enables the implemented macros for streamlining graph management
//!
//! ### _Dependencies_
//!
//! **Note:** While the `alloc` and `std` libraries are feature-gated, they are required for
//! anything useful in this crate; both are enabled by default.
//!
//! - `rayon`: enables parallel processing capabilities using the `rayon` crate
//! - `serde`: enables serialization and deserialization of hypergraphs using the `serde` crate
//!
//! ## Examples
//!
//! For more detailed examples, please refer to the [examples directory](https://github.com/pzzld-org/rshyper/blob/main/rshyper/examples).
//!
//! ### _Example 1: Basic Usage_
//!
//! ```rust
//! use rshyper::{HyperMap, Weight};
//!
//! let mut graph = HyperMap::<usize, usize>::undirected();
//! // add some unweighted vertices
//! let v0 = graph.add_vertex().expect("failed to add vertex");
//! let v1 = graph.add_vertex().expect("failed to add vertex");
//! // add a weighted node
//! let v2 = graph.add_node(Weight(10)).expect("failed to add node");
//! // create some edges using those nodes
//! let e0 = graph.add_link([v0, v1]).expect("failed to add edge");
//! let e1 = graph.add_link([v1, v2]).expect("failed to add edge");
//! // create a surface (weighted edge) using the nodes
//! let e3 = graph.add_edge([v0, v2], Weight(5)).expect("failed to add surface");
//! ```
//!
//! ### _Example 2: Macros_
//!
//! With the `macros` feature enabled, the [`hypergraph!`], [`hyperedge!`], and
//! [`hypernode!`] macros allow nodes and edges to be defined in a more declarative way.
//!
//! #### `hypergraph!`
//!
//! ```rust
//! use rshyper::{hypergraph, UnHyperMap};
//! // initialize a new undirected hypergraph
//! let mut graph = UnHyperMap::<usize, usize>::undirected();
//! // use the macro to insert nodes and edges into the graph
//! hypergraph! {
//!     graph {
//!         nodes: {
//!             let v0;
//!             let v1;
//!             let v2 = 10;
//!         };
//!         edges: {
//!             let e0: [v0, v1];
//!             let e1: [v0, v1, v2] = 10;
//!         };
//!     }
//! }
//! ```
//!
//! or
//!
//! ```rust
//! use rshyper::{hypergraph, UnHyperMap};
//!
//! // use the macro to initialize a new hypergraph, then insert nodes and edges
//! hypergraph! {
//!     let mut graph: UnHyperMap::<usize, usize> {
//!         nodes: {
//!             let v0;
//!             let v1 = 1;
//!             let v2;
//!         };
//!         edges: {
//!             let e0: [v0, v1];
//!             let e1: [v0, v2];
//!             let e2: [v0, v1, v2] = 10;
//!         };
//!     }
//! }
//! ```
//!
//! #### `hyperedge!`
//!
//! ```rust
//! use rshyper::{HyperMap, IntoWeight};
//!
//! fn main() -> rshyper::Result<()> {
//!     // initialize a new undirected hypergraph
//!     let mut graph = HyperMap::<usize, usize>::undirected();
//!     // add some vertices
//!     let v0 = graph.add_node(1.into_weight())?;
//!     let v1 = graph.add_node(2.into_weight())?;
//!     let v2 = graph.add_node(3.into_weight())?;
//!     // use the macro to insert edges into the graph
//!     rshyper::hyperedge! {
//!         graph {
//!             let e0: [v0, v1];
//!             let e1: [v0, v1, v2] = 10;
//!         }
//!     }
//!     // verify the order of the edges
//!     assert_eq!(graph.get_edge_order(&e0)?, 2);
//!     assert_eq!(graph.get_edge_order(&e1)?, 3);
//!     // verify the weights of the edges
//!     assert_eq!(graph.get_edge_weight(&e0)?, &<usize>::default());
//!     assert_eq!(graph.get_edge_weight(&e1)?, &10);
//!     // finish
//!     Ok(())
//! }
//! ```
//!
//! #### `hypernode!`
//!
//! ```rust
//! use rshyper::HyperMap;
//! // initialize a new, undirected hypergraph
//! let mut graph = HyperMap::<usize, usize>::undirected();
//! // use the macro to insert nodes into the graph
//! rshyper::hypernode! {
//!     graph {
//!         let v0;
//!         let v1;
//!         let v2;
//!         let v3 = 1;
//!         let v4 = 2;
//!         let v5 = 3;
//!     }
//! }
//! ```
//!
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
// compile check
#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! { "either the `alloc` or `std` feature must be enabled for the rshyper crate" }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// external modules
#[doc(inline)]
#[cfg(feature = "algo")]
/// the `algo` module focuses on implementing algorithms and operators for hypergraphs
pub use rshyper_algo as algo;
#[doc(inline)]
#[cfg(feature = "hyper_map")]
/// a map-based implementation of the hypergraph
pub use rshyper_hmap as hyper_map;
// re-exports
#[doc(inline)]
#[cfg(feature = "hyper_map")]
pub use self::hyper_map::{DiHyperMap, HyperMap, UnHyperMap};
#[doc(inline)]
pub use rshyper_core::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use rshyper_core::prelude::*;

    #[cfg(feature = "algo")]
    pub use rshyper_algo::prelude::*;
    #[cfg(feature = "hyper_map")]
    pub use rshyper_hmap::prelude::*;
}
