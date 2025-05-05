use core::ops::{Index, IndexMut};

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct FlatArray<T, const C: usize>(Vec<T>);

impl<T, const C: usize> FlatArray<T, C> {
    pub fn to_index(row: usize, column: usize) -> usize {
        row * C + column
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.0.get(row * C + column).or(None)
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.0.get_mut(row * C + column).or(None)
    }

    pub fn new(values: Vec<T>) -> Self {
        Self(values)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, const C: usize> Index<(usize, usize)> for FlatArray<T, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.0[row * C + column]
    }
}

impl<T, const C: usize> IndexMut<(usize, usize)> for FlatArray<T, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        &mut self.0[row * C + column]
    }
}
