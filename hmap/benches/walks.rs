/*
    Appellation: walks <bench>
    Contrib: @FL03
*/
//! #124: `shortest_walks` on S110k, the shape of Silo's vault index: 10,000 notes, each with 10
//! link edges and one tag edge to one of 50 tags. N0 → N5000, depth ≤ 8, tag edges rejected.
//! Informational; Silo's engine expands 1,564 vertices for this query. Run with
//! `CARGO_PROFILE_BENCH_OPT_LEVEL=3`.
extern crate rshyper_algo as algo;
extern crate rshyper_core as rshyper;
extern crate rshyper_hmap as hmap;

use algo::PathLimits;
use core::hint::black_box;
use criterion::Criterion;
use hmap::UnHyperMap;
use rshyper::Weight;
use rshyper::idx::VertexId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Link,
    Tag,
}

type G = UnHyperMap<(), Kind>;

const NOTES: usize = 10_000;

fn s110k() -> (G, Vec<VertexId<usize>>) {
    let mut g = G::undirected();
    let notes = (0..NOTES)
        .map(|_| g.add_vertex().expect("add_vertex"))
        .collect::<Vec<_>>();
    let tags = (0..50)
        .map(|_| g.add_vertex().expect("add_vertex"))
        .collect::<Vec<_>>();
    for i in 0..NOTES {
        for j in 0..10 {
            let t = (i * 7 + j * 13 + 1) % NOTES;
            g.add_edge([notes[i], notes[t]], Weight(Kind::Link))
                .expect("add_edge");
        }
        g.add_edge([notes[i], tags[i % 50]], Weight(Kind::Tag))
            .expect("add_edge");
    }
    (g, notes)
}

fn bench_s110k(c: &mut Criterion) {
    let (g, n) = s110k();
    let limits = PathLimits {
        max_depth: 8,
        max_expansions: 20_000,
        max_paths: 10,
    };
    let query = || {
        g.bft()
            .shortest_walks(n[0], n[5000], limits, |_, k| *k != Kind::Tag, |_, _| true)
            .expect("shortest_walks")
    };
    let found = query();
    assert!(found.complete && !found.truncated);
    assert_eq!(found.walks.len(), 6);
    assert!(found.walks.iter().all(|w| w.edges.len() == 4));
    eprintln!(
        "S110k N0 -> N5000: {} walks of 4 hops, {} expansions",
        found.walks.len(),
        found.expansions
    );
    let mut group = c.benchmark_group("walks");
    group.sample_size(10);
    group.bench_function("s110k/N0-N5000", |b| b.iter(|| black_box(query())));
    group.finish();
}

criterion::criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench_s110k
}

criterion::criterion_main! {
    benches
}
