use crate::days::Day;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Distance {
    pub units: f64,
}

impl Distance {
    pub fn new(units: f64) -> Distance {
        Distance { units }
    }
}

// Implement Total Ordering for sorting
impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.units == other.units
    }
}
impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use total_cmp for safe float sorting
        self.units.total_cmp(&other.units)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Copy)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn from_str(s: &str) -> Point {
        let coords: Vec<i64> = s
            .trim()
            .split(',')
            .map(|val| val.parse().expect("Error parsing coordinate integer"))
            .collect();
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    pub fn dist(&self, other: &Point) -> Distance {
        let d =
            (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
                as f64)
                .sqrt();
        Distance::new(d)
    }
}

struct Edge {
    p1_idx: usize,
    p2_idx: usize,
    dist: Distance,
}

// --- UNION FIND ---

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(count: usize) -> UnionFind {
        UnionFind {
            parent: (0..count).collect(),
            size: vec![1; count],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}

fn part1(data: Vec<String>) -> u64 {
    if data.is_empty() {
        println!("ERROR: Input data is empty!");
        return 0;
    }

    let points: Vec<Point> = data.into_iter().map(|s| Point::from_str(&s)).collect();
    let n = points.len();

    let mut edges: Vec<Edge> = Vec::with_capacity(n * n / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                p1_idx: i,
                p2_idx: j,
                dist: points[i].dist(&points[j]),
            });
        }
    }

    edges.sort_by(|a, b| a.dist.cmp(&b.dist));

    let mut uf = UnionFind::new(n);

    let limit = if cfg!(test) { 10 } else { 1000 };

    let effective_limit = limit.min(edges.len());
    for edge in edges.iter().take(effective_limit) {
        uf.union(edge.p1_idx, edge.p2_idx);
    }

    let mut circuit_sizes: Vec<usize> = Vec::new();
    let mut seen_roots = HashSet::new();

    for i in 0..n {
        let root = uf.find(i);
        if !seen_roots.contains(&root) {
            circuit_sizes.push(uf.size[root]);
            seen_roots.insert(root);
        }
    }

    circuit_sizes.sort_by(|a, b| b.cmp(a));

    circuit_sizes.iter().take(3).product::<usize>() as u64
}

fn part2(_data: Vec<String>) -> u64 {
    0
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
