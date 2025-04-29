use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
struct PlayerPointText;

#[derive(Event)]
struct PointsUpdateEvent(u32);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PointsUpdateEvent>()
            .add_systems(OnEnter(GameState::default()), setup)
            .add_systems(
                Update,
                (update_player_points).run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Joystix.otf");

    commands
        .spawn((
            Text::new("Points:"),
            TextFont {
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::new("0"),
            (
                TextFont {
                    font: font.clone(),
                    font_size: 23.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
            ),
            PlayerPointText,
        ));
}

fn update_player_points(
    mut query: Query<&mut TextSpan, With<PlayerPointText>>,
    mut update_points_ev: EventReader<PointsUpdateEvent>,
) {
    let mut text_span = query.single_mut();

    for ev in update_points_ev.read() {
        **text_span = format!("Points: {}", ev.0);
    }
}
