/*
    Appellation: incidence <bench>
    Contrib: @FL03
*/
//! #123: the incidence index against the full edge scan it replaces. The `scan` rows read the
//! same graph after `edges_mut()` marked its index stale, so every read falls back to scanning
//! the edge map (and sorts the hits). The `old_scan` rows replay the pre-index implementations
//! through the public API. Run with `CARGO_PROFILE_BENCH_OPT_LEVEL=3`.
extern crate rshyper_core as rshyper;
extern crate rshyper_hmap as hmap;

use core::hint::black_box;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use hashbrown::DefaultHashBuilder;
use hmap::UnHyperMap;
use rshyper::VertexSet;
use rshyper::idx::{EdgeId, VertexId};
use std::collections::VecDeque;

type G = UnHyperMap<usize, usize>;

/// the fixed-seed LCG of the #120 witness generator
struct Lcg(u64);

impl Lcg {
    fn new() -> Self {
        Self(0x5110_2026)
    }

    fn below(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) % n as u64) as usize
    }
}

/// member positions for a W10k-shaped graph: `edges` edges of 2..=4 members over `edges / 3`
/// vertices (30,000 edges is exactly W10k)
fn shape(edges: usize) -> (usize, Vec<Vec<usize>>) {
    let nodes = edges / 3;
    let mut rng = Lcg::new();
    let domains = (0..edges)
        .map(|_| {
            let k = 2 + rng.below(3);
            (0..k).map(|_| rng.below(nodes)).collect()
        })
        .collect();
    (nodes, domains)
}

fn vertices(g: &mut G, nodes: usize) -> Vec<VertexId<usize>> {
    (0..nodes)
        .map(|_| g.add_vertex().expect("add_vertex"))
        .collect()
}

fn build(edges: usize) -> (G, Vec<VertexId<usize>>) {
    let (nodes, domains) = shape(edges);
    let mut g = G::undirected();
    let v = vertices(&mut g, nodes);
    for d in &domains {
        g.add_link(d.iter().map(|&i| v[i])).expect("add_link");
    }
    (g, v)
}

/// `find_edges_with_node` as it was before the index: a lazy filter over every edge
fn old_find_edges_with_node(g: &G, v: &VertexId<usize>) -> usize {
    g.edges()
        .iter()
        .filter(|(_, edge)| edge.contains(v))
        .count()
}

/// `find_node_neighbors` as it was before the index
fn old_find_node_neighbors(g: &G, v: &VertexId<usize>) -> usize {
    let mut neighbors = VertexSet::<usize, DefaultHashBuilder>::default();
    g.facets().for_each(|edge| {
        if edge.contains(v) {
            neighbors.extend(edge.domain().iter().filter(|&u| u != v).cloned());
        }
    });
    neighbors.len()
}

/// the same graph with its index marked stale, so reads scan every edge
fn scanning(g: &G) -> G {
    let mut scan = g.clone();
    scan.edges_mut();
    scan
}

