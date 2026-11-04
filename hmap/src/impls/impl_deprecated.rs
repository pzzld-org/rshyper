/*
    appellation: impl_deprecated <module>
    authors: @FL03
*/
use crate::HyperMap;
use crate::types::{EdgeEntry, EdgeMap};
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use rshyper::error::Result;
use rshyper::prelude::*;

#[doc(hidden)]
impl<N, E, A, S, Ix, K> HyperMap<N, E, A, S>
where
    A: GraphProps<Kind = K, Ix = Ix>,
    S: BuildHasher,
    Ix: HashIndex,
    K: GraphType,
{
    #[deprecated(
        note = "use `set_edges` instead; this method will be removed in the next major release",
        since = "0.1.8"
    )]
    pub fn set_surfaces(&mut self, edges: EdgeMap<E, K, Ix, S>) -> &mut Self {
        self.set_edges(edges)
    }
    #[deprecated(
        note = "use `load_edge_nodes` instead; this method will be removed in the next major release",
        since = "0.1.7"
    )]
    pub fn get_edge_nodes<Q>(&self, index: &Q) -> Result<Vec<&Node<N, Ix>>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.load_edge_nodes(index)
    }
    #[deprecated(
        note = "use `get_edge` instead; this method will be removed in the next major release",
        since = "0.1.7"
    )]
    pub fn get_surface<Q>(&self, index: &Q) -> Result<&HashEdge<E, K, Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge(index)
    }
    #[deprecated(
        note = "use `get_edge_mut` instead; this method will be removed in the next major release",
        since = "0.1.7"
    )]
    pub fn get_surface_mut<Q>(&mut self, index: &Q) -> Result<&mut HashEdge<E, K, Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge_mut(index)
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `add_edge` instead; this method will be removed in the next major release",
        since = "0.1.7"
    )]
    pub fn add_surface<I>(&mut self, vertices: I, weight: Weight<E>) -> Result<EdgeId<Ix>>
    where
        I: IntoIterator<Item = VertexId<Ix>>,
        Ix: AddStep<Output = Ix> + Clone + Ord,
        S: Default,
    {
        self.add_edge(vertices, weight)
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `retain_edges` instead; this method will be removed in the next major release",
        since = "0.1.7"
    )]
    pub fn retain_surfaces<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&EdgeId<Ix>, &mut HashEdge<E, K, Ix, S>) -> bool,
        Ix: Ord,
    {
        self.retain_edges(f)
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `remove_edge` instead; this method will be removed in the next major release",
        since = "0.1.7"
    )]
    pub fn remove_surface<Q>(&mut self, index: &Q) -> Result<HashEdge<E, K, Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
        Ix: Ord,
    {
        self.remove_edge(index)
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `get_domain` instead; this method will be removed in the next major release",
        since = "0.1.5"
    )]
    pub fn get_edge_vertices<Q>(&self, index: &Q) -> Result<&VertexSet<Ix, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Ix>: core::borrow::Borrow<Q>,
    {
        self.get_domain(index)
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `get_domain_mut` instead; this method will be removed in the next major release",
        since = "0.1.5"
    )]
    pub fn get_edge_vertices_mut<Q>(&mut self, index: &Q) -> Result<&mut VertexSet<Ix, S>>
    where
        Q: Eq + Hash + ?Sized,
        EdgeId<Ix>: core::borrow::Borrow<Q>,
    {
        self.get_domain_mut(index)
    }
    #[doc(hidden)]
    #[deprecated(
        since = "0.1.5",
        note = "use `edge` instead; this method will be removed in the next major release."
    )]
    pub fn surface(&mut self, index: EdgeId<Ix>) -> EdgeEntry<'_, E, K, Ix, S>
    where
        Ix: Eq + Hash,
    {
        self.edge(index)
    }
    #[doc(hidden)]
    #[deprecated(
        since = "0.1.5",
        note = "use `edges` instead; this method will be removed in the next major release."
    )]
    pub const fn surfaces(&self) -> &EdgeMap<E, K, Ix, S> {
        self.edges()
    }
    #[doc(hidden)]
    #[deprecated(
        since = "0.1.5",
        note = "use `edges_mut` instead; this method will be removed in the next major release."
    )]
    pub const fn surfaces_mut(&mut self) -> &mut EdgeMap<E, K, Ix, S> {
        self.edges_mut()
    }
    #[doc(hidden)]
    #[deprecated(
        since = "0.1.3",
        note = "use `is_node_in_domain` instead; this method will be removed in the next major release."
    )]
    pub fn contains_node_in_edge<Q, Q2>(&self, index: &Q, vertex: &Q2) -> bool
    where
        A::Ix: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        Q2: Eq + Hash,
        EdgeId<A::Ix>: Borrow<Q>,
        VertexId<A::Ix>: Borrow<Q2>,
    {
        if let Some(surface) = self.edges().get(index) {
            return surface.contains(vertex);
        }
        false
    }
    #[doc(hidden)]
    #[deprecated(
        since = "0.1.2",
        note = "use `contains_edge` instead; this method will be removed in the next major release."
    )]
    pub fn contains_surface<Q>(&self, index: &Q) -> bool
    where
        A::Ix: Eq + Hash,
        Q: Eq + Hash + ?Sized,
        EdgeId<A::Ix>: Borrow<Q>,
    {
        self.edges().contains_key(index)
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `size` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn total_edges(&self) -> usize {
        self.edges().len()
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `order` instead; this method will be removed in the next major release.",
        since = "0.1.2"
    )]
    pub fn total_nodes(&self) -> usize {
        self.nodes().len()
    }
    #[doc(hidden)]
    #[deprecated(
        note = "use `order` instead; this method will be removed in the next major release.",
        since = "0.1.0"
    )]
    pub fn total_vertices(&self) -> usize {
        self.order()
    }
}
