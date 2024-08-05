use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use rand::{self, Rng};

use crate::consts;

#[derive(Clone, Copy)]
pub enum BallColors {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Cyan,
    Pink,
}

const TOTAL_BUBBLE_COLORS: u8 = 7u8;

#[derive(Component, Clone, Copy)]
pub struct Ball {
    pub radius: f32,
    pub color: BallColors,
}

pub struct BubbleSystems;

impl Into<BallColors> for u8 {
    fn into(self) -> BallColors {
        match self {
            0 => BallColors::Red,
            1 => BallColors::Green,
            2 => BallColors::Blue,
            3 => BallColors::Yellow,
            4 => BallColors::Purple,
            5 => BallColors::Cyan,
            _ => BallColors::Pink,
        }
    }
}

impl Into<Color> for BallColors {
    fn into(self) -> Color {
        match self {
            BallColors::Red => Color::srgb(1.0, 0., 0.),
            BallColors::Green => Color::srgb(0., 1.0, 0.),
            BallColors::Blue => Color::srgb(0., 0., 1.0),
            BallColors::Yellow => Color::srgb(1.0, 1.0, 0.),
            BallColors::Purple => Color::srgb(1.0, 0., 1.0),
            BallColors::Cyan => Color::srgb(0., 1.0, 1.0),
            BallColors::Pink => Color::srgb(1.0, 0.5, 0.5),
        }
    }
}

pub fn new_ball(radius: f32) -> Ball {
    let mut random = rand::thread_rng();

    Ball {
        radius,
        color: random.gen_range(0u8..TOTAL_BUBBLE_COLORS).into(),
    }
}

impl Plugin for BubbleSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut bubble_size = rand::thread_rng();

    let mut spawn_ball = |commands: &mut Commands,
                          meshes: &mut ResMut<Assets<Mesh>>,
                          materials: &mut ResMut<Assets<ColorMaterial>>,
                          x: f32,
                          y: f32| {
        let ball = Ball {
            radius: consts::BUBBLE_SIZE,
            color: bubble_size.gen_range(0u8..TOTAL_BUBBLE_COLORS).into(),
        };

        let circle = Mesh2dHandle(meshes.add(Circle {
            radius: ball.radius,
        }));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle,
                material: materials.add(Into::<Color>::into(ball.color)),
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            },
            ball,
        ));
    };

    const HALF_WINDOW_WIDTH: f32 = consts::WINDOW_SIZE.0 / 2.;

    const TOTAL_BUBBLES: i32 = (HALF_WINDOW_WIDTH / consts::BUBBLE_SIZE) as i32;

    let start_x = -(HALF_WINDOW_WIDTH - consts::BUBBLE_SIZE) as f32;
    let start_y = 0. - consts::BUBBLE_SIZE;
    let mut bubble_padding = rand::thread_rng();

    for sy in 0..6 as u32 {
        let padding: f32 = bubble_padding.gen_range(1u8..=20u8).into();
        for sx in 0..TOTAL_BUBBLES as i32 {
            let x = start_x + (sx as f32 * consts::BUBBLE_SIZE * 2.) + padding;

            let y = start_y + (sy as f32) * consts::BUBBLE_SIZE * 2.;

            spawn_ball(&mut commands, &mut meshes, &mut materials, x, y);
        }
    }
}
