use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

mod player;

const WINDOW_SIZE: (f32, f32) = (1280., 880.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bobbles".to_string(),
                resizable: false,
                resolution: WINDOW_SIZE.into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let bag_size: f32 = 50. * 2.;
    let ball = player::new_ball(35.);

    let rect = Mesh2dHandle(meshes.add(Rectangle::new(bag_size, bag_size)));
    let circle = Mesh2dHandle(meshes.add(Circle {
        radius: ball.radius,
    }));

    const CENTER_Y: f32 = WINDOW_SIZE.1 / 2.;

    let ball_pos: Vec3 =
        Vec3::new(-bag_size + ball.radius, -(CENTER_Y - ball.radius), 0.);

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
            -bag_size * 2.,
            -(CENTER_Y - bag_size / 2.),
            0.,
        )),
        ..default()
    });

    commands.entity(ball_id).insert(ball);

    const RECTICLE_SIZE: f32 = 10.0;
    let triangle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * RECTICLE_SIZE,
        Vec2::new(-RECTICLE_SIZE, -RECTICLE_SIZE),
        Vec2::new(RECTICLE_SIZE, -RECTICLE_SIZE),
    )));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: triangle,
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_translation(
                ball_pos + Vec3::new(0., RECTICLE_SIZE, 0.),
            ),
            ..default()
        },
        Recticle,
    ));
}
