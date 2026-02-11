//! Pan Entities
//!
//! Contains all pan-related marker components used for different cooking stations:
//! - FryingPan: Main frying pan entity
//! - PanKapaow: Pan for the Kapaow (Thai Basil Pork) recipe
//! - PanEgg: Pan for the Egg-based recipe

use bevy::prelude::*;

/// Marker component for the frying pan
/// Uses Required Components to ensure the pan always has Transform and Visibility
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct FryingPan;

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

/// Marker component for the frying pan drop area
#[derive(Component)]
pub struct Pan;
