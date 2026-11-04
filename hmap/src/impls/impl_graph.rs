/*
    appellation: impl_graph <module>
    authors: @FL03
*/
use crate::graph::HyperMap;
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use rshyper_core::edge::{HashEdge, HyperEdge};
use rshyper_core::idx::{EdgeId, HashIndex, IndexError, VertexId};
use rshyper_core::{AddStep, Error, GraphProps, GraphType, Node, Result, VertexSet, Weight};

/// this implementation of the [`HyperMap`] works to provide fundamental manipulation methods
/// alongside additional functional accessors, validators, and more.
impl<N, E, A, S, K, Ix> HyperMap<N, E, A, S>
where
    A: GraphProps<Kind = K, Ix = Ix>,
    S: BuildHasher,
    K: GraphType,
    Ix: HashIndex,
{
    /// add a new _unweighted_ hyperedge into the graph composed from the given vertices.
    pub fn add_link<I>(&mut self, vertices: I) -> Result<EdgeId<Ix>>
    where
        I: IntoIterator<Item = VertexId<Ix>>,
        Ix: AddStep<Output = Ix> + Clone + PartialOrd,
        E: Default,
        S: Default,
    {
        self.add_edge(vertices, Default::default())
    }
    /// add a new hyperedge with the given vertices and weight, returning the corresponding
    /// edge index.
    ///
    /// Repeated vertices collapse into one incidence (set semantics). The call fails with
    /// [`Error::NodeNotFound`] if any vertex is missing and [`Error::EmptyHyperedge`] if the
    /// domain is empty; on error the graph, including its history, is unchanged.
    pub fn add_edge<I>(&mut self, vertices: I, weight: Weight<E>) -> Result<EdgeId<Ix>>
    where
        I: IntoIterator<Item = VertexId<Ix>>,
        Ix: AddStep<Output = Ix> + Clone + PartialOrd,
        S: Default,
    {
        let mut verts = VertexSet::<Ix, S>::default();
        for v in vertices {
            if !self.contains_node(&v) {
                return Err(Error::NodeNotFound);
            }
            verts.insert(v);
        }
        // validate before allocating an id so a failure leaves the history untouched
        if verts.is_empty() {
            return Err(Error::EmptyHyperedge);
        }
        // fetch the next edge index
        let id = self.next_edge_id()?;
        // create a new surface
        let surface = HyperEdge::from_parts(id.clone(), verts, weight);
        // add the hyperedge to the graph
        self.insert_edge_unchecked(surface)
    }
    /// add a new node with the given weight and return its index
    pub fn add_node(&mut self, Weight(weight): Weight<N>) -> Result<VertexId<Ix>>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
    {
        // generate a new index to identify the new node
        let ndx = self.next_vertex_id()?;
        // initialize a new node with the given weight & index
        let node = Node::new(ndx, weight);
        // insert the new node into the vertices map
        self.insert_node_unchecked(node)
    }
    /// given a set of weights, insert a new node into the graph for each and return its id as
    /// an iterator of [`VertexId`]. Nodes are added lazily as the iterator is consumed, and it
    /// stops at the first allocation error; use [`add_node`](Self::add_node) to observe it.
    pub fn add_nodes<I>(&mut self, weights: I) -> impl Iterator<Item = VertexId<Ix>>
    where
        I: IntoIterator<Item = N>,
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
    {
        // an allocation error repeats for every later call, so stop at the first one
        weights
            .into_iter()
            .map_while(|weight| self.add_node(Weight(weight)).ok())
    }
    /// add a new hypernode using the logical [`Default`] for the weight of type `N` and
    /// return its index.
    pub fn add_vertex(&mut self) -> Result<VertexId<Ix>>
    where
        N: Default,
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
    {
        self.add_node(Default::default())
    }
    /// reset the hypergraph by clearing all nodes, edges, and facets
    pub fn clear(&mut self) -> &mut Self
    where
        Ix: Default,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("clearing the hypergraph...");
        // clear the edges
        self.edges_mut().clear();
        // clear the nodes
        self.nodes_mut().clear();
        // clear the history
        self.history_mut().clear();
        #[cfg(feature = "tracing")]
        tracing::info!("cleared the hypergraph successfully...");
        self
    }
    /// returns an interator over all the edges within the graph that contains the node
    /// associated with the given index.
    pub fn find_edges_with_node<Q>(&self, index: &Q) -> impl Iterator<Item = &EdgeId<Ix>>
    where
        Q: ?Sized + PartialEq,
        VertexId<Ix>: Borrow<Q>,
    {
        // filter the edges to find those that contain the vertex
        self.edges().iter().filter_map(move |(edge_id, facet)| {
            if facet.contains(index) {
                Some(edge_id)
            } else {
                None
            }
        })
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "neighbors", target = "hyper_map")
    )]
    /// returns a set of vertices that neighbor the node associated with the given index; this
    /// method will return an error if the vertex does not exist in the hypergraph.
    pub fn find_node_neighbors(&self, index: &VertexId<Ix>) -> Result<VertexSet<Ix, S>>
    where
        Ix: Clone,
        S: Default,
    {
        if !self.contains_node(index) {
            #[cfg(feature = "tracing")]
            tracing::error!("the vertex {index:?} does not exist in the hypergraph");
            return Err(Error::NodeNotFound);
        }
        // initialize an empty set to hold the neighbors
        let mut neighbors = VertexSet::<Ix, S>::default();
        // iterate through all the connections
        self.facets().for_each(|edge| {
            if edge.contains(index) {
                neighbors.extend(edge.domain().iter().filter(|&v| v != index).cloned());
            }
        });
        Ok(neighbors)
    }
    /// returns the set of vertices composing the given edge
    pub fn get_domain<Q>(&self, index: &Q) -> Result<&VertexSet<Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge(index).map(|edge| edge.domain())
    }
    /// returns a mutable reference to the set of vertices composing the given edge
    pub fn get_domain_mut<Q>(&mut self, index: &Q) -> Result<&mut VertexSet<Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge_mut(index).map(|edge| edge.domain_mut())
    }
    /// returns an immutable reference to the [`HashEdge`] associated with the given index
    pub fn get_edge<Q>(&self, index: &Q) -> Result<&HashEdge<E, K, Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.edges().get(index).ok_or_else(Error::edge_not_found)
    }
    /// returns a mutable reference to the [`HashEdge`] associated with the given index
    pub fn get_edge_mut<Q>(&mut self, index: &Q) -> Result<&mut HashEdge<E, K, Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.edges_mut()
            .get_mut(index)
            .ok_or_else(|| Error::EdgeNotFound)
    }
    /// returns the number of vertices within the given edge
    pub fn get_edge_order<Q>(&self, index: &Q) -> Result<usize>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge(index).map(|edge| edge.len())
    }
    /// returns an immutable reference to the weight of a hyperedge
    pub fn get_edge_weight<Q>(&self, index: &Q) -> Result<&Weight<E>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge(index).map(|edge| edge.weight())
    }
    /// returns a mutable reference to the weight of a hyperedge
    pub fn get_edge_weight_mut<Q>(&mut self, index: &Q) -> Result<&mut Weight<E>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge_mut(index).map(|edge| edge.weight_mut())
    }
    /// returns the degree of a given vertex where the degree is the number of hyperedges that
    /// contain the vertex
    pub fn get_node_degree<Q>(&self, index: &Q) -> usize
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.edges()
            .values()
            .filter(|facet| facet.link().domain().contains(index))
            .count()
    }
    /// returns the weight of a particular vertex
    pub fn get_node<Q>(&self, index: &Q) -> Result<&Node<N, Ix>>
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.nodes().get(index).ok_or(Error::NodeNotFound)
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_mut<Q>(&mut self, index: &Q) -> Result<&mut Node<N, Ix>>
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.nodes_mut().get_mut(index).ok_or(Error::NodeNotFound)
    }
    /// returns an immutable reference to the weight of a vertex
    pub fn get_node_weight<Q>(&self, index: &Q) -> Result<&Weight<N>>
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.get_node(index).map(|node| node.weight())
    }
    /// returns a mutable reference to the weight of a vertex
    pub fn get_node_weight_mut<Q>(&mut self, index: &Q) -> Result<&mut Weight<N>>
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.get_node_mut(index).map(|node| node.weight_mut())
    }
    /// returns the [`Node`]s of the members of the given edge.
    ///
    /// # Errors
    ///
    /// - [`EdgeNotFound`](Error::EdgeNotFound) if there is no such edge;
    /// - [`NodeNotFound`](Error::NodeNotFound) if a member has no node entry, which only an
    ///   escape hatch such as `get_domain_mut` or [`retain_nodes`](Self::retain_nodes) can
    ///   cause. No member is skipped silently.
    pub fn load_edge_nodes<Q>(&self, index: &Q) -> Result<Vec<&Node<N, Ix>>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge(index)?
            .domain()
            .iter()
            .map(|v| self.get_node(v))
            .collect()
    }
    /// merge two edges within the hypergraph into one by combining their vertices and using
    /// the [`Add`](core::ops::Add) trait to merge their weights;
    /// trait
    pub fn merge_edges<Q>(&mut self, e1: &Q, e2: &Q) -> Result<EdgeId<Ix>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
        S: Default,
        for<'a> &'a E: core::ops::Add<Output = E>,
    {
        self.merge_edges_with(e1, e2, |w1, w2| {
            // use the `Add` trait to merge the weights of the two edges
            w1 + w2
        })
    }
    /// merge two edges within the hypergraph into one by combining their vertices and using
    /// the provided function to merge their weights;
    pub fn merge_edges_with<Q, F>(&mut self, e1: &Q, e2: &Q, f: F) -> Result<EdgeId<Ix>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
        Ix: AddStep<Output = Ix> + Copy + PartialOrd,
        F: FnOnce(&E, &E) -> E,
        S: Default,
    {
        // validate both operands before removing either so a failure loses nothing
        if e1 == e2 {
            return Err(IndexError::DuplicateIndex.into());
        }
        if !self.contains_edge(e1) || !self.contains_edge(e2) {
            return Err(Error::EdgeNotFound);
        }
        // allocate first: a failed allocation must not lose either operand
        let edge_id = self.next_edge_id()?;
        // try and remove the two edges from the hypergraph
        let s1 = self.remove_edge(e1)?;
        let s2 = self.remove_edge(e2)?;
        // merge the vertices of the two edges by unionizing their domains
        let vertices = s1
            .domain()
            .union(s2.domain())
            .copied()
            .collect::<VertexSet<Ix, S>>();
        // merge the two weights using the provided function
        let weight = f(*s1.weight().view(), *s2.weight().view());
        // initialize a new facet using the merged vertices, new index, and source weight
        let surface = HyperEdge::from_parts(edge_id, vertices, Weight(weight));
        // insert the new hyperedge into the surfaces map
        self.edges_mut().insert(edge_id, surface);
        // return the new edge ID
        Ok(edge_id)
    }
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    /// remove the hyperedge with the given index from the hypergraph
    pub fn remove_edge<Q>(&mut self, index: &Q) -> Result<HashEdge<E, K, Ix, S>>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.edges_mut()
            .remove(index)
            .ok_or(Error::EdgeNotFound)
            .inspect(|edge| {
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "successfully removed the hyperedge from the hypergraph with id: {}",
                    edge.id()
                );
                // remove the edge id from the history
                self.history_mut().remove_edge(edge.id());
                #[cfg(feature = "tracing")]
                tracing::trace!("removed the edge id ({}) from the history", edge.id());
            })
    }
    #[inline]
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    /// removes the vertex with the given id and all of its associated hyperedges
    pub fn remove_node<Q>(&mut self, index: &Q) -> Result<Node<N, Ix>>
    where
        Q: Eq + core::fmt::Debug + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        #[cfg(feature = "tracing")]
        tracing::debug!("removing the vertex {index:?} from the hypergraph...");

        self.nodes_mut()
            .remove(index)
            .ok_or(Error::NodeNotFound)
            .inspect(|node| {
                // remove the node from the history
                self.history_mut().remove_node(node.id());
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "successfully removed the node; removing edges that contained the vertex..."
                );
                // Remove all hyperedges containing this vertex
                self.retain_edges(|_, facet| !facet.contains(index));
                #[cfg(feature = "tracing")]
                tracing::trace!(
                    "successfully removed the edges containing the removed vertex {index:?}..."
                );
            })
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    /// retain nodes in the hypergraph based on a predicate; the node history is pruned to
    /// match.
    ///
    /// ## Safety
    ///
    /// The body is safe Rust; `unsafe` only marks the contract. This removes nodes only:
    /// incident edges are left dangling, still naming the removed vertices, and reads such as
    /// [`load_edge_nodes`](Self::load_edge_nodes) then return
    /// [`NodeNotFound`](Error::NodeNotFound). Which deletion policy the graph should enforce
    /// is ADR-DELETE (#117).
    pub unsafe fn retain_nodes<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&VertexId<Ix>, &mut Node<N, Ix>) -> bool,
    {
        let Self { nodes, history, .. } = self;
        nodes.retain(f);
        history.retain_nodes(|id| nodes.contains_key(id));
        self
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, level = "trace", target = "hyper_map")
    )]
    /// retain surfaces in the hypergraph based on a predicate; the edge history is pruned to
    /// match.
    pub fn retain_edges<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut(&EdgeId<Ix>, &mut HashEdge<E, K, Ix, S>) -> bool,
    {
        let Self { edges, history, .. } = self;
        edges.retain(f);
        history.retain_edges(|id| edges.contains_key(id));
        self
    }
    #[inline]
    /// update the weight of an edge with the given index
    pub fn set_edge_weight<Q>(&mut self, index: &Q, weight: Weight<E>) -> Result<&mut Self>
    where
        Q: ?Sized + Eq + Hash,
        EdgeId<Ix>: Borrow<Q>,
    {
        self.get_edge_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| Error::EdgeNotFound)?;
        Ok(self)
    }
    #[inline]
    /// update the weight of a given vertex
    pub fn set_node_weight<Q>(&mut self, index: &Q, weight: Weight<N>) -> Result<&mut Self>
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.get_node_weight_mut(index)
            .map(|w| {
                *w = weight;
            })
            .map_err(|_| Error::NodeNotFound)?;
        Ok(self)
    }
}

