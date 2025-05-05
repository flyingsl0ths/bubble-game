use bevy::{
    asset::Handle,
    math::Vec3,
    prelude::{Mesh, Resource},
};

use super::colors::BubbleColors;

pub const EDGE_COUNT: usize = 6;

#[derive(Resource)]
pub struct BubbleShape(pub Handle<Mesh>);

pub struct Bubble {
    pub row: usize,
    pub column: usize,
    pub pos: Vec3,
    pub edges: [(usize, usize); EDGE_COUNT],
    pub color: BubbleColors,
}

impl Bubble {
    pub fn rc_sum(&self) -> usize {
        self.row + self.column
    }
}
