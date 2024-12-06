use crate::{advent_day::AdventDay, grid::Grid};

pub struct Day04 {
    grid: Grid<char>,
}

impl Day04 {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
        }
    }
}

impl AdventDay for Day04 {
    fn parse(&mut self, input: &str) {
        self.grid = parse(input);
    }

    fn p1(&self) -> String {
        count_xmas(&self.grid).to_string()
    }

    fn p2(&self) -> String {
        count_x_mas(&self.grid).to_string()
    }
}

fn parse(input: &str) -> Grid<char> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let mut grid = Grid::<char>::new(rows, cols);

    for (row, line) in input.lines().enumerate() {
        for (col, value) in line.chars().enumerate() {
            grid.set(row, col, value);
        }
    }
    return grid;
}

fn count_xmas(grid: &Grid<char>) -> usize {
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (-1, -1), // Up-left
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
    ];

    let word = "XMAS";
    let word_chars: Vec<char> = word.chars().collect();

    let mut count = 0;

    for x in 0..grid.rows {
        for y in 0..grid.cols {
            for (dx, dy) in &directions {
                let mut found = true;
                // slide a "window" of word size across the grid
                // in all valid directions
                for i in 0..word.len() {
                    let nx = x as isize + i as isize * dx;
                    let ny = y as isize + i as isize * dy;

                    if let Some(val) = grid.get(nx as usize, ny as usize) {
                        if val != &word_chars[i] {
                            found = false
                        }
                    } else {
                        found = false
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    return count;
}

/*
    M.S
    .A.
    M.S
*/
fn count_x_mas(grid: &Grid<char>) -> usize {
    let mut count = 0;
    for x in 1..grid.rows - 1 {
        for y in 1..grid.cols - 1 {
            if grid.get(x, y) == Some(&'A') {
                // top-left bottom-right diagonal
                let a1 =
                    grid.get(x - 1, y - 1) == Some(&'M') && grid.get(x + 1, y + 1) == Some(&'S');
                let a2 =
                    grid.get(x - 1, y - 1) == Some(&'S') && grid.get(x + 1, y + 1) == Some(&'M');

                // top-right bottom-left diagonal
                let b1 =
                    grid.get(x - 1, y + 1) == Some(&'M') && grid.get(x + 1, y - 1) == Some(&'S');
                let b2 =
                    grid.get(x - 1, y + 1) == Some(&'S') && grid.get(x + 1, y - 1) == Some(&'M');

                if (a1 || a2) && (b1 || b2) {
                    count += 1;
                }
            }
        }
    }

    return count;
}

#[test]
fn test_count_xmas() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let mut day = Day04::new();
    day.parse(input);
    assert_eq!(count_xmas(&day.grid), 18);
}

#[test]
fn test_count_x_mas() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let mut day = Day04::new();
    day.parse(input);
    assert_eq!(count_x_mas(&day.grid), 9);
}
