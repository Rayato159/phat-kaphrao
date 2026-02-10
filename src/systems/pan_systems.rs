//! Pan Systems
//!
//! Contains all systems related to pan setup and management:
//! - setup_frying_pan: Spawns two pans (Kapaow and Egg) as main characters in the center
//!   These are the main characters positioned in the center for easy ingredient dragging

use bevy::prelude::*;

use crate::spawn::pan_spawn::{pan_egg_spawn, pan_kapaow_spawn};

/// Setup the frying pans as main characters
/// Phase 1: Core Engine Setup
/// Spawns two pans (Kapaow and Egg) in the center as main characters
/// Positioned side by side in the center area for optimal gameplay
/// Makes the pans pickable for drag & drop detection
pub fn setup_frying_pan(mut commands: Commands) {
    // Spawn Kapaow pan (left side of center) - Main character
    commands.spawn(pan_kapaow_spawn());

    // Spawn Egg pan (right side of center) - Main character
    commands.spawn(pan_egg_spawn());

    info!("Two pans spawned as main characters in center:");
    info!("  - PanKapaow at (-150.0, -100.0)");
    info!("  - PanEgg at (150.0, -100.0)");
}
