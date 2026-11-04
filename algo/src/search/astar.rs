/*
    Appellation: impl_astar <module>
    Contrib: @FL03
*/
//! this module implements the A* search algorithm

use crate::search::Heuristic;
use hashbrown::{DefaultHashBuilder, HashMap};
use rshyper_core::idx::{VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph};

pub(crate) type SourceMap<Ix, S = DefaultHashBuilder> = HashMap<VertexId<Ix>, VertexId<Ix>, S>;

pub(crate) type ScoreMap<K, V, S = DefaultHashBuilder> = HashMap<VertexId<K>, V, S>;

/// An A* Search algorithm implementation for hypergraphs
pub struct AStarSearch<'a, N, E, A, F, H, S = DefaultHashBuilder>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
    F: Heuristic<A::Ix>,
{
    pub(crate) graph: &'a H,
    pub(crate) open_set: VertexSet<A::Ix, S>,
    pub(crate) closed_set: VertexSet<A::Ix, S>,
    pub(crate) came_from: SourceMap<A::Ix, S>,
    pub(crate) g_score: ScoreMap<A::Ix, F::Output, S>,
    pub(crate) f_score: ScoreMap<A::Ix, F::Output, S>,
    pub(crate) heuristic: F,
    pub(crate) _marker: core::marker::PhantomData<(N, E)>,
}
