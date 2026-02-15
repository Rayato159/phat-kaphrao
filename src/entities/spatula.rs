use bevy::prelude::*;

pub const SCALE: f32 = 10.0; // Scale factor for all pan objects

#[derive(Component)]
pub struct Spatula;

/// Marker component for Kapaow pan's spatula
#[derive(Component)]
pub struct KapaowSpatula;

/// Marker component for Egg pan's spatula
#[derive(Component)]
pub struct EggSpatula;

/// Component for animating spatula cooking movement
#[derive(Component)]
pub struct SpatulaAnimation {
    pub timer: f32,
    pub duration: f32,
    pub original_position: Vec3,
}