#[allow(dead_code)]
/// private implementations of the [`HyperMap`] providing methods, for convenience and
/// consistency.
impl<N, E, A, K, Idx, S> HyperMap<N, E, A, S>
where
    A: GraphProps<Ix = Idx, Kind = K>,
    K: GraphType,
    Idx: HashIndex,
    S: BuildHasher,
{
    /// this method is responsible for directly registering new surfaces with the system,
    /// implementing additional checks to ensure the validity of the instance. More
    /// specifically, it ensures that:
    ///
    /// - the surface must not be empty
    /// - the associated id must be recorded in the ledger, but not present within the graph
    ///
    /// if **any** of these condition are not met, an error will be thrown.
    pub(crate) fn insert_edge(&mut self, edge: HashEdge<E, K, Idx, S>) -> Result<EdgeId<Idx>>
    where
        Idx: Clone,
    {
        // check the graph to make sure the edge doesn't exist
        if self.contains_edge(edge.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the edge with id ({}) already exists in the graph; cannot insert it again",
                edge.id()
            );
            return Err(Error::edge_already_exists(edge.id().get().clone()));
        }
        // verify the edge id is already recorded in the history
        if !self.history().contains_edge(edge.id()) {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "the id ({}) is not recorded in the history; insterting the edge id into the history",
                edge.id()
            );
            self.history_mut().add_edge(edge.id().clone());
        }
        // get the id of the surface
        self.insert_edge_unchecked(edge).inspect(|_id| {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "successfully inserted the hyperedge ({}) into the graph",
                _id
            );
        })
    }
    /// this method is responsible for directly registering new nodes with the system,
    /// implementing additional checks to ensure the validity of the instance. More
    /// specifically, it ensures that:
    ///
    /// - the associated is is recorded in the ledger
    /// - the node doesn't already exist in the graph
    ///
    /// if **any** of these condition are not met, an error will be thrown.
    pub(crate) fn insert_node(&mut self, data: Node<N, Idx>) -> Result<VertexId<Idx>>
    where
        Idx: Clone,
    {
        // if the node already exists in the graph, return an error
        if self.contains_node(data.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the node with id ({}) already exists in the graph; cannot insert it again",
                data.id()
            );
            return Err(Error::node_already_exists(data.id().get().clone()));
        }
        // verify the edge id is already recorded in the history
        if !self.history().contains_node(data.id()) {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "the id ({}) is not recorded in the history; inserting the node id into the history",
                data.id()
            );
            self.history_mut().add_node(data.id().clone());
            return Err(Error::NodeNotFound);
        }
        // add the node
        self.insert_node_unchecked(data).inspect(|_id| {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "successfully inserted the hypernode ({}) into the graph",
                _id
            );
        })
    }
    /// this method is responsible for directly registering new surfaces with the system,
    /// implementing a single check to verify the composition of the edge _domain_.
    ///
    /// ## Saftey
    ///
    /// This method is considered unsafe because it allows for the direct insertion of an
    /// externally initialized [`HashEdge`] instance meaning that it is up to the developer to
    /// ensure that:
    ///
    /// - the associated id must be recorded in the ledger
    /// - the id must not already exist in the graph
    ///
    /// if **any** of these condition are not met, errors will eventually propagate within the
    /// graph.
    pub(crate) fn insert_edge_unchecked(
        &mut self,
        edge: HashEdge<E, K, Idx, S>,
    ) -> Result<EdgeId<Idx>>
    where
        Idx: Clone,
    {
        if edge.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "attempted to insert an empty hyperedge ({id})",
                id = edge.id()
            );
            return Err(Error::EmptyHyperedge);
        }
        // get the id of the surface
        let id = edge.id().clone();
        #[cfg(feature = "tracing")]
        tracing::debug!("inserting a new hyperedge ({id}) into the graph...");
        // insert the new hyperedge into the adjacency map
        self.edges_mut().insert(id.clone(), edge);
        // return the id
        Ok(id)
    }
    /// this method is responsible for directly registering new nodes with the system,
    /// implementing additional checks to ensure the validity of the instance.
    ///
    /// ## Saftey
    ///
    /// This method is considered unsafe because it allows for the direct insertion of an
    /// externally initialized [`Node`] instance meaning that it is up to the developer to
    /// ensure that:
    ///
    /// - the associated is is recorded in the ledger
    /// - the node doesn't already exist in the graph
    ///
    /// if **any** of these condition are not met, errors will eventually propagate within the
    /// graph.
    pub(crate) fn insert_node_unchecked(&mut self, data: Node<N, Idx>) -> Result<VertexId<Idx>>
    where
        Idx: Clone,
    {
        // get the id of the surface
        let id = data.id().clone();
        #[cfg(feature = "tracing")]
        tracing::debug!("inserting a new hypernode ({id}) into the graph...");
        // insert the new hyperedge into the adjacency map
        self.nodes_mut().insert(id.clone(), data);
        // return the id
        Ok(id)
    }
}
