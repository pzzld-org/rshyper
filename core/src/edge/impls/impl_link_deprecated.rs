/*
    appellation: impl_link_deprecated <module>
    authors: @FL03
*/
#![allow(deprecated)]

use crate::edge::Link;
use crate::idx::RawIndex;
use crate::{Domain, GraphType};

#[doc(hidden)]
impl<S, K, Idx> Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    #[deprecated(
        note = "Use `Link::from_domain` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn from_points(nodes: S) -> Self
    where
        Idx: Default,
    {
        Self::from_domain(nodes)
    }
    #[deprecated(
        note = "Use `Link::domain` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    /// returns an immutable reference to the nodes
    pub const fn points(&self) -> &S {
        self.domain()
    }
    #[deprecated(
        note = "Use `Link::domain_mut` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    /// returns a mutable reference to the nodes
    pub const fn points_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
    #[deprecated(
        note = "Use `Link::set_domain` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn set_points(&mut self, nodes: S) -> &mut Self {
        self.set_domain(nodes)
    }
    #[deprecated(
        note = "Use `Link::with_domain` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn with_points<S2: Domain<Idx>>(self, nodes: S2) -> Link<S2, K, Idx> {
        self.with_domain(nodes)
    }
}
