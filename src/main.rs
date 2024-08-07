use bevy::prelude::*;

mod bubbles;
mod consts;
mod player;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bobbles".to_string(),
                resizable: false,
                resolution: consts::WINDOW_SIZE.into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_resource::<player::MousePosition>()
        .add_systems(Startup, setup)
        .add_plugins(bubbles::BubbleSystems)
        .add_plugins(player::PlayerSystems)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
