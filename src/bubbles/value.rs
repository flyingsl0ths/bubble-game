use bevy::math::Vec3;

pub struct Bubble {
    pub row: usize,
    pub column: usize,
    pub pos: Vec3,
    pub radius: f32,
    pub edges: [usize; 6],
}
