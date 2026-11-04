/*
    appellation: shortest_path_oracle <test>
    authors: @FL03
*/
//! #122: Dijkstra and A* against brute-force distances on the undirected 2-section, over
//! 2,000 fixed-seed hypergraphs and every ordered pair of vertices.
use rshyper_core::{VertexId, Weight};
use rshyper_hmap::HyperMap;

const SEEDS: u64 = 2_000;
const INF: usize = usize::MAX / 4;

/// splitmix64, so the cases are fixed across platforms and runs
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: u64) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        (z ^ (z >> 31)) % n
    }
}

struct Case {
    graph: HyperMap<(), usize>,
    v: Vec<VertexId>,
    /// `weighted[a][b]`: cheapest path cost; `hops[a][b]`: fewest hyperedges
    weighted: Vec<Vec<usize>>,
    hops: Vec<Vec<usize>>,
    /// cheapest single hyperedge joining each pair, if any
    step: Vec<Vec<Option<usize>>>,
}

fn case(seed: u64) -> Case {
    let mut rng = Rng(seed);
    let n = 2 + rng.below(4) as usize;
    let mut graph = HyperMap::<(), usize>::undirected();
    let v: Vec<VertexId> = (0..n).map(|_| graph.add_vertex().unwrap()).collect();
    let mut step = vec![vec![None::<usize>; n]; n];
    for _ in 0..1 + rng.below(8) {
        let k = 2 + rng.below(2) as usize;
        let members: Vec<usize> = (0..k).map(|_| rng.below(n as u64) as usize).collect();
        let w = rng.below(11) as usize;
        graph
            .add_edge(members.iter().map(|&i| v[i]), Weight(w))
            .unwrap();
        for &a in &members {
            for &b in &members {
                if a != b {
                    step[a][b] = Some(step[a][b].map_or(w, |m: usize| m.min(w)));
                }
            }
        }
    }
    // Floyd–Warshall twice: once on weights, once on hop counts
    let close = |cost: &dyn Fn(usize) -> usize| {
        let mut d = vec![vec![INF; n]; n];
        for a in 0..n {
            d[a][a] = 0;
            for b in 0..n {
                if let Some(w) = step[a][b] {
                    d[a][b] = cost(w);
                }
            }
        }
        for m in 0..n {
            for a in 0..n {
                for b in 0..n {
                    d[a][b] = d[a][b].min(d[a][m] + d[m][b]);
                }
            }
        }
        d
    };
    let weighted = close(&|w| w);
    let hops = close(&|_| 1);
    Case {
        graph,
        v,
        weighted,
        hops,
        step,
    }
}

impl Case {
    fn index(&self, x: VertexId) -> usize {
        self.v.iter().position(|&y| y == x).unwrap()
    }
    /// checks that `path` walks real steps from `a` to `b` and returns its weighted cost and
    /// its hop count
    fn walk(&self, path: &[VertexId], a: usize, b: usize) -> (usize, usize) {
        assert_eq!(path.first(), Some(&self.v[a]));
        assert_eq!(path.last(), Some(&self.v[b]));
        let cost = path
            .windows(2)
            .map(|p| self.step[self.index(p[0])][self.index(p[1])].expect("not a step"))
            .sum();
        (cost, path.len() - 1)
    }
}

#[test]
fn dijkstra_matches_the_2_section_oracle() {
    for seed in 0..SEEDS {
        let c = case(seed);
        let n = c.v.len();
        for a in 0..n {
            for b in 0..n {
                let got = c.graph.dijkstra().find_path(c.v[a], c.v[b]);
                if c.weighted[a][b] >= INF {
                    assert!(got.is_err(), "seed {seed}: {a}->{b} should be unreachable");
                    continue;
                }
                let path = got.unwrap_or_else(|e| panic!("seed {seed}: {a}->{b}: {e}"));
                let (cost, _) = c.walk(&path, a, b);
                assert_eq!(cost, c.weighted[a][b], "seed {seed}: {a}->{b} via {path:?}");
            }
        }
    }
}

#[test]
fn astar_matches_the_hop_oracle() {
    for seed in 0..SEEDS {
        let c = case(seed);
        let n = c.v.len();
        for a in 0..n {
            for b in 0..n {
                let hop = |x: VertexId| {
                    let d = c.hops[c.index(x)][b];
                    // a finite stand-in keeps the heuristic valid and consistent off-component
                    if d >= INF { 1_000.0 } else { d as f64 }
                };
                let heuristics: [&dyn Fn(VertexId, VertexId) -> f64; 3] =
                    [&|_, _| 0.0, &|x, _| hop(x), &|x, _| (hop(x) / 2.0).floor()];
                for (i, h) in heuristics.into_iter().enumerate() {
                    let got = c.graph.astar(h).find_path(c.v[a], c.v[b]);
                    if c.hops[a][b] >= INF {
                        assert!(
                            got.is_err(),
                            "seed {seed} h{i}: {a}->{b} should be unreachable"
                        );
                        continue;
                    }
                    let path = got.unwrap_or_else(|e| panic!("seed {seed} h{i}: {a}->{b}: {e}"));
                    let (_, hops) = c.walk(&path, a, b);
                    assert_eq!(
                        hops, c.hops[a][b],
                        "seed {seed} h{i}: {a}->{b} via {path:?}"
                    );
                }
            }
        }
    }
}
