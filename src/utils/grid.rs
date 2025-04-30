use bevy::math::Vec2;

use super::rect;

pub struct Grid {
    top: (rect::Rect2D, Vec<usize>),
    left: (rect::Rect2D, Vec<usize>),
    right: (rect::Rect2D, Vec<usize>),
    bottom: (rect::Rect2D, Vec<usize>),
}

impl Grid {
    pub fn new(positions: [rect::Rect2D; 4usize]) -> Self {
        let [t, l, r, b] = positions;
        Self {
            top: (t, vec![]),
            left: (l, vec![]),
            right: (r, vec![]),
            bottom: (b, vec![]),
        }
    }

    pub fn values_for(&self, position: Vec2) -> Option<&Vec<usize>> {
        if self.top.0.contains(position) {
            Some(&self.top.1)
        } else if self.left.0.contains(position) {
            Some(&self.left.1)
        } else if self.right.0.contains(position) {
            Some(&self.right.1)
        } else if self.bottom.0.contains(position) {
            Some(&self.bottom.1)
        } else {
            None
        }
    }
}
