// --- Day 9: Movie Theater ---

use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::zip,
};

use rust_tools::grid::Grid;

pub type Input = Vec<Point>;

// (col, row) in input
type Point = (usize, usize);

pub fn parse(input: &str) -> Input {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|n| n.trim().parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    points
}

pub fn part1(points: &Input) -> String {
    let pairs = compute_sorted_pairs(points.clone());
    let max_area = pairs[0].0;
    max_area.to_string()
}

pub fn part2(points: &Input) -> String {
    let mut xs: Vec<usize> = points
        .iter()
        .map(|p| p.0)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    xs.sort();
    let mut ys: Vec<usize> = points
        .iter()
        .map(|p| p.1)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    ys.sort();

    // Adding +1 to rows and cols to pad the grid on the outside
    // to be able to flood-fill later.
    let mut grid: Grid<char> = Grid::new(ys.len() * 2 + 1, xs.len() * 2 + 1);
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            grid.set(row, col, '.');
        }
    }

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()]; // Wrap around to connect last to first

        let cx1 = xs.iter().position(|c| *c == x1).unwrap() * 2 + 1;
        let cx2 = xs.iter().position(|c| *c == x2).unwrap() * 2 + 1;
        let cy1 = ys.iter().position(|c| *c == y1).unwrap() * 2 + 1;
        let cy2 = ys.iter().position(|c| *c == y2).unwrap() * 2 + 1;

        let (cx1, cx2) = (cx1.min(cx2), cx1.max(cx2)); // Sort each pair, so we can iterate
        let (cy1, cy2) = (cy1.min(cy2), cy1.max(cy2));

        for cx in cx1..(cx2 + 1) {
            for cy in cy1..(cy2 + 1) {
                grid.set(cy, cx, 'x');
            }
        }
    }

    // Flood-fill on the outside of the shape
    let start: Point = (0, 0);
    let mut outside_points = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(start);
    outside_points.insert(start);

    while let Some(p) = queue.pop_front() {
        grid.for_each_neighbor_of(p, |val, (row, col)| {
            if let Some(val) = val {
                if val == &'x' || outside_points.contains(&(row, col)) {
                    //
                } else {
                    outside_points.insert((row, col));
                    queue.push_back((row, col));
                }
            }
        });
    }

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if !outside_points.contains(&(row, col)) {
                grid.set(row, col, 'x');
            }
        }
    }

    let pairs = compute_sorted_pairs(points.clone());

    let mut max = 0;
    for pair in pairs.iter() {
        let (x1, y1) = pair.1;
        let (x2, y2) = pair.2;
        let cx1 = xs.iter().position(|c| *c == x1).unwrap() * 2 + 1;
        let cx2 = xs.iter().position(|c| *c == x2).unwrap() * 2 + 1;
        let cy1 = ys.iter().position(|c| *c == y1).unwrap() * 2 + 1;
        let cy2 = ys.iter().position(|c| *c == y2).unwrap() * 2 + 1;

        let (cx1, cx2) = (cx1.min(cx2), cx1.max(cx2)); // Sort each pair, so we can iterate
        let (cy1, cy2) = (cy1.min(cy2), cy1.max(cy2));

        let mut valid = true;
        for cx in cx1..(cx2 + 1) {
            for cy in cy1..(cy2 + 1) {
                if grid.get(cy, cx).unwrap() == &'.' {
                    valid = false;
                    continue;
                }
            }
        }

        if valid {
            max = pair.0;
            break;
        }
    }

    max.to_string()
}

fn compute_sorted_pairs(points: Vec<Point>) -> Vec<(usize, Point, Point)> {
    // Pre-compute all pairs with their areas
    let mut pairs = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let l = (p1.0).abs_diff(p2.0) + 1;
            let w = (p1.1).abs_diff(p2.1) + 1;
            let area = l * w;
            let pair = (area, p1, p2);
            pairs.push(pair);
        }
    }
    // and sort them by area (highest first)
    pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    pairs
}
