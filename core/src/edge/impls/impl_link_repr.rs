/*
    Appellation: impl_link_repr <module>
    Created At: 2026.01.10:10:36:39
    Contrib: @FL03
*/
use crate::edge::Link;
use crate::idx::{EdgeId, RawIndex};
use crate::{Directed, Domain, Undirected};

impl<S, Idx> Link<S, Directed, Idx>
where
    Idx: RawIndex,
    S: Domain<Idx>,
{
    /// returns a new [`Directed`] hyperedge with the given id and nodes
    pub fn directed(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}

impl<S, Idx> Link<S, Undirected, Idx>
where
    Idx: RawIndex,
    S: Domain<Idx>,
{
    /// creates a new [`Undirected`] hyperedge with the given id and nodes
    pub fn undirected(id: EdgeId<Idx>, nodes: S) -> Self {
        Self::new(id, nodes)
    }
}
