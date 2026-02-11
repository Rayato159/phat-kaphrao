//! Pan Entities
//!
//! Contains all pan-related marker components used for different cooking stations:
//! - FryingPan: Main frying pan entity
//! - PanKapaow: Pan for the Kapaow (Thai Basil Pork) recipe
//! - PanEgg: Pan for the Egg-based recipe

use bevy::prelude::*;

pub const SCALE: f32 = 10.0; // Scale factor for all pan objects
pub const PAN_SIZE: f32 = 32.0 * SCALE; // 32 is original pan size, scaled up by SCALE

/// Marker component for the Kapaow pan
/// Used for the Pad Kapaow (Thai Basil Pork Stir-fry) recipe
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct PanKapaow;

/// Marker component for the Egg pan
/// Used for the Egg-based recipe
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct PanEgg;
