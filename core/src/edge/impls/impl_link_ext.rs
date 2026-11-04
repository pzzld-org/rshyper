/*
    appellation: impl_link_ext <module>
    authors: @FL03
*/
use crate::edge::{HyperEdgeRepr, Link, RawEdge};
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Domain, GraphType};

impl<S, K, Idx> RawEdge for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    type Kind = K;
    type Index = Idx;
    type Store = S;

    seal!();

    fn index(&self) -> &EdgeId<Idx> {
        self.id()
    }

    fn domain(&self) -> &S {
        self.domain()
    }

    fn domain_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
}

impl<S, K, Idx> HyperEdgeRepr for Link<S, K, Idx>
where
    S: Domain<Idx>,
    Idx: RawIndex,
    K: GraphType,
{
    fn new(id: EdgeId<Idx>, vertices: S) -> Self {
        Self::new(id, vertices)
    }
}

impl<S, K, Idx> Default for Link<S, K, Idx>
where
    Idx: RawIndex + Default,
    K: GraphType,
    S: Domain<Idx> + Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<S, K, Idx> core::fmt::Debug for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Edge")
            .field("id", &self.id())
            .field("domain", &self.domain())
            .finish()
    }
}

impl<S, K, Idx> core::fmt::Display for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ id: {}, domain: {:?} }}", self.id(), self.domain())
    }
}

impl<S, K, Idx> FromIterator<Idx> for Link<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    S: Domain<Idx> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Idx>,
    {
        let iter = iter.into_iter().map(|v| VertexId::from(v));
        Self::from_iter(iter)
    }
}

impl<S, K, Idx> FromIterator<VertexId<Idx>> for Link<S, K, Idx>
where
    Idx: Default + RawIndex,
    K: GraphType,
    S: Domain<Idx> + FromIterator<VertexId<Idx>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = VertexId<Idx>>,
    {
        let store = S::from_iter(iter);
        Self::from_domain(store)
    }
}

impl<S, K, Idx> From<EdgeId<Idx>> for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Default + Domain<Idx>,
{
    fn from(from: EdgeId<Idx>) -> Self {
        Self::from_id(from)
    }
}

impl<S, K, Idx> From<Link<S, K, Idx>> for EdgeId<Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn from(from: Link<S, K, Idx>) -> Self {
        from.id
    }
}

impl<S, K, Idx> AsRef<S> for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn as_ref(&self) -> &S {
        self.domain()
    }
}

impl<S, K, Idx> AsMut<S> for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn as_mut(&mut self) -> &mut S {
        self.domain_mut()
    }
}

impl<S, K, Idx> core::borrow::Borrow<EdgeId<Idx>> for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn borrow(&self) -> &EdgeId<Idx> {
        self.id()
    }
}

impl<S, K, Idx> core::borrow::BorrowMut<EdgeId<Idx>> for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn borrow_mut(&mut self) -> &mut EdgeId<Idx> {
        self.id_mut()
    }
}

impl<S, K, Idx> core::ops::Deref for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    type Target = EdgeId<Idx>;

    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

impl<S, K, Idx> core::ops::DerefMut for Link<S, K, Idx>
where
    Idx: RawIndex,
    K: GraphType,
    S: Domain<Idx>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.id_mut()
    }
}
