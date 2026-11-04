/*
    appellation: kinds <module>
    authors: @FL03
*/
//! this module defines the [`GraphIndex`] trait and the [`IndexKind`] enumeration, which are
//! used to represent various kinds of indices in a hypergraph. Additionally, the module
//! provides two specific implemenations of the `GraphIndex` trait: [`EdgeIndex`] and
//! [`VertexIndex`], each of which are used to represent edge and vertex indices, respectively.
//! The `IndexKind` enumeration is used primarily:
//!
//! - during the initialization process
//! - enables dynamic dispatching of indices
//! - allows for maps and other data structures to be keyed by index kind
use crate::idx::{IndexBase, RawIndex};

/// This trait is used to define various _kinds_ of indices that are used to compose graphical
/// structures.
pub trait IndexType
where
    Self: Clone
        + Copy
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash,
{
    private!();
}
/// the [`Grid`] enumerate the possible variations of indicies that can be used within the
/// scope of a hypergraph. This entity is useful for enabling "compoite" collections of
/// indicies for iteration, traversal, etc.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    strum::EnumCount,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum_discriminants(
    name(IndexKind),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantArray,
        strum::VariantNames,
    ),
    strum(serialize_all = "snake_case")
)]
pub enum Grid<Idx>
where
    Idx: RawIndex,
{
    #[cfg_attr(
        feature = "serde",
        strum_discriminants(serde(alias = "hyperedge", alias = "facet", alias = "surface"))
    )]
    Edge(IndexBase<Idx, EdgeIndex>),
    #[cfg_attr(
        feature = "serde",
        strum_discriminants(serde(
            alias = "hypernode",
            alias = "node",
            alias = "point",
            alias = "vertex"
        ))
    )]
    Vertex(IndexBase<Idx, VertexIndex>),
    Raw(Idx),
}

macro_rules! impl_type_kind {
    (@def $(#[$meta:meta])* $vis:vis enum $kind:ident) => {
        $(#[$meta])*
        $vis enum $kind {};
    };
    (@def $(#[$meta:meta])* $vis:vis struct $kind:ident) => {
        $(#[$meta])*
        #[derive(Default)]
        $vis struct $kind;
    };
    (@impl $(#[$meta:meta])* $vis:vis $i:ident $kind:ident($name:ident)) => {
        // create the implementation for the kind
        impl_type_kind! { @def $(#[$meta])*
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[repr(transparent)]
            $vis $i $kind
        }

        impl $kind {
            /// get the name of the kind as a string slice
            pub fn name(&self) -> &str {
                stringify!($name)
            }
        }
        // implement the necessary traits for the kind
        unsafe impl ::core::marker::Send for $kind {}

        unsafe impl ::core::marker::Sync for $kind {}

        impl IndexType for $kind { seal! {} }

        impl AsRef<str> for $kind {
            fn as_ref(&self) -> &str {
                self.name()
            }
        }
        // stringify the kind and implement Display
        impl ::core::fmt::Display for $kind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
    ($($(#[$meta:meta])* $vis:vis $i:ident $kind:ident($name:ident));* $(;)?) => {
        $(impl_type_kind! { @impl $(#[$meta])* $vis $i $kind($name) })*
    };
}

impl_type_kind! {
    #[doc = "A kind of index for edges in a graph"]
    pub struct EdgeIndex(edge);
    #[doc = "A kind of index for vertices in a graph"]
    pub struct VertexIndex(vertex);
}

impl IndexType for IndexKind {
    seal! {}
}
