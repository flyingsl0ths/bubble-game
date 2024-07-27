use bevy::prelude::*;

pub struct BubbleSystems;

impl Plugin for BubbleSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup() {}
