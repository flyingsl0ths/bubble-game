use bevy::prelude::*;

use crate::app_constants;

const GRID_SIZE: i32 = 14;
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
    let shape = meshes.add(Circle::new(BUBBLE_RADIUS));
    let (wx, wy) = app_constants::WINDOW_SIZE;

    let top_left = Vec3::new(-wx / 2.0, wy / 2.0 - BUBBLE_DIAMETER, 0.0);

    for r in 1..=GRID_SIZE - 3 {
        for c in 1..=GRID_SIZE {
            let color = Color::hsl(360. * r as f32 / c as f32, 0.95, 0.7);

            let mut pos = top_left
                + Vec3::new(
                    c as f32 * BUBBLE_DIAMETER,
                    -r as f32 * BUBBLE_DIAMETER,
                    1.0,
                );

            if r % 2 == 0 {
                pos.x += BUBBLE_RADIUS;
            }

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(pos),
            ));
        }
    }
}
