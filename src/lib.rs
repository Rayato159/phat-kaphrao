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
pub mod logic;
pub mod message;
pub mod resource;
pub mod spawn;
pub mod systems;

// Re-export commonly used items for convenience
//
//

pub use entities::{Dragging, Ingredient, IngredientType, OriginalPosition, PanArea, TimingGauge};
pub use message::{
    game_message::{GameLoseMessage, GameWinMessage},
    gaug_message::{GaugeEggHitMassage, GaugeKapoawHitMassage, GaugeSpawnMassage},
    ingredient_message::IngredientDroppedMessage,
};

pub use resource::game_state::{AppState, GameState, InGame};
pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const GAME_TITLE: &str = "Pad Kaprao - Thai Cooking Game";
