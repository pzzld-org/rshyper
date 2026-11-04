/*
    appellation: impl_edge <module>
    authors: @FL03
*/
use crate::edge::Link;
use crate::idx::{EdgeId, RawIndex, VertexId};
use crate::{Domain, GraphType};

impl<S, K, Ix> Link<S, K, Ix>
where
    Ix: RawIndex,
    K: GraphType,
    S: Domain<Ix>,
{
    /// returns a new instance using the given edge id and domain
    pub const fn new(id: EdgeId<Ix>, domain: S) -> Self {
        Self {
            id,
            domain,
            _kind: core::marker::PhantomData::<K>,
        }
    }
    /// creates a new edge with the given id
    pub fn from_id(id: EdgeId<Ix>) -> Self
    where
        S: Default,
    {
        Self::new(id, Default::default())
    }
    /// creates a new edge with the given nodes
    pub fn from_domain(nodes: S) -> Self
    where
        Ix: Default,
    {
        Self::new(Default::default(), nodes)
    }
    /// returns an immutable reference to the id
    pub const fn id(&self) -> &EdgeId<Ix> {
        &self.id
    }
    /// returns an immutable reference to the nodes
    pub const fn domain(&self) -> &S {
        &self.domain
    }
    /// returns a mutable reference to the nodes
    pub const fn domain_mut(&mut self) -> &mut S {
        &mut self.domain
    }
    /// updates the nodes and returns a mutable reference to the instance
    pub fn set_domain(&mut self, nodes: S) -> &mut Self {
        *self.domain_mut() = nodes;
        self
    }
    /// consumes the current instance to create another with the given id.
    pub fn with_id(self, id: EdgeId<Ix>) -> Self {
        Self { id, ..self }
    }
    /// consumes the current instance to create another with the given nodes.
    pub fn with_domain<S2: Domain<Ix>>(self, nodes: S2) -> Link<S2, K, Ix> {
        Link {
            id: self.id,
            domain: nodes,
            _kind: self._kind,
        }
    }
    /// returns true if the edge contains the given vertex index
    pub fn contains<Q>(&self, index: &Q) -> bool
    where
        Ix: PartialEq,
        Q: ?Sized + PartialEq,
        VertexId<Ix>: core::borrow::Borrow<Q>,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Ix>>,
    {
        use core::borrow::Borrow;
        self.domain().into_iter().any(|v| v.borrow() == index)
    }
    /// returns true if the edge contains all the given vertex indices
    pub fn contains_all<Q, I>(&self, indices: I) -> bool
    where
        Ix: PartialEq,
        I: IntoIterator<Item = Q>,
        Q: PartialEq,
        VertexId<Ix>: core::borrow::Borrow<Q>,
        for<'a> &'a S: IntoIterator<Item = &'a VertexId<Ix>>,
    {
        indices.into_iter().all(|index| self.contains(&index))
    }
    /// returns the number of vertices in the edge
    pub fn len(&self) -> usize {
        self.domain().len()
    }
    /// returns true if the edge has no vertices
    pub fn is_empty(&self) -> bool {
        self.domain().is_empty()
    }
}
/// private implementations for the [`Link`] struct
#[doc(hidden)]
#[allow(dead_code)]
impl<S, K, Ix> Link<S, K, Ix>
where
    Ix: RawIndex,
    K: GraphType,
    S: Domain<Ix>,
{
    /// returns a mutable reference to the id
    pub(crate) const fn id_mut(&mut self) -> &mut EdgeId<Ix> {
        &mut self.id
    }
    /// updates the id and returns a mutable reference to the instance
    pub(crate) fn set_id(&mut self, id: EdgeId<Ix>) -> &mut Self {
        self.id = id;
        self
    }
}
