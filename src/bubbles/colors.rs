use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Colors {
    bubble_colors: HashMap<usize, Handle<ColorMaterial>>,
    blank_color: Handle<ColorMaterial>,
}

impl Colors {
    pub fn new(blank_color: Handle<ColorMaterial>) -> Self {
        Self {
            bubble_colors: HashMap::new(),
            blank_color,
        }
    }

    pub fn insert(&mut self, row_column: usize, color: Handle<ColorMaterial>) {
        self.bubble_colors.insert(row_column, color);
    }

    pub fn get(&self, row_column: usize) -> Option<Handle<ColorMaterial>> {
        self.bubble_colors.get(&row_column).map(|h| h.clone_weak())
    }

    pub fn get_blank(&self) -> Handle<ColorMaterial> {
        self.blank_color.clone_weak()
    }
}
