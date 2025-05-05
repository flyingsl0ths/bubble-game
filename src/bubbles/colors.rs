use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum BubbleColors {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Orange,
    Blank,
}

impl BubbleColors {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => BubbleColors::Red,
            1 => BubbleColors::Green,
            2 => BubbleColors::Blue,
            3 => BubbleColors::Yellow,
            4 => BubbleColors::Purple,
            5 => BubbleColors::Orange,
            _ => BubbleColors::Blank,
        }
    }
}

#[derive(Resource)]
pub struct ColorMap {
    #[allow(dead_code)]
    bubble_colors: [Handle<ColorMaterial>; 6],
    #[allow(dead_code)]
    blank_color: Handle<ColorMaterial>,
}

impl ColorMap {
    pub fn new(
        bubble_colors: [Handle<ColorMaterial>; 6],
        blank_color: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            bubble_colors,
            blank_color,
        }
    }
}
