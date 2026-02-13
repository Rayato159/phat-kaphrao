//! Initial Game Setup Systems
//!
//! Contains systems for initializing the game engine, scene, and game state.
//! These systems run during startup and when entering the InGame state.

use crate::{
    resource::game_state::GameStats,
    spawn::{camera_spawn::camera_2d_spawn, spawn_table::table_spawn},
};
use bevy::prelude::*;

/// Phase 1: Core Engine Setup
/// Sets up the 2D camera and initial scene
pub fn setup_camera_and_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let table_sprite = asset_server.load("kitchenware/image/Table.png");
    let pan_kaprow_transform = Transform::from_translation(Vec3::new(0.0, 0.0, -10.0));

    // Spawn a 2D camera for the cooking game
    commands.spawn(camera_2d_spawn());
    commands.spawn(table_spawn(table_sprite.clone(), pan_kaprow_transform));
}

/// Initialize the game with starting values
/// Sets up the GameStats resource for HP and cooking progress
pub fn setup_initial_game_state(mut commands: Commands) {
    // Game Statistics Resource - tracks HP and cooking progress
    commands.init_resource::<GameStats>();
}

/// Reset game state when entering InGame state
/// This allows for restart functionality
pub fn reset_game_state(mut game_stats: ResMut<GameStats>, mut commands: Commands) {
    game_stats.hp = game_stats.max_hp;
    // game_stats.current_step = 0;
    game_stats.steps_completed = 0;

    info!("Game state reset - HP: {}, Step: 0", game_stats.hp);
}
