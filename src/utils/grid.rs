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
        let (t, ts) = &self.top;
        let (l, ls) = &self.left;
        let (r, rs) = &self.right;
        let (b, bs) = &self.bottom;

        if t.contains(position) {
            Some(ts)
        } else if l.contains(position) {
            Some(ls)
        } else if r.contains(position) {
            Some(rs)
        } else if b.contains(position) {
            Some(bs)
        } else {
            None
        }
    }
}
