use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
struct PlayerPointText;

#[derive(Component)]
struct TimerText;

#[derive(Event)]
struct PointsUpdateEvent(u32);

#[derive(Event)]
struct TimerUpdateEvent(u32, u32);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PointsUpdateEvent>()
            .add_event::<TimerUpdateEvent>()
            .add_systems(OnEnter(GameState::default()), setup)
            .add_systems(
                Update,
                (update_player_points, update_timer).run_if(in_state(GameState::InGame)),
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
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::new("0"),
            (
                TextFont {
                    font: font.clone_weak(),
                    font_size: 23.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
            ),
            PlayerPointText,
        ));

    commands
        .spawn((
            Text::new(""),
            TextFont {
                font: font.clone_weak(),
                font_size: 20.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(46.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::new("00"),
            (
                TextFont {
                    font: font.clone_weak(),
                    font_size: 23.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
            ),
            TimerText,
        ));

    commands.spawn((
        Text::new(":"),
        TextFont {
            font: font.clone_weak(),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(49.0),
            top: Val::Percent(0.5),
            ..default()
        },
    ));

    commands
        .spawn((
            Text::new(""),
            TextFont {
                font: font.clone_weak(),
                font_size: 20.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.27),
                ..default()
            },
        ))
        .with_child((
            TextSpan::new("00"),
            (
                TextFont {
                    font: font.clone_weak(),
                    font_size: 23.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
            ),
            TimerText,
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

fn update_timer(
    mut query: Query<&mut TextSpan, With<TimerText>>,
    mut update_timer_ev: EventReader<TimerUpdateEvent>,
) {
    let append_to = |unit: u32, buffer: &mut String| {
        if unit >= 10 {
            *buffer = unit.to_string();
        } else {
            *buffer += &unit.to_string();
        }
    };

    for ev in update_timer_ev.read() {
        let mut hour_text = String::from("0");
        let mut minutes_text = String::from("0");

        append_to(ev.0, &mut hour_text);
        append_to(ev.1, &mut minutes_text);

        let mut text_spans = query.iter_mut();

        let mut hours = text_spans.next().unwrap();
        let mut minutes = text_spans.next().unwrap();

        **hours = hour_text;

        **minutes = minutes_text;
    }
}
