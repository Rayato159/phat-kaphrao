//! Spatula Animation Systems
//!
//! Contains systems for animating the spatula when the player successfully hits the gauge:
//! - trigger_spatula_animation: Adds animation component to spatulas when gauge is hit
//! - update_spatula_animation: Updates spatula position during cooking animation

use crate::entities::spatula::{EggSpatula, KaphraoSpatula, Spatula, SpatulaAnimation};
use crate::message::gaug_message::{GaugeEggHitMassage, GaugeKaphraoHitMassage};
use bevy::prelude::*;

/// Trigger spatula cooking animation when gauge is hit
/// This system reads gauge hit messages and adds the animation component
/// to the spatula of the activated pan only (determined by x position)
pub fn trigger_spatula_animation(
    mut commands: Commands,
    mut kaphrao_hit_events: MessageReader<GaugeKaphraoHitMassage>,
    mut egg_hit_events: MessageReader<GaugeEggHitMassage>,
    q_kaphrao_spatula: Query<(Entity, &Transform), (With<Spatula>, With<KaphraoSpatula>)>,
    q_egg_spatula: Query<(Entity, &Transform), (With<Spatula>, With<EggSpatula>)>,
) {
    // Handle Kaphrao gauge hits - animate only kaphrao spatula
    for _ in kaphrao_hit_events.read() {
        for (spatula_entity, transform) in q_kaphrao_spatula.iter() {
            commands.entity(spatula_entity).insert(SpatulaAnimation {
                timer: 0.5,
                duration: 0.5,
                original_position: transform.translation,
            });
        }
    }

    // Handle Egg gauge hits - animate only egg spatula
    for _ in egg_hit_events.read() {
        for (spatula_entity, transform) in q_egg_spatula.iter() {
            commands.entity(spatula_entity).insert(SpatulaAnimation {
                timer: 0.5,
                duration: 0.5,
                original_position: transform.translation,
            });
        }
    }
}

/// Update spatula animation
/// This system animates the spatula by moving it back and forth
/// while the animation is active
pub fn update_spatula_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut q_animating_spatulas: Query<(Entity, &mut SpatulaAnimation, &mut Transform)>,
) {
    for (entity, mut animation, mut transform) in q_animating_spatulas.iter_mut() {
        animation.timer -= time.delta().as_secs_f32();

        // Calculate animation progress (0.0 to 1.0)
        let progress = 1.0 - (animation.timer / animation.duration);

        // Oscillate the spatula using a sine wave
        // Move up and down to simulate cooking
        let oscillation = (progress * std::f32::consts::PI * 4.0).sin() * 20.0; // 2 full oscillations

        transform.translation.y = animation.original_position.y + oscillation;

        // Remove animation component when timer reaches 0
        if animation.timer <= 0.0 {
            transform.translation = animation.original_position; // Reset to original position
            commands.entity(entity).remove::<SpatulaAnimation>();
        }
    }
}
