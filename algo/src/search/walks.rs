/*
    Appellation: walks <module>
    Contrib: @FL03
*/
//! the bounds and results of [`BreadthFirstTraversal::shortest_walks`](crate::search::BreadthFirstTraversal::shortest_walks)
use alloc::vec::Vec;
use rshyper_core::idx::{EdgeId, VertexId};

/// the bounds of a [`shortest_walks`](crate::search::BreadthFirstTraversal::shortest_walks)
/// search. **Unstable** until ADR-PATH (#117) and #49 settle the naming.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PathLimits {
    /// a vertex at this level or deeper is not expanded, so a walk has at most this many edges
    pub max_depth: usize,
    /// the search stops, incomplete, instead of expanding one more vertex than this
    pub max_expansions: usize,
    /// at most this many walks are returned
    pub max_paths: usize,
}

/// a walk in the star expansion: `vertices[i]` and `vertices[i + 1]` are distinct members of
/// `edges[i]`, so `vertices.len() == edges.len() + 1`. **Unstable** (see [`PathLimits`]).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Walk<Ix> {
    pub vertices: Vec<VertexId<Ix>>,
    pub edges: Vec<EdgeId<Ix>>,
}

/// the walks a [`shortest_walks`](crate::search::BreadthFirstTraversal::shortest_walks) search
/// found. **Unstable** (see [`PathLimits`]).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Walks<Ix> {
    /// the shortest walks, in the order of their interleaved `(e1, v1, e2, v2, …)` ids
    pub walks: Vec<Walk<Ix>>,
    /// false when `max_expansions` stopped the search: the walks are shortest, but there may be
    /// more of them
    pub complete: bool,
    /// true when more walks than `max_paths` were found
    pub truncated: bool,
    /// the number of vertices the search expanded
    pub expansions: usize,
}
