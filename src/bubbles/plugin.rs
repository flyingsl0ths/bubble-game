use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    color::Color,
    math::Vec3,
    prelude::{Circle, Commands, Mesh, Mesh2d, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
};

use crate::{
    app_constants,
    utils::{flat_array::FlatArray, grid::Grid},
};

use super::{colors, value::Bubble};

const GRID_SIZE: i32 = 20;
const GRID_SIZE_U: usize = GRID_SIZE as usize;
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

    let top_left = Vec3::new(
        -wx / 2.0 + BUBBLE_DIAMETER * 2.0,
        wy / 2.0 - BUBBLE_DIAMETER,
        0.0,
    );

    let hollow = Color::hsl(0., 0., 0.68);

    let hollow_mat = materials.add(hollow);

    let mut color_map = colors::ColorMap::new(hollow_mat.clone());

    let mut grid_items: Vec<Bubble> = Vec::with_capacity((GRID_SIZE_U - 1) * GRID_SIZE_U);

    let mut last_row = GRID_SIZE - 10;

    for i in 0..(last_row * GRID_SIZE) {
        let r = (i / GRID_SIZE) + 1;
        let c = (i % GRID_SIZE) + 1;

        let color = Color::hsl(360. * r as f32 / c as f32, 0.95, 0.7);

        let mut pos = top_left
            + Vec3::new(
                c as f32 * BUBBLE_DIAMETER - 10.0,
                -r as f32 * BUBBLE_DIAMETER - 10.0,
                0.0,
            );

        if r % 2 == 0 {
            pos.x += BUBBLE_RADIUS;
        }

        let color = materials.add(color);

        color_map.insert((r + c) as usize, color.clone());

        grid_items.push(Bubble {
            row: r as usize,
            column: c as usize,
            pos,
            radius: BUBBLE_RADIUS,
            edges: [(0, 0); 6],
        });

        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(color.clone_weak()),
            Transform::from_translation(pos),
        ));
    }

    last_row += 1;
    for r in 0..2 {
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

            grid_items.push(Bubble {
                row: r_ as usize,
                column: c as usize,
                pos,
                radius: BUBBLE_RADIUS,
                edges: [(0, 0); 6],
            });

            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(hollow_mat.clone_weak()),
                Transform::from_translation(pos),
            ));
        }
    }

    let mut grid: FlatArray<Bubble, GRID_SIZE_U> = FlatArray::new(grid_items);
    add_edges(&mut grid);

    commands.insert_resource(grid);
    commands.insert_resource(color_map);
}

fn add_edges(grid: &mut FlatArray<Bubble, GRID_SIZE_U>) {
    // The edges are the neighbors of the bubble
    // and are always present and in the following order
    const DOWN: usize = 0;
    const UP: usize = 1;
    const RIGHT: usize = 2;
    const LEFT: usize = 3;

    // This depends on the row number
    // At most a bubble can only have one of these two
    // Either a BOTTOM_LEFT and TOP_LEFT
    const BOTTOM_LEFT: usize = 4;
    const TOP_LEFT: usize = 5;
    // or a BOTTOM_RIGHT and TOP_RIGHT
    const BOTTOM_RIGHT: usize = BOTTOM_LEFT;
    const TOP_RIGHT: usize = TOP_LEFT;

    const GRID_SIZE_: usize = GRID_SIZE_U - 1;

    // In general a bubble can have up to 6 edges
    //
    // Anything starting/ending between the top and last row can have 4 edges
    // ASSUMMING THERE IS AT LEAST ONE BUBBLE ABOVE AND BELOW IT AND 2 BUBBLES
    // TO THE LEFT AND RIGHT
    // -       -
    // -o-   -o-
    // -       -
    //
    // The top row's bubbles can either have one of two patterns
    // -o-   -o-
    // --     --
    //
    // The last row's bubbles can either have one of two patterns
    // --     --
    // -o-    -o-
    let total = grid.len();
    for i in 0..total {
        let r = i / GRID_SIZE_U;
        let c = r % GRID_SIZE_U;

        let bubble = &mut grid[(r, c)];

        if r < GRID_SIZE_ {
            bubble.edges[DOWN] = (r + 1, c);
        }

        if r > 0 {
            bubble.edges[UP] = (r - 1, c);
        }

        if c < GRID_SIZE_ {
            bubble.edges[RIGHT] = (r, c + 1);
        }

        if c > 0 {
            bubble.edges[LEFT] = (r, c - 1);
        }

        if r % 2 == 0 {
            if r < GRID_SIZE_ && c > 0 {
                bubble.edges[BOTTOM_LEFT] = (r + 1, c - 1);
            }
            if r > 0 && c > 0 {
                bubble.edges[TOP_LEFT] = (r - 1, c - 1);
            }
        } else {
            if r < GRID_SIZE_ && c < GRID_SIZE_ {
                bubble.edges[BOTTOM_RIGHT] = (r + 1, c + 1);
            }

            if r > 0 && c < GRID_SIZE_ {
                bubble.edges[TOP_RIGHT] = (r - 1, c + 1);
            }
        }
    }
}
