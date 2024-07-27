use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

use rand::{self, Rng};

use crate::MainCamera;

pub struct PlayerSystems;

#[derive(Component)]
struct Recticle;

#[derive(Component, Resource, Default)]
pub struct MousePosition(Vec2);

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

impl Plugin for PlayerSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (get_mouse_position, draw_reticle));
    }
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let bag_size: f32 = 50. * 2.;
    let ball = new_ball(35.);

    let rect = Mesh2dHandle(meshes.add(Rectangle::new(bag_size, bag_size)));
    let circle = Mesh2dHandle(meshes.add(Circle {
        radius: ball.radius,
    }));

    const CENTER_Y: f32 = crate::consts::WINDOW_SIZE.1 / 2.;

    let ball_pos: Vec3 = Vec3::new(0., -(CENTER_Y - ball.radius), 0.);

    let ball_id = commands
        .spawn(MaterialMesh2dBundle {
            mesh: circle,
            material: materials.add(Into::<Color>::into(ball.color)),
            transform: Transform::from_translation(ball_pos),
            ..default()
        })
        .id();

    commands.spawn(MaterialMesh2dBundle {
        mesh: rect,
        material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
        transform: Transform::from_translation(Vec3::new(
            -ball.radius * 3.25,
            -(CENTER_Y - bag_size / 2.),
            0.,
        )),
        ..default()
    });

    commands.entity(ball_id).insert(ball);

    const RECTICLE_SIZE: f32 = 10.0;

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player/reticle.png"),
            transform: Transform::from_translation(
                ball_pos + Vec3::new(0., RECTICLE_SIZE, 1.),
            ),
            ..default()
        },
        Recticle,
    ));
}

fn get_mouse_position(
    mut coordinates: ResMut<MousePosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let Some(cursor) = q_window.single().cursor_position() else {
        return;
    };

    if let Some(position) =
        camera.viewport_to_world_2d(camera_transform, cursor)
    {
        coordinates.0 = position;
    }
}

fn draw_reticle(
    coordinates: ResMut<MousePosition>,
    mut recticle: Query<&mut Transform, With<Recticle>>,
) {
    let coordinates = coordinates.0;
    let mut recticle = recticle.single_mut();
    recticle.translation = coordinates.extend(1.);
}
