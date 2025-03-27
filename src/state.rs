use bevy::prelude::*;

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Resource, States,
)]
pub enum GameState {
    MainMenu,
    SettingsMenu,
    PauseMenu,
    #[default]
    InGame,
    GameOver,
}
