use bevy::math::Vec3;

pub struct Bubble {
    pub row: usize,
    pub column: usize,
    pub pos: Vec3,
    pub radius: f32,
    pub edges: [(usize, usize); 6],
}

impl Bubble {
    pub fn rc_sum(&self) -> usize {
        self.row + self.column
    }
}
