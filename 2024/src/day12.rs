use std::collections::HashSet;

use crate::{advent_day::AdventDay, grid::Grid, point::Point};

pub struct Day12 {
    grid: Grid<char>,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
        }
    }
}

impl AdventDay for Day12 {
    fn parse(&mut self, input: &str) {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();

        let mut grid = Grid::<char>::new(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.chars().enumerate() {
                grid.set(row, col, value);
            }
        }
        self.grid = grid;
    }

    fn p1(&self) -> String {
        let directions = [
            (0, 1),  // Right
            (0, -1), // Left
            (1, 0),  // Down
            (-1, 0), // Up
        ];

        let mut total = 0;
        let mut seen: HashSet<Point> = HashSet::new();

        for x in 0..self.grid.rows {
            for y in 0..self.grid.cols {
                let start = Point::new(x as i32, y as i32);

                if seen.contains(&start) {
                    continue;
                }

                let plant_type = self.grid.get(x, y).unwrap();

                // flood fill
                // to find all points belonging to parcel with plant_type
                let parcel = self.grid.bfs(start, |current, neighbor, visited| {
                    let neighbor_plant_type = self
                        .grid
                        .get(neighbor.x as usize, neighbor.y as usize)
                        .unwrap();
                    let res = neighbor_plant_type == plant_type && !visited.contains(&neighbor);
                    if res {
                        seen.insert(current);
                        seen.insert(neighbor);
                    }
                    res
                });

                let area = parcel.len();
                let mut perimeter = 0;

                for &point in parcel.iter() {
                    for &(dx, dy) in &directions {
                        let neighbor = point + Point::new(dx, dy);
                        if let Some(value) = self.grid.get(neighbor.x as usize, neighbor.y as usize)
                        {
                            if value != plant_type {
                                // at parcel edge
                                perimeter += 1;
                            }
                        } else {
                            // at grid edge
                            perimeter += 1
                        }
                    }
                }

                total += area * perimeter;
            }
        }

        total.to_string()
    }

    fn p2(&self) -> String {
        "".to_string()
    }
}

#[test]
fn test_d12_p1() {
    let mut day = Day12::new();

    let input1 = "AAAA
BBCD
BBCC
EEEC";
    day.parse(input1);
    assert_eq!(day.p1(), "140");

    let input2 = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    day.parse(input2);
    assert_eq!(day.p1(), "772");

    let input3 = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    day.parse(input3);
    assert_eq!(day.p1(), "1930");
}
