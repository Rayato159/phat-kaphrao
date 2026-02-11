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
use crate::entities::gauge::{BallGaugeEgg, BallGaugeKapaow, GaugeFollowsPan};
use crate::entities::PanEgg;
use crate::entities::PanKapaow;
use crate::resource::game_state::GameStats;
use crate::{spawn_ball_gauge_egg, spawn_ball_gauge_kapaow};

/// Spawn a ball gauge for Egg pan at the specified position
/// Phase 3: Cooking Mini-game
/// Returns the entity ID of the spawned gauge

/// Helper function to spawn a ball gauge with randomized target zone
/// Returns the entity ID of the spawned gauge

/// Spawn appropriate ball gauge from event
/// Listens for GaugeSpawnEvent and spawns the correct gauge based on target_pan
/// Randomizes the target zone position each time
pub fn spawn_gauge_from_event(
    mut commands: Commands,
    mut game_stats: ResMut<GameStats>,
    mut gauge_events: MessageReader<GaugeSpawnEvent>,
    q_kapaow_pans: Query<Entity, With<PanKapaow>>,
    q_egg_pans: Query<Entity, With<PanEgg>>,
) {
    for event in gauge_events.read() {
        let mut gauge_entity = None;

        if let Some(target_pan) = event.target_pan {
            // Determine which pan type this is and spawn the appropriate gauge
            if q_kapaow_pans.contains(target_pan) && !game_stats.kpaow_has_guage {
                // This is a Kapaow pan
                // Randomize target zone (0.2 to 0.8, leaving some margin)
                let mut rng = rand::thread_rng();
                let target_width = 0.15; // Width of target zone (15% of gauge)
                let target_zone_start = rng.gen_range(0.1..(1.0 - target_width - 0.1));
                let target_zone_end = target_zone_start + target_width;
                game_stats.kpaow_has_guage = true;
                gauge_entity = Some(spawn_ball_gauge_kapaow(
                    &mut commands.reborrow(),
                    event.position,
                    target_zone_start,
                    target_zone_end,
                ));
            } else if q_egg_pans.contains(target_pan) && !game_stats.egg_has_guage {
                // This is an Egg pan
                // Randomize target zone (0.2 to 0.8, leaving some margin)
                let mut rng = rand::thread_rng();
                let target_width = 0.15; // Width of target zone (15% of gauge)
                let target_zone_start = rng.gen_range(0.1..(1.0 - target_width - 0.1));
                let target_zone_end = target_zone_start + target_width;
                game_stats.egg_has_guage = true;
                gauge_entity = Some(spawn_ball_gauge_egg(
                    &mut commands.reborrow(),
                    event.position,
                    target_zone_start,
                    target_zone_end,
                ));
            }
        } else {
            // No target pan specified, spawn Kapaow gauge by default
            // Randomize target zone (0.2 to 0.8, leaving some margin)
            let mut rng = rand::thread_rng();
            let target_width = 0.15; // Width of target zone (15% of gauge)
            let target_zone_start = rng.gen_range(0.1..(1.0 - target_width - 0.1));
            let target_zone_end = target_zone_start + target_width;
            gauge_entity = Some(spawn_ball_gauge_kapaow(
                &mut commands.reborrow(),
                event.position,
                target_zone_start,
                target_zone_end,
            ));
        }

        // Set the GaugeFollowsPan component if we have a target pan
        if let (Some(gauge_id), Some(target_pan)) = (gauge_entity, event.target_pan) {
            commands.entity(gauge_id).insert(GaugeFollowsPan {
                pan_entity: target_pan,
            });
        }
    }
}

/// Update the ball gauge animations
/// Moves the balls back and forth continuously
pub fn update_ball_gauge(
    mut q_kapaow_gauges: Query<(&mut BallGaugeKapaow, &Children)>,
    mut q_egg_gauges: Query<(&mut BallGaugeEgg, &Children), Without<BallGaugeKapaow>>,
    mut q_indicators: Query<&mut Transform, With<Name>>,
    time: Res<Time>,
) {
    let delta = time.delta();

    // Update Kapaow ball gauges
    for (mut gauge, children) in q_kapaow_gauges.iter_mut() {
        update_gauge_animation(
            &mut *gauge,
            delta.as_secs_f32(),
            "Kapaow",
            children,
            &mut q_indicators,
        );
    }

    // Update Egg ball gauges
    for (mut gauge, children) in q_egg_gauges.iter_mut() {
        update_gauge_animation(
            &mut *gauge,
            delta.as_secs_f32(),
            "Egg",
            children,
            &mut q_indicators,
        );
    }
}

