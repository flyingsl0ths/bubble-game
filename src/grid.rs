use core::ops::{Index, IndexMut};

pub struct Grid<T, const N: usize> {
    values: [T; N],
    columns: usize,
}

impl<T, const N: usize> Grid<T, N> {
    pub fn new(values: [T; N], columns: usize) -> Self {
        Self { values, columns }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.values.get(row * self.columns + column).or(None)
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.values.get_mut(row * self.columns + column).or(None)
    }
}

impl<T, const N: usize> Index<(usize, usize)> for Grid<T, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.values[row * self.columns + column]
    }
}

impl<T, const N: usize> IndexMut<(usize, usize)> for Grid<T, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        &mut self.values[row * self.columns + column]
    }
}
