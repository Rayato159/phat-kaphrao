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
mod entities;
mod resource;
mod spawn;
mod systems;

// Re-export commonly used items for convenience
use entities::GaugeSpawnEvent;
use pad_kaprao::{GAME_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};
use resource::game_state::{
    AppState, GameLoseEvent, GameWinEvent, InGame, IngredientDroppedEvent, StepCompletedEvent,
};
use spawn::gaueg_spawn::{gaueg_bar_spawn, spawn_ball_gauge_kapaow};
use systems::{
    check_gauge_hit_window, handle_ingredient_drop, handle_menu_button_click, reset_game_state,
    setup_camera_and_scene, setup_frying_pan, setup_hud, setup_initial_game_state, setup_main_menu,
    spawn_gauge_from_event, spawn_ingredients, update_ball_gauge, update_dragging_ingredient,
    update_gauge_follow_pan, update_hud,
};

fn main() {
    App::new()
        // Bevy 0.17+ Setup with Default Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_TITLE.to_string(),
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        // Initialize the main game state
        .init_state::<AppState>()
        // Add computed state - InGame is automatically computed based on AppState
        // This follows Bevy 0.16+ ComputedStates pattern
        .add_computed_state::<InGame>()
        // Register all game events
        .add_message::<GaugeSpawnEvent>()
        .add_message::<GameWinEvent>()
        .add_message::<GameLoseEvent>()
        .add_message::<StepCompletedEvent>()
        .add_message::<IngredientDroppedEvent>()
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
        .add_systems(
            Update,
            (
                handle_ingredient_drop,
                spawn_gauge_from_event,
                update_ball_gauge,
                update_gauge_follow_pan,
                check_gauge_hit_window,
            )
                .run_if(in_state(InGame)),
        )
        // System Schedules - Ingredient Systems
        .add_systems(Update, update_dragging_ingredient.run_if(in_state(InGame)))
        // System Schedules - UI Systems
        .add_systems(Update, update_hud.run_if(in_state(InGame)))
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
