// --- Day 4: Printing Department ---

use rust_tools::grid::Grid;

pub type Input = Grid<char>;

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
    let mut count = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.get(row, col).unwrap() == &'@' {
                let mut sum = 0;
                grid.for_each_neighbor_of((row, col), |neighbor, _| {
                    if neighbor.unwrap_or(&'.') == &'@' {
                        sum += 1
                    }
                });
                if sum < 4 {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

pub fn part2(input: &Input) -> String {
    let mut grid = input.clone();

    let mut prev_count = -1;
    let mut count = 0;

    while count != prev_count {
        prev_count = count;
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if grid.get(row, col).unwrap() == &'@' {
                    let mut sum = 0;
                    grid.for_each_neighbor_of((row, col), |neighbor, _| {
                        if neighbor.unwrap_or(&'.') == &'@' {
                            sum += 1
                        }
                    });
                    if sum < 4 {
                        count += 1;
                        grid.set(row, col, '.');
                    }
                }
            }
        }
    }

    count.to_string()
}
