/*
    appellation: aliases <module>
    authors: @FL03
*/
//! this module provides various type aliases hyperedges such as: [`DiEdge`], [`UnEdge`] and
//! [`HashEdge`]

#[cfg(feature = "alloc")]
pub use self::use_alloc::*;

use crate::edge::{HyperEdge, Link};
use crate::idx::{VertexArray, VertexSet, VertexSlice, VertexSliceMut, VertexSliceRef};
use crate::{Directed, Undirected};

/// a type alias for a [`Directed`] relationship
pub type DiLink<S, Ix> = Link<S, Directed, Ix>;
/// a type alias for an [`Undirected`] relationship
pub type UnLink<S, Ix> = Link<S, Undirected, Ix>;

/// a type alias for a [`Link`] whose _vertices_ are stored in a [`VertexSet`]
pub type HashLink<K, I, S> = Link<VertexSet<I, S>, K, I>;

/// a type alias for a [`Link`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type LinkArray<K, Ix, const N: usize> = Link<VertexArray<Ix, N>, K, Ix>;
/// a type alias for a [`Link`] whose _vertices_ are stored in a slice
pub type LinkSlice<K, Ix> = Link<VertexSlice<Ix>, K, Ix>;
/// a type alias for a [`Link`] with a reference domain representation of [`VertexSliceRef`]
pub type LinkSliceRef<'a, K, Ix> = Link<VertexSliceRef<'a, Ix>, K, Ix>;
/// a type alias for a [`Link`] with a mutable domain representation of [`VertexSliceMut`]
pub type LinkSliceMut<'a, K, Ix> = Link<VertexSliceMut<'a, Ix>, K, Ix>;

/// a type alias for a [`Directed`] hyperedge
pub type DiEdge<T, S, Idx = usize> = HyperEdge<T, S, Directed, Idx>;
/// a type alias for an [`Undirected`] hyperedge
pub type UnEdge<T, S, Idx = usize> = HyperEdge<T, S, Undirected, Idx>;

/// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a [`VertexSet`]
pub type HashEdge<T, K, Ix, S> = HyperEdge<T, VertexSet<Ix, S>, K, Ix>;

/// a type alias for a [`HyperEdge`] whose _vertices_ are stored in an array of fixed
/// size with the size defined by the generic parameter `N`.
pub type EdgeArray<T, const N: usize, K, Ix = usize> = HyperEdge<T, VertexArray<Ix, N>, K, Ix>;
/// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a slice
pub type EdgeSlice<T, K, Ix = usize> = HyperEdge<T, VertexSlice<Ix>, K, Ix>;
/// a type alias for a [`HyperEdge`] whose _vertices_ are stored in an owned slice
pub type EdgeSliceRef<'a, T, K, Ix = usize> = HyperEdge<T, VertexSliceRef<'a, Ix>, K, Ix>;
/// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a mutable slice
pub type EdgeSliceMut<'a, T, K, Ix = usize> = HyperEdge<T, VertexSliceMut<'a, Ix>, K, Ix>;

#[cfg(feature = "alloc")]
mod use_alloc {
    use crate::edge::{HyperEdge, Link};
    use crate::idx::{VertexBSet, VertexDeque, VertexVec};

    /// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a [`Vec`]
    pub type EdgeVec<T, K, Ix> = HyperEdge<T, VertexVec<Ix>, K, Ix>;
    /// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a [`VecDeque`](alloc::collections::VecDeque)
    pub type EdgeDeque<T, K, Ix> = HyperEdge<T, VertexDeque<Ix>, K, Ix>;
    /// a type alias for a [`HyperEdge`] whose _vertices_ are stored in a [`BTreeSet`](alloc::collections::BTreeSet)
    pub type EdgeBSet<T, K, Ix> = HyperEdge<T, VertexBSet<Ix>, K, Ix>;

    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`Vec`]
    pub type LinkVec<K, Ix> = Link<VertexVec<Ix>, K, Ix>;
    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`VecDeque`](alloc::collections::VecDeque)
    pub type LinkDeque<K, Ix> = Link<VertexDeque<Ix>, K, Ix>;
    /// a type alias for an [`Link`] whose _vertices_ are stored in a [`BTreeSet`](alloc::collections::BTreeSet)
    pub type LinkBSet<K, Ix> = Link<VertexBSet<Ix>, K, Ix>;
}
