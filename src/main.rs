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

    let ball_pos: Vec3 =
        Vec3::new(-bag_size + ball.radius, -bag_size * 3.25, 0.);

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
            -bag_size * 3.09,
            0.,
        )),
        ..default()
    });


    commands.entity(ball_id).insert(ball);
}
