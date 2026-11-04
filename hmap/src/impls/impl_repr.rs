/*
    appellation: impl_repr <module>
    authors: @FL03
*/
use crate::{DiHyperMap, HyperMap, UnHyperMap};
use core::hash::BuildHasher;
use rshyper::error::Result;
use rshyper::idx::{HashIndex, RawIndex, VertexId};
use rshyper::{AddStep, GraphProps, Mode, Weight};

impl<N, E, A, S, Ix> HyperMap<N, E, A, S>
where
    A: GraphProps<Kind = Mode, Ix = Ix>,
    S: BuildHasher,
    Ix: RawIndex,
{
    /// initialize a new, empty hypergraph with a dynamic [`Mode`] kind and the logical default
    /// for the indices.
    pub fn dynamic() -> Self
    where
        Ix: Default,
        S: Default,
    {
        HyperMap::new()
    }
}

impl<N, E, S, Ix> DiHyperMap<N, E, Ix, S>
where
    S: BuildHasher,
    Ix: RawIndex,
{
    /// initialize a new, empty hypergraph
    pub fn directed() -> Self
    where
        Ix: Default,
        S: Default,
    {
        HyperMap::new()
    }
}
impl<N, E, S, Ix> UnHyperMap<N, E, Ix, S>
where
    S: BuildHasher,
    Ix: RawIndex,
{
    /// initialize a new, empty hypergraph
    pub fn undirected() -> Self
    where
        Ix: Default,
        S: Default,
    {
        HyperMap::new()
    }
}

impl<E, A, S, Ix> HyperMap<(), E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: Copy + HashIndex + AddStep<Output = Ix> + PartialOrd,
{
    /// add an empty node with weight `()` and return its ID
    pub fn add_empty_node(&mut self) -> Result<VertexId<Ix>> {
        let weight = Weight::new(());
        self.add_node(weight)
    }
}

impl<N, E, A, S, Ix> HyperMap<Option<N>, E, A, S>
where
    A: GraphProps<Ix = Ix>,
    S: BuildHasher,
    Ix: Copy + HashIndex + AddStep<Output = Ix> + PartialOrd,
{
    /// insert [`Some`] vertex with weight `T` and return its ID
    pub fn add_some_node(&mut self, weight: N) -> Result<VertexId<Ix>> {
        self.add_node(Weight::some(weight))
    }
    /// insert [`None`] vertex with weight `T` and return its ID
    pub fn add_none_node(&mut self) -> Result<VertexId<Ix>> {
        self.add_node(Weight::none())
    }
}
