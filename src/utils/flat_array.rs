use core::ops::{Index, IndexMut};

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct FlatArray<T, const N: usize>(Vec<T>);

impl<T, const N: usize> FlatArray<T, N> {
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.0.get(row * N + column).or(None)
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.0.get_mut(row * N + column).or(None)
    }

    pub fn new(values: Vec<T>) -> Self {
        Self(values)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, const N: usize> Index<(usize, usize)> for FlatArray<T, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.0[row * N + column]
    }
}

impl<T, const N: usize> IndexMut<(usize, usize)> for FlatArray<T, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        &mut self.0[row * N + column]
    }
}
