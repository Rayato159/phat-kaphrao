use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::entities::gauge::RectGauge;
use crate::entities::pan::{PanEgg, PanKaphrao};
use crate::helper::random_target_start::random_target_start;

use crate::message::gaug_message::{
    GaugeEggHitMassage, GaugeKaphraoHitMassage, GaugeMissMassage, GaugeSpawnMassage,
};
use crate::resource::game_state::{CookingAudioTimer, GameState};
use crate::spawn::gaueg_spawn::{
    gauge_container_background_spawn, gauge_container_spawn, gauge_rect_spawn,
    gauge_target_zone_spawn,
};

/// Component for the damage flash effect (red screen)
#[derive(Component)]
pub struct DamageFlash {
    pub timer: f32,
}

pub fn spawn_gauge_from_event(
    mut commands: Commands,
    mut game_stats: ResMut<GameState>,
    mut gauge_events: MessageReader<GaugeSpawnMassage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_kaphrao_pan: Query<&PanKaphrao>,
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

        if let Ok(_) = q_kaphrao_pan.get(event.target_pan.unwrap()) {
            if !game_stats.kaphrao_has_gauge {
                game_stats.kaphrao_has_gauge = true;
                game_stats.target_kaphrao_x = Some(start);

                commands.entity(container_entity).with_children(|parent| {
                    parent.spawn(gauge_target_zone_spawn(
                        "kaphrao_gauge",
                        Color::srgb(108.0 / 255.0, 166.0 / 255.0, 81.0 / 255.0),
                        world_width,
                        game_stats.gauge_container_height,
                        world_x,
                    ));
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
    mut hit_kaphrao: MessageWriter<GaugeKaphraoHitMassage>,
    mut hit_egg: MessageWriter<GaugeEggHitMassage>,
    mut miss_event: MessageWriter<GaugeMissMassage>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let position = ball_gauge.position; // normalized
        let zone_width = game_stats.target_width;

        info!("Spacebar pressed, gauge position: {:.3}", position);

        let mut hit_any = false;

        // ----- KAPHRAO -----
        if let Some(start) = game_stats.target_kaphrao_x {
            let end = start + zone_width;

            if check_zone(position, start, end) {
                info!(
                    "Hit Kaphrao target zone! Position {:.3} in range [{:.3}, {:.3}]",
                    position, start, end
                );
                hit_kaphrao.write(GaugeKaphraoHitMassage {});
                hit_any = true;
                game_stats.count_phat_kaphrao += 1.0;
            } else {
                info!(
                    "Missed Kaphrao target zone. Position {:.3} not in range [{:.3}, {:.3}]",
                    position, start, end
                );
            }
        }

        // ----- EGG -----
        if let Some(start) = game_stats.target_egg_x {
            let end = start + zone_width;

            if check_zone(position, start, end) {
                info!(
                    "Hit Egg target zone! Position {:.3} in range [{:.3}, {:.3}]",
                    position, start, end
                );
                hit_egg.write(GaugeEggHitMassage {});
                hit_any = true;
                game_stats.count_tod_kai += 1.0;
            } else {
                info!(
                    "Missed Egg target zone. Position {:.3} not in range [{:.3}, {:.3}]",
                    position, start, end
                );
            }
        }

        if !hit_any {
            game_stats.hp = game_stats.hp.saturating_sub(1);
            info!("Writing GaugeMissMassage - HP reduced to {}", game_stats.hp);
            miss_event.write(GaugeMissMassage);
        } else {
            info!("Hit a target zone, no miss message sent");
        }
    }
}

fn check_zone(position: f32, start: f32, end: f32) -> bool {
    let in_target = position >= start && position <= end;
    in_target
}

/// Spawn red screen flash when the player misses the gauge
pub fn spawn_damage_flash(
    mut commands: Commands,
    mut events: MessageReader<GaugeMissMassage>,
    q_damage_flash: Query<(), With<DamageFlash>>,
) {
    // Only spawn if there isn't already a damage flash active
    if q_damage_flash.iter().count() > 0 {
        info!("Damage flash already active, skipping spawn");
        return;
    }

    let event_count = events.read().count();
    if event_count > 0 {
        info!("Received {} GaugeMissMassage messages", event_count);

        commands.spawn((
            DamageFlash { timer: 0.2 }, // Flash duration in seconds
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.5)), // Red with 50% opacity
            ZIndex(1000), // Very high Z to be on top of everything
            Name::new("DamageFlash"),
        ));
        info!("Spawned damage flash as UI node");
    }
}

/// Fade out and despawn the damage flash effect
pub fn update_damage_flash(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageFlash, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (entity, mut flash, mut bg_color) in query.iter_mut() {
        flash.timer -= time.delta().as_secs_f32();

        // Fade out the alpha value
        let alpha = (flash.timer / 0.2) * 0.5; // 0.2 is the initial duration, 0.5 is initial alpha
        *bg_color = BackgroundColor(Color::srgba(1.0, 0.0, 0.0, alpha.max(0.0)));

        info!(
            "Damage flash timer: {:.3}, alpha: {:.3}",
            flash.timer,
            alpha.max(0.0)
        );

        // Despawn when timer reaches 0
        if flash.timer <= 0.0 {
            commands.entity(entity).despawn();
            info!("Despawned damage flash");
        }
    }
}

/// Despawn target zones when the gauge is hit
/// This removes the colored target zone after a successful hit
/// allowing it to spawn again when the next ingredient is dropped
pub fn despawn_target_zone_on_hit(
    mut commands: Commands,
    mut game_stats: ResMut<GameState>,
    mut audio_timer: ResMut<CookingAudioTimer>,
    mut kaphrao_hit_events: MessageReader<GaugeKaphraoHitMassage>,
    mut egg_hit_events: MessageReader<GaugeEggHitMassage>,
    query: Query<(Entity, &Name)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // Handle Kaphrao gauge hits
    for _ in kaphrao_hit_events.read() {
        if game_stats.target_kaphrao_x.is_some() {
            cooking_audio_play(&mut audio_timer, &asset_server, &audio);
            // Find and despawn the kaphrao target zone
            for (entity, name) in query.iter() {
                if name.as_str() == "kaphrao_gauge" {
                    commands.entity(entity).despawn();
                }
            }
            // Reset the kaphrao gauge state so it can spawn again
            game_stats.kaphrao_has_gauge = false;
            game_stats.target_kaphrao_x = None;
        }
    }

    // Handle Egg gauge hits
    for _ in egg_hit_events.read() {
        if game_stats.target_egg_x.is_some() {
            cooking_audio_play(&mut audio_timer, &asset_server, &audio);
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

pub fn cooking_audio_play(
    audio_timer: &mut ResMut<CookingAudioTimer>,
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
) {
    // Only play if timer has reached 0 (1.5 seconds have passed)
    if audio_timer.timer <= 0.0 {
        audio_timer.timer = 1.5; // Reset timer to 1.5 seconds
        audio
            .play(asset_server.load("audio/Cooking.mp3"))
            .with_volume(-10.)
            .fade_in(AudioTween::new(
                std::time::Duration::from_millis(100),
                AudioEasing::OutPowi(2),
            ));
    }
}

/// Update cooking audio timer - counts down from 3.0 to 0.0
pub fn update_cooking_audio_timer(time: Res<Time>, mut audio_timer: ResMut<CookingAudioTimer>) {
    if audio_timer.timer > 0.0 {
        audio_timer.timer -= time.delta().as_secs_f32();
    }
}
