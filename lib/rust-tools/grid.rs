use std::fmt;

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

    pub fn pretty_print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.get(row, col).unwrap());
            }
            println!();
        }
    }

    pub fn for_each_neighbor_of<F>(&self, item: (usize, usize), mut cb: F)
    where
        F: FnMut(Option<&T>, (isize, isize)),
    {
        let directions: [(isize, isize); 8] = [
            (0, 1),   // Right
            (0, -1),  // Left
            (1, 0),   // Down
            (-1, 0),  // Up
            (1, 1),   // Down-right
            (-1, -1), // Up-left
            (1, -1),  // Down-left
            (-1, 1),  // Up-right
        ];
        for (dx, dy) in &directions {
            let n_row = item.0 as isize + dx;
            let n_col = item.1 as isize + dy;
            let val = self.get(n_row as usize, n_col as usize);
            cb(val, (n_row, n_col));
        }
    }
}
