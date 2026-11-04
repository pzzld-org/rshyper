/*
    appellation: dijkstra <module>
    authors: @FL03
*/
//! this module implements Dijkstra's shortest-path algorithm for hypergraphs

use hashbrown::{DefaultHashBuilder, HashMap};
use rshyper_core::idx::{VertexId, VertexSet};
use rshyper_core::{GraphProps, HyperGraph};

/// a type alias for a map of distances for vertices in the graph
pub(crate) type Distances<K, V = f64, S = DefaultHashBuilder> = HashMap<VertexId<K>, V, S>;
/// a type alias for the history of previous vertices in the graph, maps vertices to vertices
pub(crate) type PreviousHistory<K, S = DefaultHashBuilder> = HashMap<VertexId<K>, VertexId<K>, S>;

/// Dijkstra's shortest path algorithm for hypergraphs
pub struct Dijkstra<'a, N, E, A, H, S = DefaultHashBuilder>
where
    A: GraphProps,
    H: HyperGraph<N, E, A>,
{
    pub(crate) graph: &'a H,
    pub(crate) distances: Distances<A::Ix, E, S>,
    pub(crate) previous: PreviousHistory<A::Ix, S>,
    pub(crate) visited: VertexSet<A::Ix, S>,
    pub(crate) _marker: core::marker::PhantomData<(N, E)>,
}
