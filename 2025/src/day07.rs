// --- Day 7: Laboratories ---

use std::collections::{HashMap, HashSet, VecDeque};

use rust_tools::grid::Grid;

pub type Input = Grid<char>;

type Point = (usize, usize);

pub fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let mut grid = Grid::<char>::new(rows, cols);

    for (row, line) in lines.iter().enumerate() {
        for (col, value) in line.chars().enumerate() {
            grid.set(row, col, value);
        }
    }

    grid
}

pub fn part1(grid: &Input) -> String {
    let start = find_start_point(grid);

    let mut beams: VecDeque<Point> = VecDeque::new();
    beams.push_back(start);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut count = 0;
    while let Some((row, col)) = beams.pop_front() {
        if let Some(char) = grid.get(row, col) {
            if char == &'.' || char == &'S' {
                split_beam(&mut beams, &mut visited, (row + 1, col));
            } else if char == &'^' {
                count += 1;
                split_beam(&mut beams, &mut visited, (row + 1, col - 1));
                split_beam(&mut beams, &mut visited, (row + 1, col + 1));
            }
        }
    }

    count.to_string()
}

pub fn part2(grid: &Input) -> String {
    let mut cache: HashMap<Point, usize> = HashMap::new();
    let start = find_start_point(grid);
    let count = start_journey(grid, start, &mut cache);
    count.to_string()
}

fn start_journey(grid: &Input, (row, col): Point, cache: &mut HashMap<Point, usize>) -> usize {
    if let Some(&result) = cache.get(&(row, col)) {
        return result;
    }

    let mut result = 1;
    if let Some(char) = grid.get(row, col) {
        if char == &'.' || char == &'S' {
            result = start_journey(grid, (row + 1, col), cache);
        } else if char == &'^' {
            result = start_journey(grid, (row + 1, col - 1), cache)
                + start_journey(grid, (row + 1, col + 1), cache);
        }
    }

    cache.insert((row, col), result);
    result
}

fn split_beam(beams: &mut VecDeque<Point>, visited: &mut HashSet<Point>, beam: Point) {
    if visited.contains(&beam) {
        return;
    }
    beams.push_back(beam);
    visited.insert(beam);
}

fn find_start_point(grid: &Input) -> Point {
    let mut start: Point = (0, 0);
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.get(row, col).unwrap() == &'S' {
                start = (row, col);
                break;
            }
        }
    }
    start
}
