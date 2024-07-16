use bevy::prelude::{
    Color,
};
use rand::{self, Rng};
#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub color: BallColors,
}
#[derive(Clone, Copy)]
pub enum BallColors {
    Red(f32, f32, f32),
    Green(f32, f32, f32),
    Blue(f32, f32, f32),
}

impl Into<BallColors> for u8 {
    fn into(self) -> BallColors {
        match self {
            0 => BallColors::Red(1.0, 0.0, 0.0),
            1 => BallColors::Green(0.0, 1.0, 0.0),
            _ => BallColors::Blue(0.0, 0.0, 1.0),
        }
    }
}

impl Into<Color> for BallColors {
    fn into(self) -> Color {
        match self {
            BallColors::Red(r, g, b) => Color::srgb(r, g, b),
            BallColors::Green(r, g, b) => Color::srgb(r, g, b),
            BallColors::Blue(r, g, b) => Color::srgb(r, g, b),
        }
    }
}

pub fn new_ball(radius: f32) -> Ball {
    let mut random = rand::thread_rng();

    Ball {
        radius,
        color: random.gen_range(0u8..=2u8).into(),
    }
}
