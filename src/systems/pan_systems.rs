//! Pan Systems
//!
//! Contains all systems related to pan setup and management:
//! - setup_frying_pan: Spawns two pans (Kaphrao and Egg) as main characters in center
//!   These are the main characters positioned in the center for easy ingredient dragging

use bevy::prelude::*;

use crate::{
    entities::{pan, spatula, spatula::*},
    spawn::pan_spawn::{pan_egg_spawn, pan_kaphrao_spawn, spatula_spawn},
};

/// Setup the frying pans as main characters
/// Phase 1: Core Engine Setup
/// Spawns two pans (Kaphrao and Egg) in the center as main characters
/// Positioned side by side in the center area for optimal gameplay
/// Makes the pans pickable for drag & drop detection
pub fn setup_frying_pan(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pan_sprite = asset_server.load("kitchenware/image/Pan.png");
    let spatula_sprite = asset_server.load("kitchenware/image/Spatula.png");
    let kaphrao_pan_text_sprite = asset_server.load("kitchenware/image/KaphraoPanText.png");
    let egg_pan_text_sprite = asset_server.load("kitchenware/image/EggPanText.png");

    let gap = 1.0 * pan::SCALE; // 1.5 is original gap size, scaled up by SCALE

    let pan = pan::PAN_SIZE;

    let pan_kaphrao_transform =
        Transform::from_translation(Vec3::new(-(pan * 0.5 + gap), 0.0, 1.0));
    let pan_egg_transform = Transform::from_translation(Vec3::new(pan * 0.5 + gap, 0.0, 0.0));

    let spatula_offset = 10.0 * spatula::SCALE; // 10.0 is original spatula offset, scaled up by SCALE

    // Spawn Kaphrao pan (left side of center) - Main character
    commands
        .spawn(pan_kaphrao_spawn(pan_sprite.clone(), pan_kaphrao_transform))
        .with_children(|p| {
            p.spawn((
                spatula_spawn(
                    spatula_sprite.clone(),
                    Transform::from_translation(Vec3::new(-spatula_offset, 0.0, 2.0)),
                ),
                KaphraoSpatula,
            ));
        })
        .with_children(|p| {
            p.spawn((
                Sprite {
                    image: kaphrao_pan_text_sprite,
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, -(pan * 0.5 + 120.0), 100.0)),
            ));
        });

    // Spawn Egg pan (right side of center) - Main character
    commands
        .spawn(pan_egg_spawn(pan_sprite.clone(), pan_egg_transform))
        .with_children(|p| {
            p.spawn((
                spatula_spawn(
                    spatula_sprite.clone(),
                    Transform::from_translation(Vec3::new(-spatula_offset, 0.0, 2.0)),
                ),
                EggSpatula,
            ));
        })
        .with_children(|p| {
            p.spawn((
                Sprite {
                    image: egg_pan_text_sprite,
                    ..default()
                },
                Transform::from_translation(Vec3::new(0.0, -(pan * 0.5 + 120.0), 100.0)),
            ));
        });

    info!("Two pans spawned as main characters in center:");
}
