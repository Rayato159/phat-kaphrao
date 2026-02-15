use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::entities::gauge::RectGauge;
use crate::entities::pan::{PanEgg, PanKapaow};
use crate::helper::random_target_start::random_target_start;

use crate::message::gaug_message::{GaugeEggHitMassage, GaugeKaprowHitMassage, GaugeSpawnMassage};
use crate::resource::game_state::GameState;
use crate::spawn::gaueg_spawn::{
    gauge_container_background_spawn, gauge_container_spawn, gauge_rect_spawn,
    gauge_target_zone_spawn,
};

pub fn spawn_gauge_from_event(
    mut commands: Commands,
    mut game_stats: ResMut<GameState>,
    mut gauge_events: MessageReader<GaugeSpawnMassage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_kapaow_pan: Query<&PanKapaow>,
    q_egg_pan: Query<&PanEgg>,
) {
    for event in gauge_events.read() {
        let start = random_target_start(game_stats.target_width);

        let container_entity = if game_stats.gauge_container_entity.is_none() {
            let entity = commands
                .spawn(gauge_container_spawn((-10.0, -400.0, 11.0).into()))
                .id();

            commands.entity(entity).with_children(|parent| {
                parent.spawn(gauge_container_background_spawn(
                    game_stats.gauge_container_width,
                    game_stats.gauge_container_height,
                ));

                parent.spawn(gauge_rect_spawn(
                    Color::srgb(1.0, 0.3, 0.3),
                    &mut *meshes,
                    &mut *materials,
                ));
            });

            game_stats.gauge_container = true;
            game_stats.gauge_container_entity = Some(entity);
            entity
        } else {
            game_stats.gauge_container_entity.unwrap()
        };

        let world_width = game_stats.target_width * game_stats.gauge_container_width;

        let world_x =
            (start + game_stats.target_width / 2.0 - 0.5) * game_stats.gauge_container_width;

        if let Ok(_) = q_kapaow_pan.get(event.target_pan.unwrap()) {
            if !game_stats.kpaow_has_guage {
                game_stats.kpaow_has_guage = true;
                game_stats.target_kapaow_x = Some(start);

                commands.entity(container_entity).with_children(|parent| {
                    let zone_entity = parent
                        .spawn(gauge_target_zone_spawn(
                            "kapaow_gauge",
                            Color::srgb(108.0 / 255.0, 166.0 / 255.0, 81.0 / 255.0),
                            world_width,
                            game_stats.gauge_container_height,
                            world_x,
                        ))
                        .id();

                    parent.commands().entity(zone_entity);
                });
            }
        } else if let Ok(_) = q_egg_pan.get(event.target_pan.unwrap()) {
            if !game_stats.egg_has_guage {
                game_stats.egg_has_guage = true;
                game_stats.target_egg_x = Some(start);

                commands.entity(container_entity).with_children(|parent| {
                    let zone_entity = parent
                        .spawn(gauge_target_zone_spawn(
                            "egg_gauge",
                            Color::srgb(255.0 / 255.0, 209.0 / 255.0, 80.0 / 255.0),
                            world_width,
                            game_stats.gauge_container_height,
                            world_x,
                        ))
                        .id();

                    parent.commands().entity(zone_entity);
                });
            }
        }
    }
}

/// Check if the gauge is in the hit window
/// This is called when the player presses space to hit the gauge
pub fn check_gauge_hit_window(
    mut game_stats: ResMut<GameState>,
    ball_gauge: Single<&mut RectGauge>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hit_kapaow: MessageWriter<GaugeKaprowHitMassage>,
    mut hit_egg: MessageWriter<GaugeEggHitMassage>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let position = ball_gauge.position; // normalized
        let zone_width = game_stats.target_width;

        let mut hit_any = false;

        // ----- KAPAO -----
        if let Some(start) = game_stats.target_kapaow_x {
            let end = start + zone_width;

            if check_zone(position, start, end) {
                hit_kapaow.write(GaugeKaprowHitMassage {});
                hit_any = true;
                game_stats.count_pud_kapoaw += 1.0;
            }
        }

        // ----- EGG -----
        if let Some(start) = game_stats.target_egg_x {
            let end = start + zone_width;

            if check_zone(position, start, end) {
                hit_egg.write(GaugeEggHitMassage {});
                hit_any = true;
                game_stats.count_tod_kai += 1.0;
            }
        }

        if !hit_any {
            game_stats.hp = game_stats.hp.saturating_sub(1);
        }
    }
}

fn check_zone(position: f32, start: f32, end: f32) -> bool {
    let in_target = position >= start && position <= end;
    in_target
}

/// Despawn target zones when the gauge is hit
/// This removes the colored target zone after a successful hit
/// allowing it to spawn again when the next ingredient is dropped
pub fn despawn_target_zone_on_hit(
    mut commands: Commands,
    mut game_stats: ResMut<GameState>,
    mut kaprow_hit_events: MessageReader<GaugeKaprowHitMassage>,
    mut egg_hit_events: MessageReader<GaugeEggHitMassage>,
    query: Query<(Entity, &Name)>,
) {
    // Handle Kapaow gauge hits
    for _ in kaprow_hit_events.read() {
        if game_stats.target_kapaow_x.is_some() {
            // Find and despawn the kapaow target zone
            for (entity, name) in query.iter() {
                if name.as_str() == "kapaow_gauge" {
                    commands.entity(entity).despawn();
                }
            }
            // Reset the kapaow gauge state so it can spawn again
            game_stats.kpaow_has_guage = false;
            game_stats.target_kapaow_x = None;
        }
    }

    // Handle Egg gauge hits
    for _ in egg_hit_events.read() {
        if game_stats.target_egg_x.is_some() {
            // Find and despawn the egg target zone
            for (entity, name) in query.iter() {
                if name.as_str() == "egg_gauge" {
                    commands.entity(entity).despawn();
                }
            }
            // Reset the egg gauge state so it can spawn again
            game_stats.egg_has_guage = false;
            game_stats.target_egg_x = None;
        }
    }
}