/// Helper function to update gauge animation and visual ball position
fn update_gauge_animation<T>(
    gauge: &mut T,
    delta_seconds: f32,
    _name: &str,
    children: &Children,
    q_indicators: &mut Query<&mut Transform, With<Name>>,
) where
    T: GaugeData,
{
    // Extract values before mutable borrow
    let direction = gauge.direction();
    let speed = gauge.speed();

    // Update ball position
    let position = gauge.position();
    let new_position = position + direction * speed * delta_seconds;

    // Bounce off edges
    let (new_position, new_direction) = if new_position >= 1.0 {
        (1.0, -1.0)
    } else if new_position <= 0.0 {
        (0.0, 1.0)
    } else {
        (new_position, direction)
    };

    // Update position and direction in gauge
    *gauge.position_mut() = new_position;
    if new_direction != direction {
        gauge.set_direction(new_direction);
    }

    // Update the visual ball position
    // Find the moving ball (z = 0.2)
    for child in children.iter() {
        if let Ok(indicator_transform) = q_indicators.get(child) {
            if indicator_transform.translation.z == 0.2 {
                if let Ok(mut transform) = q_indicators.get_mut(child) {
                    let gauge_width = 300.0;
                    let ball_x = (new_position - 0.5) * gauge_width;
                    transform.translation.x = ball_x;
                }
                break;
            }
        }
    }
}

/// Update gauge positions to follow their respective pans
/// Keeps the gauges positioned above the pans
pub fn update_gauge_follow_pan(
    mut transform_queries: ParamSet<(
        Query<(Entity, &mut Transform, &GaugeFollowsPan)>,
        Query<&Transform, With<crate::entities::Pan>>,
    )>,
) {
    // Collect all gauges and their target pans
    let mut gauges_to_update: Vec<(Entity, Entity)> = Vec::new();

    for (gauge_entity, _, gauge_follows) in transform_queries.p0().iter() {
        gauges_to_update.push((gauge_entity, gauge_follows.pan_entity));
    }

    // Collect all pan transforms
    let mut pan_data: Vec<(Entity, Vec3)> = Vec::new();
    for (gauge_entity, pan_entity) in gauges_to_update.iter() {
        if let Ok(pan_transform) = transform_queries.p1().get(*pan_entity) {
            pan_data.push((*gauge_entity, pan_transform.translation));
        }
    }

    // Now update each gauge using the collected pan translations
    for (gauge_entity, pan_translation) in pan_data.iter() {
        if let Ok((_gauge_entity, mut gauge_transform, _gauge_follows)) =
            transform_queries.p0().get_mut(*gauge_entity)
        {
            // Position the gauge 150 pixels above the pan
            gauge_transform.translation.x = pan_translation.x;
            gauge_transform.translation.y = pan_translation.y + 150.0;
            gauge_transform.translation.z = 10.0;
        }
    }
}

/// Check if the gauge is in the hit window
/// This is called when the player presses space to hit the gauge
pub fn check_gauge_hit_window(
    mut q_kapaow_gauges: Query<(Entity, &mut BallGaugeKapaow)>,
    mut q_egg_gauges: Query<(Entity, &mut BallGaugeEgg), Without<BallGaugeKapaow>>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Check for spacebar press
    if keyboard.just_pressed(KeyCode::Space) {
        // Check Kapaow ball gauges
        for (entity, mut gauge) in q_kapaow_gauges.iter_mut() {
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

        // Check Egg ball gauges
        for (entity, mut gauge) in q_egg_gauges.iter_mut() {
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
                    "Egg",
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

/// Trait for common gauge data operations
trait GaugeData {
    fn position(&self) -> f32;
    fn position_mut(&mut self) -> &mut f32;
    fn direction(&self) -> f32;
    fn speed(&self) -> f32;
    fn set_direction(&mut self, direction: f32);
}

impl GaugeData for BallGaugeKapaow {
    fn position(&self) -> f32 {
        self.position
    }

    fn position_mut(&mut self) -> &mut f32 {
        &mut self.position
    }

    fn direction(&self) -> f32 {
        self.direction
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }
}

impl GaugeData for BallGaugeEgg {
    fn position(&self) -> f32 {
        self.position
    }

    fn position_mut(&mut self) -> &mut f32 {
        &mut self.position
    }

    fn direction(&self) -> f32 {
        self.direction
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }
}
