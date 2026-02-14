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
//! - hud_systems: Systems for HUD display (hearts and step indicators)
//! - game_end_systems: Systems for victory and game over screens
//! - menu_systems: Systems for main menu and button interactions
//! - observer_systems: Systems for observing game state changes
//! - init_game_systems: Systems for initial game setup

pub mod check_list_systems;
pub mod cooking_systems;
pub mod egg_cooking_systems;
pub mod game_end_systems;
pub mod gauge_systems;
pub mod hud_systems;
pub mod ingredient_systems;
pub mod init_game_systems;
pub mod kapaow_cooking_systems;
pub mod menu_systems;
pub mod observer_systems;
pub mod pan_systems;
pub mod time_count_down_systems;

// Re-export commonly used items for convenience
pub use cooking_systems::handle_kaprow_pan_ingredient_drop;
pub use game_end_systems::{
    cleanup_game_end_screens, handle_game_over_screen_input, handle_victory_screen_input,
    show_game_over_screen, show_victory_screen,
};
pub use gauge_systems::{check_gauge_hit_window, spawn_gauge_from_event};
pub use hud_systems::{setup_hud, update_hp_text};
pub use ingredient_systems::{spawn_ingredients, update_dragging_ingredient};
pub use init_game_systems::{reset_game_state, setup_camera_and_scene, setup_initial_game_state};
pub use menu_systems::{cleanup_main_menu, handle_menu_button_click, setup_main_menu};
pub use observer_systems::observe_game_state_changes;
pub use pan_systems::setup_frying_pan;
pub use time_count_down_systems::{spawn_countdown_timer, update_countdown_timer};
