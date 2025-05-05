use bevy::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Rect2D {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect2D {
    pub fn new(xy: Vec2, width: f32, height: f32) -> Self {
        Self {
            x: xy.x,
            y: xy.y,
            width,
            height,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}
