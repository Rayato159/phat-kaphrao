use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::entities::gauge::BallGauge;
use crate::entities::gauge::GaugeSpawnEvent;
use crate::entities::PanEgg;
use crate::entities::PanKapaow;
use crate::helper::random_target_start::random_target_start;
use crate::resource::game_state::GameStats;
use crate::spawn::gaueg_spawn::{
    gauge_ball_spawn, gauge_container_background_spawn, gauge_container_spawn,
    gauge_target_zone_spawn, guage_perfect_spawn,
};

pub fn spawn_gauge_from_event(
    mut commands: Commands,
    mut game_stats: ResMut<GameStats>,
    mut gauge_events: MessageReader<GaugeSpawnEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_kapaow_pan: Query<&PanKapaow>,
    q_egg_pan: Query<&PanEgg>,
) {
    for event in gauge_events.read() {
        // ✅ normalized start
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

                parent.spawn(gauge_ball_spawn(
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

        // 🔥 convert normalized → world (render only)
        let world_width = game_stats.target_width * game_stats.gauge_container_width;

        let world_x =
            (start + game_stats.target_width / 2.0 - 0.5) * game_stats.gauge_container_width;

        if let Ok(_) = q_kapaow_pan.get(event.target_pan.unwrap()) {
            if !game_stats.kpaow_has_guage {
                game_stats.kpaow_has_guage = true;
                game_stats.target_kapaow_x = Some(start); // ✅ เก็บ normalized

                commands.entity(container_entity).with_children(|parent| {
                    let zone_entity = parent
                        .spawn(gauge_target_zone_spawn(
                            "kapaow_gauge",
                            world_width,
                            game_stats.gauge_container_height,
                            world_x,
                        ))
                        .id();

                    // ✅ spawn เส้นแดงเป็น child ของ zone
                    parent.commands().entity(zone_entity).with_children(|zone| {
                        zone.spawn(guage_perfect_spawn(
                            "perfect_loibe1",
                            game_stats.gauge_container_height,
                        ));
                    });
                });
            }
        } else if let Ok(_) = q_egg_pan.get(event.target_pan.unwrap()) {
            if !game_stats.egg_has_guage {
                game_stats.egg_has_guage = true;
                game_stats.target_egg_x = Some(start); // ✅ เก็บ normalized

                commands.entity(container_entity).with_children(|parent| {
                    let zone_entity = parent
                        .spawn(gauge_target_zone_spawn(
                            "egg_gauge",
                            world_width,
                            game_stats.gauge_container_height,
                            world_x,
                        ))
                        .id();
                    // ✅ spawn เส้นแดงเป็น child ของ zone
                    parent.commands().entity(zone_entity).with_children(|zone| {
                        zone.spawn(guage_perfect_spawn(
                            "perfect_loibe2",
                            game_stats.gauge_container_height,
                        ));
                    });
                });
            }
        }
    }
}

/// Check if the gauge is in the hit window
/// This is called when the player presses space to hit the gauge
pub fn check_gauge_hit_window(
    mut game_stats: ResMut<GameStats>,
    ball_gauge: Single<&mut BallGauge>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let position = ball_gauge.position; // normalized
        let zone_width = game_stats.target_width;

        let mut hit_any = false;

        // ----- KAPAO -----
        if let Some(start) = game_stats.target_kapaow_x {
            let end = start + zone_width;

            if check_zone(position, start, end, "Kapaow") {
                hit_any = true;
                game_stats.cout_pud_kapoaw += 1.0;
            }
        }

        // ----- EGG -----
        if let Some(start) = game_stats.target_egg_x {
            let end = start + zone_width;

            if check_zone(position, start, end, "Egg") {
                hit_any = true;
                game_stats.cout_tod_kai += 1.0;
            }
        }

        if !hit_any {
            info!("❌ MISS! ลด HP");
            game_stats.hp = game_stats.hp.saturating_sub(1);
        }
    }
}

fn check_zone(position: f32, start: f32, end: f32, name: &str) -> bool {
    let in_target = position >= start && position <= end;

    let center = (start + end) / 2.0;
    let is_perfect = (position - center).abs() < 0.0;
    info!("start{} -> end: {}", start, end);
    info!(
        "{} -> center: {}, position: {}, diff:{}",
        name,
        center,
        position,
        (position - center).abs()
    );

    info!("{} -> Hit: {}, Perfect: {}", name, in_target, is_perfect);

    in_target
}
