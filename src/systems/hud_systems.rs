//! HUD Systems
//!
//! Contains all systems related to the Heads-Up Display:
//! - setup_hud: Spawns the HUD with HP hearts and step indicators
//! - spawn_hearts_container: Spawns the hearts container for HP display
//! - spawn_kapaow_step_indicator: Spawns the step indicator for Kapaow pan
//! - spawn_egg_step_indicator: Spawns the step indicator for Egg pan
//! - update_hp_text: Updates the HP text display to show current HP

use bevy::prelude::*;

use crate::entities::HPText;
use crate::resource::game_state::GameState;
use crate::spawn::hp_group_spawn::{
    hp_container_child_spawn, hp_container_parent_spawn, hp_text_spawn,
};
use crate::spawn::setup_hud_spawn::setup_hud_spawn;
use crate::spawn::step_spawn::{
    step_child_current_egg_spawn, step_child_current_kapaow_spawn, step_child_drop_egg_spawn,
    step_child_drop_kapaow_spawn, step_child_spawn, step_egg_parent_spawn,
    step_kapaow_parent_spawn,
};

/// Setup the HUD (Heads-Up Display)
/// Phase 4: Recipe & Progress
pub fn setup_hud(mut commands: Commands, game_stats: Res<GameState>) {
    commands.spawn(setup_hud_spawn()).with_children(|parent| {
        spawn_hearts_container(parent, game_stats.hp, game_stats.max_hp);
    });

    // Spawn separate step indicators for each pan
    spawn_kapaow_step_indicator(&mut commands);
    spawn_egg_step_indicator(&mut commands);

    info!(
        "HUD setup complete with {} HP hearts and separate step indicators",
        game_stats.max_hp
    );
}

/// Spawn the hearts container for HP display
fn spawn_hearts_container(builder: &mut ChildSpawnerCommands, current_hp: u8, max_hp: u8) {
    builder
        .spawn(hp_container_parent_spawn())
        .with_children(|parent| {
            // Spawn HP text display (e.g., "10/10")
            parent.spawn(hp_text_spawn(current_hp, max_hp));

            // Spawn heart icons based on max_hp (10 hearts)
            for i in 0..max_hp {
                parent.spawn(hp_container_child_spawn(i));
            }
        });
}

/// Spawn the step indicator for Kapaow pan
/// Positioned on the left side of the screen
fn spawn_kapaow_step_indicator(commands: &mut Commands) {
    commands
        .spawn(step_kapaow_parent_spawn())
        .with_children(|parent| {
            // "Next Ingredient:" label
            parent.spawn(step_child_spawn());
            parent.spawn(step_child_current_kapaow_spawn());
            parent.spawn(step_child_drop_kapaow_spawn());
        });
}

/// Spawn the step indicator for Egg pan
/// Positioned on the right side of the screen
fn spawn_egg_step_indicator(commands: &mut Commands) {
    commands
        .spawn(step_egg_parent_spawn())
        .with_children(|parent| {
            // "Next Ingredient:" label
            parent.spawn(step_child_spawn());
            parent.spawn(step_child_current_egg_spawn());
            parent.spawn(step_child_drop_egg_spawn());
        });
}

/// Update the HP text display to show current HP
/// This system runs every frame during gameplay to keep the HP text up to date
pub fn update_hp_text(
    game_stats: Res<GameState>,
    mut hp_text_query: Query<&mut Text, With<HPText>>,
) {
    for mut text in hp_text_query.iter_mut() {
        text.0 = format!("{}/{}", game_stats.hp, game_stats.max_hp);
    }
}
