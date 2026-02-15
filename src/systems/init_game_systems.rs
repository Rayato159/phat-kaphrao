//! Initial Game Setup Systems
//!
//! Contains systems for initializing the game engine, scene, and game state.
//! These systems run during startup and when entering the InGame state.

use crate::{
    entities::{
        check_list::CheckListMark,
        gauge::{GaugeFollowsPan, RectGauge},
        pan::{EggStepSprite, KapaowStepSprite},
    },
    resource::{
        cooking_state::{EggCookingState, KaprowCookingState},
        game_state::GameState,
        LastDroppedIngredient,
    },
    spawn::{camera_spawn::camera_2d_spawn, spawn_table::table_spawn},
};
use bevy::prelude::*;

/// Phase 1: Core Engine Setup
/// Sets up the 2D camera and initial scene
pub fn setup_camera_and_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let table_sprite = asset_server.load("kitchenware/image/Table.png");
    let pan_kaprow_transform = Transform::from_translation(Vec3::new(0.0, 0.0, -10.0));

    // Spawn a 2D camera for the cooking game
    commands.spawn(camera_2d_spawn());
    commands.spawn(table_spawn(table_sprite.clone(), pan_kaprow_transform));
}

/// Initialize the game with starting values
/// Sets up the GameStats resource for HP and cooking progress
pub fn setup_initial_game_state(mut commands: Commands) {
    // Game Statistics Resource - tracks HP and cooking progress
    commands.init_resource::<GameState>();
    commands.init_resource::<LastDroppedIngredient>();
}

/// Reset game state when entering InGame state
/// This allows for restart functionality
pub fn reset_game_state(
    mut commands: Commands,
    mut game_stats: ResMut<GameState>,
    mut kaprow_state: ResMut<NextState<KaprowCookingState>>,
    mut egg_state: ResMut<NextState<EggCookingState>>,
    q_egg_step_sprites: Query<Entity, With<EggStepSprite>>,
    q_kapaow_step_sprites: Query<Entity, With<KapaowStepSprite>>,
    q_checklist_marks: Query<Entity, With<CheckListMark>>,
    q_rect_gauges: Query<Entity, With<RectGauge>>,
    q_gauge_containers: Query<Entity, With<GaugeFollowsPan>>,
) {
    // Despawn all step sprites on pans
    for entity in q_egg_step_sprites.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_kapaow_step_sprites.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn all marks on checklist
    for entity in q_checklist_marks.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn all gauge entities
    for entity in q_rect_gauges.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_gauge_containers.iter() {
        commands.entity(entity).despawn();
    }

    // Reset cooking states to first step
    kaprow_state.set(KaprowCookingState::Oil);
    egg_state.set(EggCookingState::Oil);

    // Reset game state
    game_stats.hp = game_stats.max_hp;
    game_stats.steps_completed = 0;
    game_stats.kapaow_has_oil = false;
    game_stats.egg_has_oil = false;
    game_stats.ingredient_kapaow_dropped = true;
    game_stats.ingredient_egg_dropped = true;
    game_stats.gauge_container = false;
    game_stats.gauge_container_entity = None;
    game_stats.kpaow_has_guage = false;
    game_stats.egg_has_guage = false;
    game_stats.target_egg_x = None;
    game_stats.target_kapaow_x = None;
    game_stats.count_pud_kapoaw = 0.0;
    game_stats.count_tod_kai = 0.0;
    game_stats.kapaow_is_finished = false;
    game_stats.egg_is_finished = false;

    info!("Game state reset - HP: {}, Step: 0", game_stats.hp);
}
