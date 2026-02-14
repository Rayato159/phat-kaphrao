use bevy::prelude::*;

#[derive(Component)]
pub struct Heart {
    pub slot_index: u8, // 0..4
    pub frame_index: usize,
}

#[derive(Component)]
pub struct HudRoot;
