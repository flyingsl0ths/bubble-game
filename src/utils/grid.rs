use bevy::{math::Vec2, prelude::Resource};

use super::rect;

#[derive(Resource)]
pub struct Grid {
    left: (rect::Rect2D, Vec<usize>),
    right: (rect::Rect2D, Vec<usize>),
    bottom_left: (rect::Rect2D, Vec<usize>),
    bottom_right: (rect::Rect2D, Vec<usize>),
}

impl Grid {
    pub fn new(positions: [rect::Rect2D; 4usize]) -> Self {
        let [t, l, r, b] = positions;
        Self {
            left: (l, vec![]),
            right: (r, vec![]),
            bottom_left: (t, vec![]),
            bottom_right: (b, vec![]),
        }
    }

    pub fn place_in(&mut self, position: Vec2, value: usize) {
        if self.bottom_left.0.contains(position) {
            self.bottom_left.1.push(value);
        } else if self.left.0.contains(position) {
            self.left.1.push(value);
        } else if self.right.0.contains(position) {
            self.right.1.push(value);
        } else if self.bottom_right.0.contains(position) {
            self.bottom_right.1.push(value);
        }
    }

    pub fn query(&self, position: Vec2) -> Option<&Vec<usize>> {
        if self.bottom_left.0.contains(position) {
            Some(&self.bottom_left.1)
        } else if self.left.0.contains(position) {
            Some(&self.left.1)
        } else if self.right.0.contains(position) {
            Some(&self.right.1)
        } else if self.bottom_right.0.contains(position) {
            Some(&self.bottom_right.1)
        } else {
            None
        }
    }
}
