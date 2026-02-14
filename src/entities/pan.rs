//! Pan Entities
//!
//! Contains all pan-related marker components used for different cooking stations:
//! - FryingPan: Main frying pan entity
//! - PanKapaow: Pan for the Kapaow (Thai Kaprow Pork) recipe
//! - PanEgg: Pan for the Egg-based recipe

use bevy::prelude::*;

use crate::resource::cooking_state::{EggCookingState, KaprowCookingState};

pub const SCALE: f32 = 10.0; // Scale factor for all pan objects
pub const PAN_SIZE: f32 = 32.0 * SCALE; // 32 is original pan size, scaled up by SCALE
pub const INGREDIENT_SIZE: f32 = 32.0 * SCALE; // 32 is original ingredient size, scaled up by SCALE

/// Marker component for the Kapaow pan
/// Used for the Pad Kapaow (Thai Kaprow Pork Stir-fry) recipe
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

#[derive(Component)]
pub struct KaprowPanStepStateTag(pub KaprowCookingState);

#[derive(Component)]
pub struct KapaowStepSprite;

#[derive(Component)]
pub struct EggPanStepStateTag(pub EggCookingState);

#[derive(Component)]
pub struct EggStepSprite;
