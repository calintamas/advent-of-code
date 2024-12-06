use std::collections::HashSet;
use std::hash::Hash;

use crate::{advent_day::AdventDay, grid::Grid};

#[derive(Debug, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Day06 {
    grid: Grid<char>,
    start_pos: Pos,
}

impl Day06 {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
            start_pos: Pos { x: 0, y: 0 },
        }
    }
}

impl AdventDay for Day06 {
    fn parse(&mut self, input: &str) {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();

        self.grid = Grid::<char>::new(rows, cols);

        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.chars().enumerate() {
                self.grid.set(row, col, value);
                if value == '^' {
                    self.start_pos = Pos { x: row, y: col };
                }
            }
        }
    }

    fn p1(&self) -> String {
        walk(self.grid.clone(), self.start_pos).to_string()
    }

    fn p2(&self) -> String {
        "".to_string()
    }
}

fn walk(mut grid: Grid<char>, start_pos: Pos) -> usize {
    let mut pos = start_pos;
    let mut direction = '^';
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(start_pos);

    while grid.get(pos.x, pos.y).is_some() {
        if direction == '^' {
            if (pos.x as isize) - 1 < 0 {
                break;
            }
            let new_pos = Pos {
                x: pos.x - 1,
                y: pos.y,
            };
            if let Some(&new_val) = grid.get(new_pos.x, new_pos.y) {
                if new_val == '#' {
                    direction = '>'
                } else {
                    grid.set(pos.x, pos.y, '.');
                    grid.set(new_pos.x, new_pos.y, direction);
                    pos = new_pos;
                    visited.insert(new_pos);
                }
            } else {
                break;
            }
        }

        if direction == '>' {
            let new_pos = Pos {
                x: pos.x,
                y: pos.y + 1,
            };
            if let Some(&new_val) = grid.get(new_pos.x, new_pos.y) {
                if new_val == '#' {
                    direction = 'v'
                } else {
                    grid.set(pos.x, pos.y, '.');
                    grid.set(new_pos.x, new_pos.y, direction);
                    pos = new_pos;
                    visited.insert(new_pos);
                }
            } else {
                break;
            }
        }

        if direction == 'v' {
            let new_pos = Pos {
                x: pos.x + 1,
                y: pos.y,
            };
            if let Some(&new_val) = grid.get(new_pos.x, new_pos.y) {
                if new_val == '#' {
                    direction = '<'
                } else {
                    grid.set(pos.x, pos.y, '.');
                    grid.set(new_pos.x, new_pos.y, direction);
                    pos = new_pos;
                    visited.insert(new_pos);
                }
            } else {
                break;
            }
        }

        if direction == '<' {
            if (pos.y as isize) - 1 < 0 {
                break;
            }
            let new_pos = Pos {
                x: pos.x,
                y: pos.y - 1,
            };
            if let Some(&new_val) = grid.get(new_pos.x, new_pos.y) {
                if new_val == '#' {
                    direction = '^'
                } else {
                    grid.set(pos.x, pos.y, '.');
                    grid.set(new_pos.x, new_pos.y, direction);
                    pos = new_pos;
                    visited.insert(new_pos);
                }
            } else {
                break;
            }
        }
    }

    visited.len()
}

#[test]
fn test_walk() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let mut day = Day06::new();
    day.parse(input);
    assert_eq!(walk(day.grid, day.start_pos), 41);
}
