// --- Day 8: Playground ---

use rust_tools::union_find::UnionFind;

pub type Input = Vec<Point>;

type Point = (usize, usize, usize);

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|n| n.trim().parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    let mut uf = create_uf(input);
    let pairs = compute_sorted_pairs(input);

    for (_, p1, p2) in pairs.iter().take(1000) {
        uf.union_sets(p1, p2);
    }

    let mut sizes = uf.get_set_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let result: usize = sizes.iter().take(3).product();

    result.to_string()
}

pub fn part2(input: &Input) -> String {
    let mut uf = create_uf(input);
    let pairs = compute_sorted_pairs(input);

    for (_, p1, p2) in pairs {
        let set_a = uf.find_set(&p1).unwrap();
        let set_b = uf.find_set(&p2).unwrap();

        if set_a != set_b {
            uf.union_sets(&p1, &p2);
        }

        if uf.get_set_sizes().len() == 1 {
            return (p1.0 * p2.0).to_string();
        }
    }

    "".to_string()
}

fn create_uf(input: &Input) -> UnionFind<Point> {
    let mut uf = UnionFind::new();
    for &p in input {
        uf.make_set(p);
    }
    uf
}

fn compute_sorted_pairs(input: &Input) -> Vec<(f64, Point, Point)> {
    // Pre-compute all pairs with their distances
    let mut pairs: Vec<(f64, Point, Point)> = Vec::new();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let d = dist(input[i], input[j]);
            pairs.push((d, input[i], input[j]));
        }
    }
    // and sort them by distance (smallest first)
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    pairs
}

// https://en.wikipedia.org/wiki/Euclidean_distance#Higher_dimensions
fn dist(p1: Point, p2: Point) -> f64 {
    (((p1.0 as isize - p2.0 as isize).pow(2)
        + (p1.1 as isize - p2.1 as isize).pow(2)
        + (p1.2 as isize - p2.2 as isize).pow(2)) as f64)
        .sqrt()
}
