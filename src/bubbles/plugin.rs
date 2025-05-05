use super::{colors, value};

use crate::{
    app_constants,
    state::GameState,
    utils::{flat_array::FlatArray, grid::Grid, rect},
};

use bevy::{
    app::{App, Plugin},
    asset::{Assets, Handle},
    color::Color,
    math::{Vec2, Vec3},
    prelude::{Circle, Commands, Mesh, Mesh2d, OnEnter, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
};

use rand::prelude::*;

const GRID_SIZE: i32 = 20;
const GRID_SIZE_U: usize = GRID_SIZE as usize;
const BUBBLE_RADIUS: f32 = 25.0;
const BUBBLE_DIAMETER: f32 = BUBBLE_RADIUS * 2.0;
// const POINTS_PER_BUBBLE: f32 = 2.0;

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::default()), BubblesPlugin::setup);
    }
}

impl BubblesPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let (wx, wy) = app_constants::WINDOW_SIZE;

        let bubble_shape = meshes.add(Circle::new(BUBBLE_RADIUS));

        let top_left = Vec3::new(
            -wx / 2.0 + BUBBLE_DIAMETER * 2.0,
            wy / 2.0 - BUBBLE_DIAMETER,
            0.0,
        );

        let mut collision_grid = Self::mk_collision_grid(top_left);

        let hollow = Color::hsl(0., 0., 0.68);

        let hollow_mat = materials.add(hollow);

        let bubble_colors = [
            materials.add(Color::hsl(0.0, 0.75, 0.75)),
            materials.add(Color::hsl(120.0, 0.75, 0.75)),
            materials.add(Color::hsl(240.0, 0.75, 0.75)),
            materials.add(Color::hsl(60.0, 0.75, 0.75)),
            materials.add(Color::hsl(280.0, 0.75, 0.75)),
            materials.add(Color::hsl(30.0, 0.75, 0.75)),
        ];

        let mut bubbles: Vec<value::Bubble> =
            Vec::with_capacity((GRID_SIZE_U - 1) * GRID_SIZE_U);

        let mut last_row = GRID_SIZE - 10;

        let mut rng = rand::rng();

        Self::mk_bubble_grid(
            &mut commands,
            top_left,
            last_row,
            &mut rng,
            &mut bubbles,
            &bubble_colors,
            bubble_shape.clone_weak(),
            &mut collision_grid,
        );

        last_row += 1;

        Self::mk_bubble_grid_footer(
            &mut commands,
            top_left,
            last_row,
            &mut rng,
            &mut bubbles,
            &bubble_colors,
            bubble_shape.clone_weak(),
            &mut collision_grid,
            hollow_mat.clone_weak(),
        );

        commands.insert_resource(Self::add_edges(FlatArray::new(bubbles)));
        commands.insert_resource(collision_grid);
        commands
            .insert_resource(colors::ColorMap::new(bubble_colors, hollow_mat.clone()));
        commands.insert_resource(value::BubbleShape(bubble_shape.clone()));
    }

    fn mk_bubble_grid(
        commands: &mut Commands,
        top_left: Vec3,
        last_row: i32,
        rng: &mut ThreadRng,
        bubbles: &mut Vec<value::Bubble>,
        bubble_colors: &[Handle<ColorMaterial>; 6],
        bubble_shape: Handle<Mesh>,
        collision_grid: &mut Grid,
    ) {
        for i in 0..(last_row * GRID_SIZE) {
            let r = (i / GRID_SIZE) + 1;
            let c = (i % GRID_SIZE) + 1;

            let mut pos = top_left
                + Vec3::new(
                    c as f32 * BUBBLE_DIAMETER - 10.0,
                    -r as f32 * BUBBLE_DIAMETER - 10.0,
                    0.0,
                );

            if r % 2 == 0 {
                pos.x += BUBBLE_RADIUS;
            }

            let color_index = rng.random_range(0..bubble_colors.len());

            bubbles.push(value::Bubble {
                row: r as usize,
                column: c as usize,
                pos,
                edges: [(0, 0); value::EDGE_COUNT],
                color: colors::BubbleColors::from_u8(color_index as u8),
            });

            commands.spawn((
                Mesh2d(bubble_shape.clone_weak()),
                MeshMaterial2d(bubble_colors[color_index].clone()),
                Transform::from_translation(pos),
            ));

            collision_grid.place_in(
                pos.truncate(),
                FlatArray::<value::Bubble, GRID_SIZE_U>::to_index(
                    r as usize - 1,
                    c as usize - 1,
                ),
            );
        }
    }

    fn mk_bubble_grid_footer(
        commands: &mut Commands,
        top_left: Vec3,
        last_row: i32,
        rng: &mut ThreadRng,
        bubbles: &mut Vec<value::Bubble>,
        bubble_colors: &[Handle<ColorMaterial>; 6],
        bubble_shape: Handle<Mesh>,
        collision_grid: &mut Grid,
        hollow_mat: Handle<ColorMaterial>,
    ) {
        for r in 0..(2 * GRID_SIZE) {
            let r_ = last_row + (r / GRID_SIZE);
            let c = (r % GRID_SIZE) + 1;

            let mut pos = top_left
                + Vec3::new(
                    c as f32 * BUBBLE_DIAMETER - 10.0,
                    -(r_ as f32) * BUBBLE_DIAMETER - 10.0,
                    1.0,
                );

            if r_ % 2 == 0 {
                pos.x += BUBBLE_RADIUS;
            }

            bubbles.push(value::Bubble {
                row: r_ as usize,
                column: c as usize,
                pos,
                edges: [(0, 0); 6],
                color: colors::BubbleColors::from_u8(
                    rng.random_range(0..bubble_colors.len()) as u8,
                ),
            });

            commands.spawn((
                Mesh2d(bubble_shape.clone_weak()),
                MeshMaterial2d(hollow_mat.clone_weak()),
                Transform::from_translation(pos),
            ));

            collision_grid.place_in(
                pos.truncate(),
                FlatArray::<value::Bubble, GRID_SIZE_U>::to_index(
                    r_ as usize,
                    c as usize - 1,
                ),
            );
        }
    }

    fn mk_collision_grid(top_left: Vec3) -> Grid {
        let quadrant_size = BUBBLE_DIAMETER * ((GRID_SIZE_U / 2) as f32);

        let left = rect::Rect2D::new(top_left.truncate(), quadrant_size, quadrant_size);

        let right = rect::Rect2D::new(
            Vec2::new(
                top_left.x + BUBBLE_DIAMETER * ((GRID_SIZE_U / 2) as f32),
                top_left.y,
            ),
            quadrant_size,
            quadrant_size,
        );

        let bottom_left = rect::Rect2D::new(
            Vec2::new(
                top_left.x,
                top_left.y - BUBBLE_DIAMETER * (GRID_SIZE_U as f32),
            ),
            quadrant_size,
            quadrant_size,
        );

        let bottom_right = rect::Rect2D::new(
            Vec2::new(
                top_left.x + BUBBLE_DIAMETER * ((GRID_SIZE_U / 2) as f32),
                top_left.y - BUBBLE_DIAMETER * (GRID_SIZE_U as f32),
            ),
            quadrant_size,
            quadrant_size,
        );

        Grid::new([left, right, bottom_left, bottom_right])
    }

    fn add_edges(
        mut bubbles: FlatArray<value::Bubble, GRID_SIZE_U>,
    ) -> FlatArray<value::Bubble, GRID_SIZE_U> {
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
        let total = bubbles.len();
        for i in 0..total {
            let r = i / GRID_SIZE_U;
            let c = r % GRID_SIZE_U;

            let bubble = &mut bubbles[(r, c)];

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

        return bubbles;
    }
}
