use core::ops::{Index, IndexMut};

macro_rules! check_usable {
    ($self:expr) => {
        if !$self.usable {
            return None;
        }
    };
}

pub struct Grid<T, const N: usize> {
    values: Vec<T>,
    columns: usize,
    usable: bool,
}

impl<T, const N: usize> Grid<T, N> {
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        check_usable!(self);

        self.values.get(row * self.columns + column).or(None)
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        check_usable!(self);

        self.values.get_mut(row * self.columns + column).or(None)
    }

    pub fn new(values: Vec<T>) -> Self {
        let usable = values.len() % N == 0;

        Self {
            values,
            columns: N,
            usable,
        }
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
