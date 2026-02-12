//! Systems Module
//!
//! Contains all system functions that operate on entities and game state.
//! Systems are the game logic that runs every frame or on specific events.
//!
//! Modules:
//! - ingredient_systems: Systems for spawning, dragging, and dropping ingredients
//! - cooking_systems: Systems for cooking logic and game mechanics
//! - pan_systems: Systems for pan setup and management
//! - gauge_systems: Systems for timing gauge mini-game
//! - ui_systems: Systems for HUD updates, game end screens, and UI interactions
//! - init_game_systems: Systems for initial game setup

pub mod cooking_systems;
pub mod gauge_systems;
pub mod ingredient_systems;
pub mod init_game_systems;
pub mod pan_systems;
pub mod ui_systems;

// Re-export commonly used items for convenience
pub use cooking_systems::handle_ingredient_drop;
pub use gauge_systems::{check_gauge_hit_window, spawn_gauge_from_event};
pub use ingredient_systems::{
    on_drag_end, on_drag_start, on_hover_end, on_hover_start, spawn_ingredients,
    update_dragging_ingredient,
};
pub use init_game_systems::{reset_game_state, setup_camera_and_scene, setup_initial_game_state};
pub use pan_systems::setup_frying_pan;
pub use ui_systems::{
    cleanup_game_end_screens, cleanup_main_menu, handle_game_over_screen_input,
    handle_menu_button_click, handle_victory_screen_input, observe_game_state_changes, setup_hud,
    setup_main_menu, show_game_over_screen, show_victory_screen, update_hud,
};
