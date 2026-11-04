/*
    appellation: incidence <module>
    authors: @FL03
*/
//! the vertex → incident-edge index kept beside the edge map of a [`HyperMap`](crate::HyperMap)
use super::aliases::EdgeMap;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};
use hashbrown::HashMap;
use rshyper_core::GraphType;
use rshyper_core::idx::{EdgeId, HashIndex, VertexId};

/// For every vertex, the ids of the edges whose domain contains it, ascending and each listed
/// once. The index is either _fresh_ (exact) or _stale_ (an escape hatch handed out `&mut`
/// access to the edge map, so the lists may be wrong and readers must scan instead).
pub(crate) struct Incidence<Ix, S> {
    map: HashMap<VertexId<Ix>, Vec<EdgeId<Ix>>, S>,
    stale: bool,
}

impl<Ix, S> Incidence<Ix, S> {
    pub(crate) fn with_hasher(hasher: S) -> Self {
        Self {
            map: HashMap::with_hasher(hasher),
            stale: false,
        }
    }

    pub(crate) fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, hasher),
            stale: false,
        }
    }
    pub(crate) const fn is_stale(&self) -> bool {
        self.stale
    }

    pub(crate) const fn invalidate(&mut self) {
        self.stale = true;
    }

    pub(crate) fn clear(&mut self) {
        self.map.clear();
        self.stale = false;
    }
    /// heap bytes held by the index, computed from capacities (hashbrown stores one control
    /// byte per bucket beside each `(key, value)` slot; group padding is ignored)
    #[cfg(test)]
    pub(crate) fn heap_bytes(&self) -> usize {
        let slot = core::mem::size_of::<(VertexId<Ix>, Vec<EdgeId<Ix>>)>() + 1;
        let buckets = if self.map.capacity() == 0 {
            0
        } else {
            (self.map.capacity() * 8 / 7).next_power_of_two()
        };
        buckets * slot
            + self
                .map
                .values()
                .map(|l| l.capacity() * core::mem::size_of::<EdgeId<Ix>>())
                .sum::<usize>()
    }
}

impl<Ix, S> Incidence<Ix, S>
where
    Ix: HashIndex,
    S: BuildHasher,
{
    /// the incident edge ids of `vertex`, ascending; meaningless while the index is stale
    pub(crate) fn get<Q>(&self, vertex: &Q) -> &[EdgeId<Ix>]
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.map.get(vertex).map_or(&[], Vec::as_slice)
    }
    /// removes and returns the list of `vertex`
    pub(crate) fn take<Q>(&mut self, vertex: &Q) -> Vec<EdgeId<Ix>>
    where
        Q: ?Sized + Eq + Hash,
        VertexId<Ix>: Borrow<Q>,
    {
        self.map.remove(vertex).unwrap_or_default()
    }
    /// records the edge `id` under every member of `domain`; a no-op while stale
    pub(crate) fn insert<'a, I>(&mut self, id: &EdgeId<Ix>, domain: I)
    where
        I: IntoIterator<Item = &'a VertexId<Ix>>,
        Ix: 'a + Clone + Ord,
    {
        if self.stale {
            return;
        }
        for v in domain {
            let list = self.map.entry(v.clone()).or_default();
            // fresh ids are allocated in ascending order, so this is almost always a push
            if list.last().is_none_or(|last| last < id) {
                list.push(id.clone());
            } else if let Err(pos) = list.binary_search(id) {
                list.insert(pos, id.clone());
            }
        }
    }
    /// forgets the edge `id` under every member of `domain`, shifting each member's list;
    /// a no-op while stale
    pub(crate) fn remove<'a, I>(&mut self, id: &EdgeId<Ix>, domain: I)
    where
        I: IntoIterator<Item = &'a VertexId<Ix>>,
        Ix: 'a + Ord,
    {
        if self.stale {
            return;
        }
        for v in domain {
            if let Some(list) = self.map.get_mut(v) {
                if let Ok(pos) = list.binary_search(id) {
                    list.remove(pos);
                }
                if list.is_empty() {
                    self.map.remove(v);
                }
            }
        }
    }
    /// forgets the ascending edge ids `ids` under every vertex in `members`, filtering each
    /// distinct member's list once (so a hub shared by many of the edges costs one pass);
    /// a no-op while stale
    pub(crate) fn remove_many<'a, I>(&mut self, ids: &[EdgeId<Ix>], members: I)
    where
        I: IntoIterator<Item = &'a VertexId<Ix>>,
        Ix: 'a + Ord,
    {
        if self.stale || ids.is_empty() {
            return;
        }
        let mut members = members.into_iter().collect::<Vec<_>>();
        members.sort_unstable();
        members.dedup();
        for v in members {
            if let Some(list) = self.map.get_mut(v) {
                list.retain(|e| ids.binary_search(e).is_err());
                if list.is_empty() {
                    self.map.remove(v);
                }
            }
        }
    }
    /// rebuilds the index from `edges` in O(I + |E| log |E|) for `I` total incidences
    pub(crate) fn rebuild<E, K>(&mut self, edges: &EdgeMap<E, K, Ix, S>)
    where
        Ix: Clone + Ord,
        K: GraphType,
    {
        self.map.clear();
        let mut sorted = edges.iter().collect::<Vec<_>>();
        sorted.sort_unstable_by(|a, b| a.0.cmp(b.0));
        for (id, edge) in sorted {
            for v in edge.domain() {
                self.map.entry(v.clone()).or_default().push(id.clone());
            }
        }
        self.stale = false;
    }
}

