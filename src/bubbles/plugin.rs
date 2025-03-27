use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    color::Color,
    math::Vec3,
    prelude::{Circle, Commands, Mesh, Mesh2d, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
};

use crate::app_constants;

use super::colors;

const GRID_SIZE: i32 = 15;
const BUBBLE_RADIUS: f32 = 25.0;
const BUBBLE_DIAMETER: f32 = BUBBLE_RADIUS * 2.0;
// const POINTS_PER_BUBBLE: f32 = 2.0;

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (wx, wy) = app_constants::WINDOW_SIZE;

    let shape = meshes.add(Circle::new(BUBBLE_RADIUS));

    let top_left = Vec3::new(-wx / 2.0, wy / 2.0 - BUBBLE_DIAMETER, 0.0);

    let hollow = Color::hsl(0., 0., 0.68);

    let hollow_mat = materials.add(hollow);

    let mut color_map = colors::Colors::new(hollow_mat.clone());

    for r in 1..=GRID_SIZE - 4 {
        for c in 1..=GRID_SIZE {
            let color = Color::hsl(360. * r as f32 / c as f32, 0.95, 0.7);

            let mut pos = top_left
                + Vec3::new(
                    c as f32 * BUBBLE_DIAMETER - 10.0,
                    -r as f32 * BUBBLE_DIAMETER - 10.0,
                    1.0,
                );

            if r % 2 == 0 {
                pos.x += BUBBLE_RADIUS;
            }

            let color = materials.add(color);

            color_map.insert((r + c) as usize, color.clone());

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(color.clone_weak()),
                Transform::from_translation(pos),
            ));
        }
    }

    let last_row = GRID_SIZE - 3;

    for r in 0..3 {
        let r_ = last_row + r;
        for c in 1..=GRID_SIZE {
            let mut pos = top_left
                + Vec3::new(
                    c as f32 * BUBBLE_DIAMETER - 10.0,
                    -r_ as f32 * BUBBLE_DIAMETER - 10.0,
                    1.0,
                );

            if r_ % 2 == 0 {
                pos.x += BUBBLE_RADIUS;
            }

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(hollow_mat.clone_weak()),
                Transform::from_translation(pos),
            ));
        }
    }

    commands.insert_resource(color_map);
}
