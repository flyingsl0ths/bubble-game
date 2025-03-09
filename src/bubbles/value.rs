use bevy::{asset::Handle, math::Vec3, sprite::ColorMaterial};

pub struct Bubble {
    pub color: Handle<ColorMaterial>,
    pub row: usize,
    pub column: usize,
    pub pos: Vec3,
    pub radius: f32,
}
