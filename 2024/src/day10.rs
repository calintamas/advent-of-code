use crate::{advent_day::AdventDay, grid::Grid, point::Point};

pub struct Day10 {
    grid: Grid<u32>,
    trailheads: Vec<Point>,
}

impl Day10 {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
            trailheads: Vec::new(),
        }
    }
}

impl AdventDay for Day10 {
    fn parse(&mut self, input: &str) {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();

        let mut grid = Grid::<u32>::new(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.chars().enumerate() {
                grid.set(row, col, value.to_digit(10).unwrap());
            }
        }
        self.grid = grid;

        let mut trailheads: Vec<Point> = Vec::new();
        for x in 0..self.grid.rows {
            for y in 0..self.grid.cols {
                if self.grid.get(x, y) == Some(&0) {
                    trailheads.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        self.trailheads = trailheads;
    }

    fn p1(&self) -> String {
        compute_score(&self.grid, &self.trailheads, true).to_string()
    }

    fn p2(&self) -> String {
        compute_score(&self.grid, &self.trailheads, false).to_string()
    }
}

fn compute_score(grid: &Grid<u32>, trailheads: &Vec<Point>, distinct_trails: bool) -> usize {
    let mut sum = 0;

    for &trailhead in trailheads.iter() {
        let path = grid.bfs(trailhead, |current, neighbor, visited| {
            if distinct_trails && visited.contains(&neighbor) {
                return false;
            }
            let current_value = grid.get(current.x as usize, current.y as usize).unwrap();
            let neighbor_value = grid.get(neighbor.x as usize, neighbor.y as usize).unwrap();
            return current_value + 1 == *neighbor_value; // gradual, uphill slope; increase by 1 at every step
        });

        let trails: Vec<Point> = path
            .into_iter()
            .filter(|p| {
                if let Some(val) = grid.get(p.x as usize, p.y as usize) {
                    return val == &9;
                }
                return false;
            })
            .collect();
        sum += trails.len();
    }

    return sum;
}

#[test]
fn test_d10_p1() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let mut day = Day10::new();
    day.parse(input);
    assert_eq!(day.p1(), "36");
}

#[test]
fn test_d10_p2() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    let mut day = Day10::new();
    day.parse(input);
    assert_eq!(day.p2(), "81");
}
