mod app_constants;
mod bubbles;
mod state;
mod ui;
mod utils;

use bevy::{
    prelude::{
        App, Camera2d, Commands, DefaultPlugins, PluginGroup, Startup, Window,
        WindowPlugin, default,
    },
    state::app::AppExtStates,
    window::PresentMode,
};
use state::GameState;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bobbles".into(),
            name: Some("bobles.app".into()),
            resolution: app_constants::WINDOW_SIZE.into(),
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            // Tells Wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            ..default()
        }),
        ..default()
    }))
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_plugins(ui::UIPlugin)
    .add_plugins(bubbles::plugin::BubblesPlugin);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
