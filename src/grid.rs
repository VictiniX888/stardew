use std::path::Iter;

pub struct Grid<T> {
    arr: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let arr = vec![T::default(); rows * cols];
        Grid { arr, rows, cols }
    }
}

impl<T> Grid<T> {
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.arr[row * self.cols + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.arr[row * self.cols + col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.arr[row * self.cols + col]
    }

    pub fn set_by_index(&mut self, index: usize, value: T) {
        self.arr[index] = value;
    }

    pub fn replace_by_index(&mut self, index: usize, value: T) -> T {
        std::mem::replace(&mut self.arr[index], value)
    }

    pub fn get_by_index(&self, index: usize) -> &T {
        &self.arr[index]
    }

    pub fn get_mut_by_index(&mut self, index: usize) -> &mut T {
        &mut self.arr[index]
    }

    pub fn enumerate_iter(&self) -> EnumerateIter<T> {
        EnumerateIter {
            grid: self,
            next_row: 0,
            next_col: 0,
        }
    }
}

impl<T> Grid<Option<T>> {
    pub fn enumerate_iter_sparse(&self) -> EnumerateIterSparse<T> {
        EnumerateIterSparse {
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

pub struct EnumerateIterSparse<'a, T> {
    grid: &'a Grid<Option<T>>,
    next_row: usize,
    next_col: usize,
}

impl<'a, T> EnumerateIterSparse<'a, T> {
    fn _next(&mut self) -> Option<((usize, usize), &'a Option<T>)> {
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

impl<'a, T> Iterator for EnumerateIterSparse<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, item)) = self._next() {
            if let Some(item) = item {
                return Some((index, item));
            }
        }

        None
    }
}
