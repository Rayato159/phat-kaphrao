//! Countdown Timer Entities
//!
//! Contains the marker component for the countdown timer display.

use bevy::prelude::*;

/// Marker component for the countdown timer text
/// This text displays the remaining game time in the center of the screen
#[derive(Component)]
pub struct CountDownTimer;
