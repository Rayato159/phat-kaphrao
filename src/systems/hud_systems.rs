//! HUD Systems
//!
//! Contains all systems related to the Heads-Up Display:
//! - setup_hud: Spawns the HUD with HP hearts and step indicators
//! - spawn_hearts_container: Spawns the hearts container for HP display
//! - spawn_kapaow_step_indicator: Spawns the step indicator for Kapaow pan
//! - spawn_egg_step_indicator: Spawns the step indicator for Egg pan

use bevy::prelude::*;

use crate::spawn::hp_group_spawn::{hp_container_child_spawn, hp_container_parent_spawn};
use crate::spawn::setup_hud_spawn::setup_hud_spawn;
use crate::spawn::step_spawn::{
    step_child_current_egg_spawn, step_child_current_kapaow_spawn, step_child_drop_egg_spawn,
    step_child_drop_kapaow_spawn, step_child_spawn, step_egg_parent_spawn,
    step_kapaow_parent_spawn,
};

/// Setup the HUD (Heads-Up Display)
/// Phase 4: Recipe & Progress
pub fn setup_hud(mut commands: Commands) {
    commands.spawn(setup_hud_spawn()).with_children(|parent| {
        spawn_hearts_container(parent);
    });

    // Spawn separate step indicators for each pan
    spawn_kapaow_step_indicator(&mut commands);
    spawn_egg_step_indicator(&mut commands);

    info!("HUD setup complete with 4 HP hearts and separate step indicators");
}

/// Spawn the hearts container for HP display
fn spawn_hearts_container(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn(hp_container_parent_spawn())
        .with_children(|parent| {
            // Spawn 4 heart icons (max HP increased to 4 for 8 ingredients)
            for i in 0..4 {
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
