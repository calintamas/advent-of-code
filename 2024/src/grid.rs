use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone + Copy + fmt::Display,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            self.data.get(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
        }
    }

    pub fn bfs<F>(&self, start: Point, mut is_valid: F) -> Vec<Point>
    where
        F: FnMut(Point, Point, &HashSet<Point>) -> bool,
    {
        let directions = [
            (0, 1),  // Right
            (0, -1), // Left
            (1, 0),  // Down
            (-1, 0), // Up
        ];

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut path = Vec::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            path.push(current);

            for &(dx, dy) in &directions {
                let neighbor = Point {
                    x: current.x + dx,
                    y: current.y + dy,
                };

                if self.get(neighbor.x as usize, neighbor.y as usize).is_none() {
                    continue;
                }

                if is_valid(current, neighbor, &visited) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }

        path
    }

    #[allow(dead_code)]
    pub fn pretty_print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.get(row, col).unwrap());
            }
            println!();
        }
    }
}