impl<Ix, S> Clone for Incidence<Ix, S>
where
    Ix: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            stale: self.stale,
        }
    }
}

impl<Ix, S> Default for Incidence<Ix, S>
where
    S: Default,
{
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::UnHyperMap;
    use rshyper_core::Weight;
    use rshyper_core::idx::{EdgeId, VertexId};

    type G = UnHyperMap<usize, usize>;

    fn graph() -> (G, [VertexId<usize>; 3], EdgeId<usize>) {
        let mut g = G::undirected();
        let v = [(); 3].map(|_| g.add_vertex().unwrap());
        let e = g.add_link([v[0], v[1]]).unwrap();
        g.add_link([v[1], v[2]]).unwrap();
        (g, v, e)
    }

    #[test]
    fn weight_and_node_accessors_keep_the_index_fresh() {
        let (mut g, v, e) = graph();
        *g.get_edge_weight_mut(&e).unwrap() = Weight(7);
        g.set_edge_weight(&e, Weight(8)).unwrap();
        *g.get_node_weight_mut(&v[0]).unwrap() = Weight(1);
        g.set_node_weight(&v[0], Weight(2)).unwrap();
        g.get_node_mut(&v[1]).unwrap();
        g.nodes_mut();
        g.iter_nodes_mut().for_each(drop);
        g.points_mut().for_each(drop);
        g.history_mut();
        g.position_mut();
        assert!(!g.incidence.is_stale());
    }

    #[test]
    #[allow(deprecated)]
    fn every_edge_escape_hatch_marks_the_index_stale() {
        let hatches: [fn(&mut G, EdgeId<usize>); 12] = [
            |g, _| {
                g.edges_mut();
            },
            |g, _| {
                let edges = g.edges().clone();
                g.set_edges(edges);
            },
            |g, e| {
                g.get_edge_mut(&e).unwrap();
            },
            |g, e| {
                g.get_domain_mut(&e).unwrap();
            },
            |g, e| {
                g.edge(e);
            },
            |g, _| g.iter_edges_mut().for_each(drop),
            |g, _| g.facets_mut().for_each(drop),
            |g, e| {
                let _ = &mut g[&e];
            },
            |g, _| {
                g.surfaces_mut();
            },
            |g, e| {
                g.get_surface_mut(&e).unwrap();
            },
            |g, e| {
                g.get_edge_vertices_mut(&e).unwrap();
            },
            |g, e| {
                g.surface(e);
            },
        ];
        // every checked edge mutation rebuilds a stale index, and so does `reindex`
        type Rebuild = fn(&mut G, [VertexId<usize>; 3], EdgeId<usize>);
        let rebuilds: [Rebuild; 5] = [
            |g, _, e| {
                g.remove_edge(&e).unwrap();
            },
            |g, v, _| {
                g.remove_node(&v[2]).unwrap();
            },
            |g, v, _| {
                g.add_link([v[0]]).unwrap();
            },
            |g, _, _| {
                g.retain_edges(|_, _| true);
            },
            |g, _, _| {
                g.reindex();
            },
        ];
        for (i, hatch) in hatches.iter().enumerate() {
            for (j, rebuild) in rebuilds.iter().enumerate() {
                let (mut g, v, e) = graph();
                hatch(&mut g, e);
                assert!(g.incidence.is_stale(), "hatch {i} left the index fresh");
                // stale reads are still exact
                assert_eq!(g.find_edges_with_node(&v[1]).count(), 2);
                rebuild(&mut g, v, e);
                assert!(!g.incidence.is_stale(), "rebuild {j} after hatch {i}");
                let want = g.edges().values().filter(|x| x.contains(&v[1])).count();
                assert_eq!(g.incidence.get(&v[1]).len(), want);
            }
        }
    }

    #[test]
    fn retain_edges_rebuilds_and_a_panicking_predicate_leaves_it_stale() {
        let (mut g, v, e) = graph();
        // the predicate edits a kept domain; the rebuilt index must see it
        g.retain_edges(|id, edge| {
            edge.domain_mut().insert(v[0]);
            *id != e
        });
        assert!(!g.incidence.is_stale());
        assert_eq!(g.incidence.get(&v[0]).len(), 1);
        assert_eq!(g.incidence.get(&v[1]).len(), 1);
        let (mut g, v, _) = graph();
        let mut calls = 0;
        let caught = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            g.retain_edges(|_, _| {
                calls += 1;
                assert!(calls < 2, "predicate failure");
                false
            });
        }));
        assert!(caught.is_err());
        assert!(g.incidence.is_stale());
        let want = g.edges().values().filter(|x| x.contains(&v[1])).count();
        assert_eq!(g.find_edges_with_node(&v[1]).count(), want);
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn par_iter_facets_mut_marks_the_index_stale() {
        use rayon::iter::ParallelIterator;
        let (mut g, ..) = graph();
        g.par_iter_facets_mut().for_each(drop);
        assert!(g.incidence.is_stale());
    }

    /// bytes per incidence (from capacities) on W10k-shaped graphs: 3 edges per vertex, 2..=4
    /// members each. The PR states the budget asserted here.
    #[test]
    fn index_memory_per_incidence_is_within_budget() {
        let mut seed = 0x5110_2026_u64;
        let mut below = |n: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % n as u64) as usize
        };
        for edges in [100, 1_000, 10_000, 100_000] {
            let nodes = edges / 3;
            let mut g = G::undirected();
            let v = (0..nodes)
                .map(|_| g.add_vertex().unwrap())
                .collect::<Vec<_>>();
            let mut incidences = 0;
            for _ in 0..edges {
                let k = 2 + below(3);
                let id = g.add_link((0..k).map(|_| v[below(nodes)])).unwrap();
                incidences += g.get_edge_order(&id).unwrap();
            }
            let incremental = g.incidence.heap_bytes() as f64 / incidences as f64;
            g.reindex();
            let rebuilt = g.incidence.heap_bytes() as f64 / incidences as f64;
            std::println!(
                "INCIDENCE edges={edges} incidences={incidences} bytes/incidence incremental={incremental:.1} rebuilt={rebuilt:.1}"
            );
            assert!(incremental <= 32.0 && rebuilt <= 32.0);
        }
    }
}