/// queries and BFS at 10²–10⁵ edges, index against scan
fn bench_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("incidence/query");
    group.sample_size(10);
    for edges in [100, 1_000, 10_000, 30_000, 100_000] {
        let (indexed, v) = build(edges);
        let scan = scanning(&indexed);
        let probes = (0..100).map(|i| v[i * v.len() / 100]).collect::<Vec<_>>();
        for (mode, g) in [("index", &indexed), ("scan", &scan)] {
            group.throughput(Throughput::Elements(probes.len() as u64));
            group.bench_with_input(
                BenchmarkId::new(format!("find_edges_with_node/{mode}"), edges),
                &probes,
                |b, probes| {
                    b.iter(|| {
                        probes
                            .iter()
                            .map(|x| g.find_edges_with_node(black_box(x)).count())
                            .sum::<usize>()
                    })
                },
            );
            group.bench_with_input(
                BenchmarkId::new(format!("find_node_neighbors/{mode}"), edges),
                &probes,
                |b, probes| {
                    b.iter(|| {
                        probes
                            .iter()
                            .map(|x| g.find_node_neighbors(black_box(x)).unwrap().len())
                            .sum::<usize>()
                    })
                },
            );
            group.throughput(Throughput::Elements(1));
            group.bench_with_input(
                BenchmarkId::new(format!("bft/{mode}"), edges),
                &v[0],
                |b, s| b.iter(|| g.bft().search(black_box(*s)).unwrap().len()),
            );
        }
        group.throughput(Throughput::Elements(probes.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("find_edges_with_node/old_scan", edges),
            &probes,
            |b, probes| {
                b.iter(|| {
                    probes
                        .iter()
                        .map(|x| old_find_edges_with_node(&indexed, black_box(x)))
                        .sum::<usize>()
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("find_node_neighbors/old_scan", edges),
            &probes,
            |b, probes| {
                b.iter(|| {
                    probes
                        .iter()
                        .map(|x| old_find_node_neighbors(&indexed, black_box(x)))
                        .sum::<usize>()
                })
            },
        );
    }
    group.finish();
}

/// a hub vertex of degree 10⁴ (like Silo's tag nodes) on top of W10k
fn bench_hub(c: &mut Criterion) {
    let mut group = c.benchmark_group("incidence/hub");
    group.sample_size(10);
    let (mut g, v) = build(30_000);
    let hub = g.add_vertex().expect("add_vertex");
    let mut spokes = (0..10_000)
        .map(|i| g.add_link([hub, v[i]]).expect("add_link"))
        .collect::<VecDeque<EdgeId<usize>>>();
    let scan = scanning(&g);
    for (mode, g) in [("index", &g), ("scan", &scan)] {
        group.bench_function(format!("find_edges_with_node/{mode}"), |b| {
            b.iter(|| g.find_edges_with_node(black_box(&hub)).count())
        });
        group.bench_function(format!("find_node_neighbors/{mode}"), |b| {
            b.iter(|| g.find_node_neighbors(black_box(&hub)).unwrap().len())
        });
    }
    group.bench_function("find_edges_with_node/old_scan", |b| {
        b.iter(|| old_find_edges_with_node(&g, black_box(&hub)))
    });
    group.bench_function("find_node_neighbors/old_scan", |b| {
        b.iter(|| old_find_node_neighbors(&g, black_box(&hub)))
    });
    // removing the oldest spoke shifts the whole hub list; the edge history adds O(h)
    let mut next = 0;
    group.bench_function("replace_oldest_spoke/index", |b| {
        b.iter(|| {
            let oldest = spokes.pop_front().expect("spoke");
            g.remove_edge(black_box(&oldest)).expect("remove_edge");
            next = (next + 1) % v.len();
            spokes.push_back(g.add_link([hub, v[next]]).expect("add_link"));
        })
    });
    group.finish();
}

/// insert and remove throughput at 12.5k–100k edges
fn bench_churn(c: &mut Criterion) {
    let mut group = c.benchmark_group("incidence/churn");
    group.sample_size(10);
    for edges in [12_500, 25_000, 50_000, 100_000] {
        let (nodes, domains) = shape(edges);
        group.throughput(Throughput::Elements(edges as u64));
        group.bench_with_input(BenchmarkId::new("insert", edges), &domains, |b, domains| {
            b.iter_batched(
                || {
                    let mut g = G::undirected();
                    let v = vertices(&mut g, nodes);
                    (g, v)
                },
                |(mut g, v)| {
                    for d in domains {
                        g.add_link(d.iter().map(|&i| v[i])).expect("add_link");
                    }
                    g
                },
                BatchSize::LargeInput,
            )
        });
        let (full, _) = build(edges);
        let mut order = full.iter_edge_keys().copied().collect::<Vec<_>>();
        order.sort_unstable();
        let mut rng = Lcg::new();
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i + 1));
        }
        group.bench_with_input(BenchmarkId::new("remove", edges), &order, |b, order| {
            b.iter_batched(
                || full.clone(),
                |mut g| {
                    for e in order {
                        g.remove_edge(e).expect("remove_edge");
                    }
                    g
                },
                BatchSize::LargeInput,
            )
        });
    }
    group.finish();
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_queries, bench_hub, bench_churn
}

criterion::criterion_main! {
    benches
}
