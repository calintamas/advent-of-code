use std::collections::{HashMap, HashSet};

use crate::{advent_day::AdventDay, grid::Grid, point::Point};

pub struct Day08 {
    grid: Grid<char>,
    antennas: HashMap<char, Vec<Point>>,
}

impl Day08 {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
            antennas: HashMap::new(),
        }
    }
}

impl AdventDay for Day08 {
    fn parse(&mut self, input: &str) {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();

        self.grid = Grid::<char>::new(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.chars().enumerate() {
                self.grid.set(row, col, value);
                if value != '.' {
                    let point = Point::new(row as i32, col as i32);
                    self.antennas.entry(value).or_insert(Vec::new()).push(point);
                }
            }
        }
    }

    fn p1(&self) -> String {
        let mut antinodes: HashSet<Point> = HashSet::new();
        for antennas in self.antennas.values() {
            // Take every combination of 2 antennas of the same frequency
            // and attempt to place an antinode at "offset" distance from second antenna.
            for &first in antennas {
                for &second in antennas {
                    if first != second {
                        let offset = second - first; // directional difference between the two points
                        let antinode = second + offset;

                        if self
                            .grid
                            .get(antinode.x as usize, antinode.y as usize)
                            .is_some()
                        {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }
        antinodes.len().to_string()
    }

    fn p2(&self) -> String {
        let mut antinodes: HashSet<Point> = HashSet::new();
        for antennas in self.antennas.values() {
            // Same as above, but do it repeatedly until out-of-grid (to account for resonance)
            for &first in antennas {
                for &second in antennas {
                    if first != second {
                        let offset = second - first;
                        let mut antinode = second;

                        while self
                            .grid
                            .get(antinode.x as usize, antinode.y as usize)
                            .is_some()
                        {
                            antinodes.insert(antinode);
                            antinode += offset;
                        }
                    }
                }
            }
        }
        antinodes.len().to_string()
    }
}

#[test]
fn test_p1() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let mut day = Day08::new();
    day.parse(input);
    assert_eq!(day.p1(), "14");
}

#[test]
fn test_p2() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let mut day = Day08::new();
    day.parse(input);
    assert_eq!(day.p2(), "34");
}
