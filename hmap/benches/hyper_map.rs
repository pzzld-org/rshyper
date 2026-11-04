/*
    Appellation: hyper_map <bench>
    Contrib: @FL03
*/
extern crate rshyper_core as rshyper;
extern crate rshyper_hmap as hyper_map;

use self::ext::*;
use hyper_map::HyperMap;
use rshyper::prelude::{VertexId, Weight, generate_random_edge};

use core::hint::black_box;
use criterion::{BatchSize, Criterion};

/// benchmark various iterators provided by the `HyperMap` implementation.
fn bench_hypermap(c: &mut Criterion) {
    let mut group = c.benchmark_group("HyperMap::iter");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the iterator for edges
    group.bench_with_input("iter_edges", &N, |b, &n| {
        b.iter_batched(
            setup,
            |graph| {
                // iterate over the edges of the graph, taking `n` edges
                graph.iter_edges().take(black_box(n)).for_each(|v| {
                    // black box the entry to prevent optimizations
                    black_box(v);
                });
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the sequential iterator for edges
    group.bench_with_input("seq_iter_edges", &N, |b, &n| {
        b.iter_batched(
            setup,
            |graph| {
                // iterate sequentially over the edges of the graph, taking `n` edges
                graph.seq_iter_edges().take(black_box(n)).for_each(|v| {
                    // black box the entry to prevent optimizations
                    black_box(v);
                });
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the iterator for nodes
    group.bench_with_input("iter_nodes", &N, |b, &n| {
        b.iter_batched(
            setup,
            |graph| {
                // iterate over the edges of the graph, taking `n` nodes
                graph.iter_nodes().take(black_box(n)).for_each(|v| {
                    black_box(v);
                })
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the sequential iterator for edges
    group.bench_with_input("seq_iter_nodes", &N, |b, &n| {
        b.iter_batched(
            setup,
            |graph| {
                // iterate over the edges of the graph, taking `n` nodes
                graph.seq_iter_nodes().take(black_box(n)).for_each(|v| {
                    black_box(v);
                })
            },
            BatchSize::SmallInput,
        )
    });
    // finish the group
    group.finish();
}

/// benchmark various edge operations on the `HyperMap` implementation.
fn bench_hypermap_edges(c: &mut Criterion) {
    let mut group = c.benchmark_group("HyperMap::edges");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the `add_edge` function
    group.bench_with_input("add_edge", &generate_random_edge::<Wt>(N), |b, input| {
        b.iter_batched(
            setup,
            |mut graph| {
                let (verts, weight) = input.clone();
                // get the edge with the given id
                graph
                    .add_edge(verts, black_box(weight))
                    .expect("failed to add edge");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `get_edge` function
    group.bench_with_input("get_edge", &rand::random_range(0..N), |b, idx| {
        b.iter_batched(
            setup,
            |graph| {
                // get the edge with the given id
                let _ = graph.get_edge(black_box(idx)).expect("failed to get edge");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `remove_edge` function
    group.bench_with_input("remove_edge", &rand::random_range(0..N), |b, idx| {
        b.iter_batched(
            setup,
            |graph| {
                // get the edge with the given id
                let _ = graph.get_edge(black_box(idx)).expect("failed to get edge");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `find_edges_with_node` function
    group.bench_with_input(
        "find_edges_with_node",
        &VertexId::<usize>::random_between(0..N),
        |b, idx| {
            b.iter_batched(
                setup,
                |graph| {
                    // find the edges that contain the given vertex id
                    let iter = graph.find_edges_with_node(black_box(idx));
                    // consume the iterator to force the computation
                    black_box(iter).count()
                },
                BatchSize::SmallInput,
            )
        },
    );
    // finish the group
    group.finish();
}

/// benchmark for the `HyperMap` implementation.
fn bench_hypermap_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("HyperMap::nodes");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the `add_nodes` function
    group.bench_function("add_nodes", |b| {
        b.iter_batched(
            HyperMap::<Wt, Wt>::new,
            |mut graph| {
                let _ = graph.add_nodes(black_box(0..(N as Wt)));
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `get_node_degree` function
    group.bench_function("get_node_degree", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let idx = VertexId::random_between(0..N).map(|i| i % N);
                // get the degree of each node
                graph.get_node_degree(black_box(&idx));
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `find_node_neighbors` function
    group.bench_function("find_node_neighbors", |b| {
        b.iter_batched(
            setup,
            |graph| {
                let idx = VertexId::random_between(0..N).map(|i| i % N);
                // get the degree of each node
                graph
                    .find_node_neighbors(black_box(&idx))
                    .expect("failed to find node neighbors");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `add_node` function
    group.bench_function("add_vertex", |b| {
        b.iter_batched(
            setup,
            |mut graph| {
                // add a vertex to the graph
                let _ = graph.add_vertex();
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `add_node` function
    group.bench_with_input("add_node", &rand::random::<Wt>(), |b, &w| {
        b.iter_batched(
            setup,
            |mut graph| {
                // add the node with the weight
                graph
                    .add_node(black_box(Weight(w)))
                    .expect("failed to add node");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `get_node` function
    group.bench_with_input("get_node", &rand::random_range(0..N), |b, idx| {
        b.iter_batched(
            setup,
            |graph| {
                // add the node with the weight
                graph.get_node(black_box(idx)).expect("failed to add node");
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the `remove_node` function
    group.bench_with_input("remove_node", &rand::random_range(0..N), |b, idx| {
        b.iter_batched(
            setup,
            |mut graph| {
                // remove the node with the given id
                graph
                    .remove_node(black_box(idx))
                    .expect("failed to remove node");
            },
            BatchSize::SmallInput,
        )
    });
    // finish the group
    group.finish();
}

/// building a graph vertex by vertex must stay near linear (#114): every `add_vertex` used to
/// scan the whole node history.
fn bench_hypermap_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("HyperMap::build");
    group.sample_size(10);
    for n in [20_000usize, 40_000, 80_000] {
        group.bench_with_input(criterion::BenchmarkId::new("add_vertex", n), &n, |b, &n| {
            b.iter_batched(
                HyperMap::<(), ()>::new,
                |mut graph| {
                    for _ in 0..n {
                        black_box(graph.add_vertex().expect("failed to add vertex"));
                    }
                    graph
                },
                BatchSize::LargeInput,
            )
        });
        group.bench_with_input(
            criterion::BenchmarkId::new("hashbrown_insert_baseline", n),
            &n,
            |b, &n| {
                b.iter_batched(
                    || {
                        hashbrown::HashMap::<
                            rshyper::idx::VertexId<usize>,
                            rshyper::Node<(), usize>,
                            hashbrown::DefaultHashBuilder,
                        >::default()
                    },
                    |mut m| {
                        for i in 0..n {
                            let id = rshyper::idx::VertexId::new(i);
                            m.insert(id, rshyper::Node::new(id, ()));
                        }
                        m
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }
    group.finish();
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(SAMPLES).measurement_time(std::time::Duration::from_secs(DURATION)).with_plots().with_output_color(true);
    targets = bench_hypermap, bench_hypermap_edges, bench_hypermap_nodes, bench_hypermap_build
}

criterion::criterion_main! {
    benches
}

#[allow(unused_variables)]
#[cfg(feature = "rand")]
mod ext {
    use rshyper::prelude::{Weight, generate_random_edge};
    use rshyper_hmap::HyperMap;

    /// the duration, in seconds, for which the benchmarks should run
    pub const DURATION: u64 = 7;
    /// a constant for the sample size of a benchmark group
    pub const SAMPLES: usize = 50;
    /// the number of initialized nodes setup by the [`setup`] method
    pub const N: usize = 100;
    /// a type alias for the type of weight used to benchmark the `HyperMap`
    pub type Wt = i128;

    /// initialize a new `HyperMap` with a predefined structure
    pub fn setup() -> HyperMap<Wt, Wt> {
        // initialize a new undirected hash graph
        let mut graph = HyperMap::<Wt, Wt>::undirected();
        let v0 = graph.add_vertex().expect("failed to add vertex");
        let v1 = graph.add_vertex().expect("failed to add vertex");
        let v2 = graph.add_vertex().expect("failed to add vertex");
        let v3 = graph.add_vertex().expect("failed to add vertex");
        let v4 = graph.add_vertex().expect("failed to add vertex");
        let v5 = graph.add_vertex().expect("failed to add vertex");
        // add a few edges to the graph
        let e0 = graph
            .add_edge([v0, v1, v2, v3, v5], Weight(1))
            .expect("failed to add surface");
        let e1 = graph
            .add_edge([v1, v2, v3, v4], Weight(2))
            .expect("failed to add surface");
        let e2 = graph
            .add_edge([v2, v3, v4, v5], Weight(3))
            .expect("failed to add surface");
        let e3 = graph
            .add_edge([v0, v1], Weight(4))
            .expect("failed to add surface");
        // add 100 nodes to the graph
        let _ = graph.add_nodes(5..(N as Wt)).collect::<Vec<_>>();
        // add 100 edges to the graph
        for _ in graph.size()..graph.order() {
            // each edge contains between 2 and 100 vertices & a random weight
            let (verts, weight) = generate_random_edge::<Wt>(N);
            // add a self-loop to each vertex
            graph
                .add_edge(verts, weight)
                .expect("failed to add surface");
        }

        graph
    }
}
