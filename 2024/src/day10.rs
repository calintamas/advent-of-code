use crate::{advent_day::AdventDay, grid::Grid, point::Point};

pub struct Day10 {
    grid: Grid<u32>,
}

impl Day10 {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
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
    }

    fn p1(&self) -> String {
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

        let mut scores: Vec<usize> = Vec::new();

        for trailhead in trailheads {
            let path = self.grid.bfs(trailhead, |current, neighbor| {
                let current_value = self
                    .grid
                    .get(current.x as usize, current.y as usize)
                    .unwrap();
                let neighbor_value = self
                    .grid
                    .get(neighbor.x as usize, neighbor.y as usize)
                    .unwrap();
                return current_value + 1 == *neighbor_value; // gradual, uphill slope; increase by 1 at every step
            });
            let summits: Vec<Point> = path
                .into_iter()
                .filter(|p| {
                    if let Some(val) = self.grid.get(p.x as usize, p.y as usize) {
                        return val == &9;
                    }
                    return false;
                })
                .collect();
            scores.push(summits.len());
        }

        scores
            .iter()
            .fold(0, |mut acc, x| {
                acc += x;
                acc
            })
            .to_string()
    }

    fn p2(&self) -> String {
        "".to_string()
    }
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
