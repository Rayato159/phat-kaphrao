//! Gauge Systems
//!
//! Contains all systems related to the timing gauge mini-game:
//! - spawn_ball_gauge: Spawns a ball gauge with randomized target zone
//! - spawn_ball_gauge_kapaow: Spawns the ball gauge for Kapaow pan
//! - spawn_ball_gauge_egg: Spawns the ball gauge for Egg pan
//! - spawn_gauge_from_event: Spawns appropriate gauge based on target pan
//! - update_ball_gauge: Animates the ball gauges
//! - update_gauge_follow_pan: Updates gauge positions to follow their respective pans
//! - check_gauge_hit_window: Detects spacebar hits on the ball gauges
//!
//! Uses Bevy 0.17+ patterns: Event-driven logic, Required Components

use bevy::input::ButtonInput;
use bevy::prelude::*;
use rand::Rng;

use crate::entities::gauge::GaugeSpawnEvent;
use crate::entities::gauge::{BallGauge, BallGaugeEgg, GaugeFollowsPan};
use crate::entities::PanEgg;
use crate::entities::PanKapaow;
use crate::resource::game_state::GameStats;
use crate::spawn::gaueg_spawn::{
    gauge_ball_spawn, gauge_container_background_spawn, gauge_container_spawn,
    gauge_target_zone_spawn,
};

/// Spawn a ball gauge for Egg pan at the specified position
/// Phase 3: Cooking Mini-game
/// Returns the entity ID of the spawned gauge

/// Helper function to spawn a ball gauge with randomized target zone
/// Returns the entity ID of the spawned gauge

/// Spawn appropriate ball gauge from event
/// Listens for GaugeSpawnEvent and spawns the correct gauge based on target_pan
/// Randomizes the target zone position each time
const gauge_width: f32 = 600.0;
const gauge_height: f32 = 40.0;
pub fn spawn_gauge_from_event(
    mut commands: Commands,
    mut game_stats: ResMut<GameStats>,
    mut gauge_events: MessageReader<GaugeSpawnEvent>,
) {
    for event in gauge_events.read() {
        if !game_stats.gauge_container {}
        let target_pan = event.target_pan;
        // Determine which pan type this is and spawn the appropriate gauge
        if !game_stats.kpaow_has_guage && !game_stats.egg_has_guage {
            // This is a Kapaow pan
            // Randomize target zone (0.2 to 0.8, leaving some margin)
            let mut rng = rand::thread_rng();
            let target_width = 0.15; // Width of target zone (15% of gauge)
            let target_zone_start = rng.gen_range(0.1..(1.0 - target_width - 0.1));
            let target_zone_end = target_zone_start + target_width;
            info!(
                "Spawning Kapaow gauge {:?} at position {:?}",
                target_zone_start, event.position
            );
            game_stats.kpaow_has_guage = true;
            commands
                .spawn(gauge_container_spawn((-10.0, -400.0, 11.0).into()))
                .with_children(|parent| {
                    // Background bar ONLY

                    parent.spawn(gauge_container_background_spawn(gauge_width, gauge_height));
                    // parent.spawn(gauge_target_zone_spawn(
                    //     zone_name,
                    //     target_width,
                    //     gauge_width,
                    //     gauge_height,
                    //     target_x,
                    // ));
                    parent.spawn(gauge_ball_spawn(gauge_height, Color::WHITE));
                });
        }
    }
}

/// Update the ball gauge animations
/// Moves the balls back and forth continuously
pub fn update_ball_gauge(
    q_gauge: Single<&mut BallGauge>,
    mut q_ball: Query<&mut Transform, With<BallGauge>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    let mut gauge = q_gauge.into_inner();
    let mut new_position = gauge.position + gauge.direction * gauge.speed * delta;
    if new_position >= 1.0 {
        new_position = 1.0;
        gauge.direction = -1.0;
    } else if new_position <= 0.0 {
        new_position = 0.0;
        gauge.direction = 1.0;
    }
    gauge.position = new_position;

    for mut transform in q_ball.iter_mut() {
        transform.translation.x = (new_position - 0.5) * gauge_width;
    }
}

/// Check if the gauge is in the hit window
/// This is called when the player presses space to hit the gauge
pub fn check_gauge_hit_window(
    mut ball_gauge: Query<(Entity, &mut BallGauge)>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Check for spacebar press
    if keyboard.just_pressed(KeyCode::Space) {
        // Check Kapaow ball gauges
        for (entity, mut gauge) in ball_gauge.iter_mut() {
            if !gauge.has_hit {
                let position = gauge.position;
                let target_start = gauge.target_zone_start;
                let target_end = gauge.target_zone_end;
                gauge.has_hit = true;
                check_gauge_hit(
                    &mut commands,
                    entity,
                    position,
                    target_start,
                    target_end,
                    "Kapaow",
                    &mut gauge.is_success,
                );
            }
        }
    }
}

/// Helper function to check if gauge hit is successful
fn check_gauge_hit(
    commands: &mut Commands,
    entity: Entity,
    position: f32,
    target_start: f32,
    target_end: f32,
    name: &str,
    is_success: &mut bool,
) {
    // Check if ball is in the target zone
    let in_target = position >= target_start && position <= target_end;

    // Determine if it's a perfect hit (very close to center)
    let center = (target_start + target_end) / 2.0;
    let is_perfect = (position - center).abs() < 0.05;

    *is_success = in_target;

    info!(
        "Spacebar pressed on {} gauge - In target: {}, Perfect: {}, Position: {:.2}, Target: {:.2}-{:.2}",
        name, in_target, is_perfect, position, target_start, target_end
    );

    // Despawn the gauge after the hit
    // commands.entity(entity).despawn();
}
