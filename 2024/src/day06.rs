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
        walk(self.grid.clone(), self.start_pos).0.to_string()
    }

    fn p2(&self) -> String {
        let mut obstacles: HashSet<Pos> = HashSet::new();

        let route = walk(self.grid.clone(), self.start_pos).1;
        for pos in route.iter().skip(1) {
            let mut grid = self.grid.clone();
            grid.set(pos.x, pos.y, '@'); // <-- custom obstacle
            if did_loop(grid, self.start_pos) {
                obstacles.insert(*pos);
            }
        }

        obstacles.len().to_string()
    }
}

fn walk(mut grid: Grid<char>, start_pos: Pos) -> (usize, Vec<Pos>) {
    let mut pos = start_pos;
    let mut direction = '^';

    let mut route = Vec::<Pos>::new();
    route.push(start_pos);

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(start_pos);

    while grid.get(pos.x, pos.y).is_some() {
        let new_pos: Option<Pos> = match direction {
            '^' => Some((pos.x as isize - 1, pos.y as isize)),
            '>' => Some((pos.x as isize, pos.y as isize + 1)),
            'v' => Some((pos.x as isize + 1, pos.y as isize)),
            '<' => Some((pos.x as isize, pos.y as isize - 1)),
            _ => None,
        }
        .and_then(|(x, y)| {
            // make sure coords are positive
            if x >= 0 && y >= 0 {
                Some(Pos {
                    x: x as usize,
                    y: y as usize,
                })
            } else {
                None
            }
        });
        let new_direction: Option<char> = match direction {
            '^' => Some('>'),
            '>' => Some('v'),
            'v' => Some('<'),
            '<' => Some('^'),
            _ => None,
        };

        if let (Some(new_pos), Some(new_direction)) = (new_pos, new_direction) {
            if let Some(&new_val) = grid.get(new_pos.x, new_pos.y) {
                if new_val == '#' {
                    direction = new_direction;
                } else {
                    grid.set(pos.x, pos.y, '.');
                    grid.set(new_pos.x, new_pos.y, direction);
                    pos = new_pos;
                    visited.insert(new_pos);
                    route.push(new_pos);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    (visited.len(), route)
}

fn did_loop(mut grid: Grid<char>, start_pos: Pos) -> bool {
    let mut pos = start_pos;
    let mut direction = '^';

    let mut visited: HashSet<(Pos, char)> = HashSet::new();
    visited.insert((start_pos, direction));

    let mut route = Vec::<Pos>::new();
    route.push(start_pos);

    while grid.get(pos.x, pos.y).is_some() {
        let new_pos: Option<Pos> = match direction {
            '^' => Some((pos.x as isize - 1, pos.y as isize)),
            '>' => Some((pos.x as isize, pos.y as isize + 1)),
            'v' => Some((pos.x as isize + 1, pos.y as isize)),
            '<' => Some((pos.x as isize, pos.y as isize - 1)),
            _ => None,
        }
        .and_then(|(x, y)| {
            // make sure coords are positive
            if x >= 0 && y >= 0 {
                Some(Pos {
                    x: x as usize,
                    y: y as usize,
                })
            } else {
                None
            }
        });
        let new_direction: Option<char> = match direction {
            '^' => Some('>'),
            '>' => Some('v'),
            'v' => Some('<'),
            '<' => Some('^'),
            _ => None,
        };

        if let (Some(new_pos), Some(new_direction)) = (new_pos, new_direction) {
            if let Some(&new_val) = grid.get(new_pos.x, new_pos.y) {
                if new_val == '#' || new_val == '@' {
                    direction = new_direction;
                } else {
                    if !visited.insert((new_pos, direction)) {
                        // was visited, same direction => loop
                        return true;
                    }
                    grid.set(pos.x, pos.y, '.');
                    grid.set(new_pos.x, new_pos.y, direction);
                    pos = new_pos;
                    route.push(new_pos);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    return false;
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
    assert_eq!(day.p1(), "41");
}

#[test]
fn test_did_loop() {
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
    assert_eq!(day.p2(), "6");
}
