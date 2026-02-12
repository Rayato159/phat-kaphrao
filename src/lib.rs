//! Pad Kaprao - A Thai Cooking Game
//!
//! This library contains all the game logic, systems, and components
//! for the Pad Kaprao cooking game.
//!
//! Features:
//! - 8 ingredients in a 2x4 grid layout
//! - Drag & Drop mechanics using Bevy's Picking API
//! - Timing Gauge mini-game when adding oil
//! - HP system (4 hearts) and step tracking
//! - Observer-based event handling
//!
//! Uses Bevy 0.16+ with modern patterns:
//! - Observers & Triggers
//! - Required Components
//! - Computed States
//! - Picking API

// Public modules
pub mod animate;
pub mod entities;
pub mod helper;
pub mod resource;
pub mod spawn;
pub mod systems;
// Re-export commonly used items for convenience
//
//

pub use entities::{
    Dragging, GaugeSpawnEvent, Ingredient, IngredientType, OriginalPosition, PanArea, TimingGauge,
    HUD,
};
pub use resource::game_state::{
    AppState, GameLoseEvent, GameStats, GameWinEvent, InGame, IngredientDroppedEvent,
    StepCompletedEvent,
};

pub use systems::{
    check_gauge_hit_window, cleanup_game_end_screens, handle_game_over_screen_input,
    handle_ingredient_drop, handle_victory_screen_input, observe_game_state_changes, on_drag_end,
    on_drag_start, setup_frying_pan, setup_hud, show_game_over_screen, show_victory_screen,
    spawn_gauge_from_event, spawn_ingredients, update_dragging_ingredient, update_hud,
};

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const GAME_TITLE: &str = "Pad Kaprao - Thai Cooking Game";
