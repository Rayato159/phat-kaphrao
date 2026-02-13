//! Pad Kaprao - A Thai Cooking Game
//!
//! An interactive cooking game where you must add ingredients in the correct order:
//! Oil -> Garlic -> Pork -> Egg -> Fish Sauce -> Oyster Sauce -> Thai Chilli -> Holy Basil Leaves
//!
//! Features:
//! - Drag & Drop mechanics using Bevy's Picking API
//! - Timing Gauge mini-game when adding oil
//! - HP system and step tracking
//! - Observer-based event handling
//! - 8 ingredients in a 2x4 grid layout
//!
//! Uses Bevy 0.17+ with modern patterns:
//! - Observers & Triggers
//! - Required Components
//! - Computed States
//! - Picking API

use bevy::{prelude::*, window::WindowMode};

// New module structure
mod animate;
mod entities;
mod helper;
mod message;
mod resource;
mod spawn;
mod systems;

// Re-export commonly used items for convenience

use bevy_spritesheet_animation::prelude::*;
use pad_kaprao::{GAME_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};

use resource::game_state::{AppState, InGame};
use systems::{
    check_gauge_hit_window, handle_kaprow_pan_ingredient_drop, handle_menu_button_click,
    reset_game_state, setup_camera_and_scene, setup_frying_pan, setup_hud,
    setup_initial_game_state, setup_main_menu, spawn_gauge_from_event, spawn_ingredients,
    update_dragging_ingredient,
};

use crate::{
    animate::gauge_animate::moving_ball_gauge_animation,
    message::{
        game_message::{GameLoseMessage, GameWinMessage},
        gaug_message::{GaugeKapoawHitMassage, GaugeSpawnMassage},
        ingredient_message::IngredientDroppedMessage,
    },
    resource::time_state::{check_game_timer, start_timer},
    spawn::step_spawn::{EggCookingState, KaprowCookingState},
    systems::kapaow_cooking_systems::next_step_kapaow_cooking,
};

fn main() {
    App::new()
        // Bevy 0.17+ Setup with Default Plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_TITLE.to_string(),
                        resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SpritesheetAnimationPlugin)
        // Initialize the main game state
        .init_state::<AppState>()
        .init_state::<KaprowCookingState>()
        .init_state::<EggCookingState>()
        // Add computed state - InGame is automatically computed based on AppState
        // This follows Bevy 0.16+ ComputedStates pattern
        .add_computed_state::<InGame>()
        // Register all game events
        .add_message::<GaugeSpawnMassage>()
        .add_message::<GameWinMessage>()
        .add_message::<GameLoseMessage>()
        // .add_message::<StepCompletedEvent>()
        .add_message::<IngredientDroppedMessage>()
        .add_message::<GaugeKapoawHitMassage>()
        // System Schedules - Startup
        .add_systems(
            Startup,
            (
                setup_camera_and_scene,
                setup_initial_game_state,
                spawn_ingredients.after(setup_initial_game_state),
                setup_frying_pan,
                setup_hud,
            ),
        )
        // System Schedules - Gameplay Systems
        .add_systems(OnEnter(InGame), start_timer)
        .add_systems(
            Update,
            (
                handle_kaprow_pan_ingredient_drop,
                spawn_gauge_from_event,
                moving_ball_gauge_animation,
                check_gauge_hit_window,
                check_game_timer,
            )
                .run_if(in_state(InGame)),
        )
        // System Schedules - Ingredient Systems
        .add_systems(Update, update_dragging_ingredient.run_if(in_state(InGame)))
        .add_systems(Update, next_step_kapaow_cooking.run_if(in_state(InGame)))
        // System Schedules - Game State Transitions
        .add_systems(OnEnter(AppState::InGame), reset_game_state)
        .add_systems(OnEnter(AppState::Victory), systems::show_victory_screen)
        .add_systems(OnEnter(AppState::GameOver), systems::show_game_over_screen)
        .add_systems(OnExit(AppState::Victory), systems::cleanup_game_end_screens)
        .add_systems(
            OnExit(AppState::GameOver),
            systems::cleanup_game_end_screens,
        )
        // System Schedules - Game End Screen Input
        .add_systems(
            Update,
            systems::handle_victory_screen_input.run_if(in_state(AppState::Victory)),
        )
        .add_systems(
            Update,
            systems::handle_game_over_screen_input.run_if(in_state(AppState::GameOver)),
        )
        // System Schedules - Main Menu
        .add_systems(OnEnter(AppState::Menu), setup_main_menu)
        .add_systems(OnExit(AppState::Menu), systems::cleanup_main_menu)
        .add_systems(
            Update,
            handle_menu_button_click.run_if(in_state(AppState::Menu)),
        )
        // System Schedules - Game State Observers
        .add_systems(Update, systems::observe_game_state_changes)
        // Run the game
        .run();
}
