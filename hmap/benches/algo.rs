/*
    Appellation: hyper_map <bench>
    Contrib: @FL03
*/
extern crate rshyper_core as rshyper;
extern crate rshyper_hmap as hyper_map;

use self::ext::*;
use rshyper::prelude::VertexId;

use core::hint::black_box;
use criterion::{BatchSize, Criterion};

/// benchmarks for search algorithms in the `HyperMap` implementation.
fn bench_hypermap_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("HyperMap::search");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));
    // benchmark the breadth-first traversal search
    group.bench_with_input("bft", &VertexId::random_between(0..N), |b, &idx| {
        b.iter_batched(
            setup,
            |graph| {
                // get the degree of each nodelet id = n.into();
                // search the graph for some target vertex
                graph.bft().search(black_box(idx)).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
    // benchmark the depth-first traversal search
    group.bench_with_input("dft", &VertexId::random_between(0..N), |b, &idx| {
        b.iter_batched(
            setup,
            |graph| {
                // search the graph for some target vertex
                graph.dft().search(black_box(idx)).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
    // finish the group
    group.finish();
}

/// benchmarks for search algorithms in the [`HyperMap`] implementation.
fn bench_hypermap_path_finders(c: &mut Criterion) {
    // a dummy hueristic function that returns a constant value
    pub fn hue<T>(_a: VertexId, _b: VertexId) -> T
    where
        T: num_traits::One,
    {
        // a dummy heuristic function that returns a constant value
        T::one()
    }
    let mut group = c.benchmark_group("HyperMap::pathfinder");
    // set the sample size for the group
    group.sample_size(SAMPLES);
    // set the duration for the measurement
    group.measurement_time(std::time::Duration::from_secs(DURATION));

    // benchmark the a* search
    group.bench_with_input(
        "a8",
        &(VertexId::zero(), VertexId::random_between(0..N)),
        |b, &(src, tgt)| {
            b.iter_batched(
                setup_paths,
                |graph| {
                    // use the a8 algorithm to find a path between two vertices
                    graph
                        .astar(hue)
                        .find_path(black_box(src), black_box(tgt))
                        .expect("failed to find path");
                },
                BatchSize::SmallInput,
            )
        },
    );
    // benchmark the dijkstra path-finding algorithm
    group.bench_with_input(
        "dijkstra",
        &(VertexId::zero(), VertexId::random_between(0..N)),
        |b, &(src, tgt)| {
            b.iter_batched(
                setup_paths,
                |graph| {
                    // use the dijkstra algorithm to find a path between two vertices
                    graph
                        .dijkstra()
                        .find_path(black_box(src), black_box(tgt))
                        .expect("failed to find path");
                },
                BatchSize::SmallInput,
            )
        },
    );
    // finish the group
    group.finish();
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(SAMPLES).measurement_time(std::time::Duration::from_secs(DURATION)).with_plots().with_output_color(true);
    targets = bench_hypermap_search, bench_hypermap_path_finders
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
    /// a type alias for the type of weight used to benchmark the [`HyperMap`]
    pub type Wt = i128;

    /// initialize a new [`HyperMap`] with a predefined structure
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
            .add_edge([v0, v1, v2, v3, v5], Weight::one())
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

    /// [`setup`] with every edge weight mapped into `0..100`: Dijkstra rejects negative
    /// weights, and small ones keep path costs far from overflow
    pub fn setup_paths() -> HyperMap<Wt, Wt> {
        let mut graph = setup();
        let ids = graph.edges().keys().copied().collect::<Vec<_>>();
        for id in ids {
            let w = graph.get_edge_weight_mut(&id).expect("missing edge");
            **w = w.rem_euclid(100);
        }
        graph
    }
}
