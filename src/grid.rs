pub struct Grid<T> {
    arr: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let arr = vec![T::default(); rows * cols];
        Grid { arr, rows, cols }
    }
}

impl<T> Grid<T> {
    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.arr[row * self.cols + col] = value;
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.arr[row * self.cols + col]
    }

    pub fn enumerate_iter(&self) -> EnumerateIter<T> {
        EnumerateIter {
            grid: self,
            next_row: 0,
            next_col: 0,
        }
    }
}

pub struct EnumerateIter<'a, T> {
    grid: &'a Grid<T>,
    next_row: usize,
    next_col: usize,
}

impl<'a, T> Iterator for EnumerateIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row < self.grid.rows {
            let row = self.next_row;
            let col = self.next_col;
            let element = unsafe {
                self.grid
                    .arr
                    .get_unchecked(self.next_row * self.grid.cols + self.next_col)
            };

            if self.next_col < self.grid.cols {
                self.next_col += 1;
            }
            if self.next_col == self.grid.cols {
                self.next_row += 1;
                self.next_col = 0;
            }

            Some(((row, col), element))
        } else {
            None
        }
    }
}
