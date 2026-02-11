//! Game State Module
//!
//! Contains all game state definitions, resources, and events:
//! - AppState: Main application states (Menu, InGame, GameOver, Victory)
//! - InGame: Computed state that exists only during gameplay
//! - GameStats: Resource tracking HP and cooking progress
//! - Game Events: Events for win/lose conditions and step completion
//!
//! Uses Bevy 0.17+ ComputedStates for automatic state derivation

use bevy::prelude::*;

/// Main application state for the game
/// This is the primary state that controls game flow
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    GameOver,
    Victory,
}

/// Computed state that exists only when we're in the InGame state
/// This is automatically computed from AppState using Bevy's ComputedStates
/// Systems can use `InState<InGame>` to run only during gameplay
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InGame;

impl ComputedStates for InGame {
    /// The source state from which InGame is derived
    /// We derive from AppState
    type SourceStates = AppState;

    /// Compute function that determines when InGame state exists
    /// Returns Some(InGame) only when AppState is InGame
    /// Returns None otherwise (which removes the State<InGame> resource)
    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::InGame => Some(InGame),
            _ => None,
        }
    }
}

/// Resource for managing game statistics
/// HP: Player's health points (game over when 0)
/// current_step: Which ingredient step we're on (0-7 for 8 ingredients, 8 = none/completed)
#[derive(Resource, Debug, Clone)]
pub struct GameStats {
    pub hp: u8,
    pub max_hp: u8,
    pub current_step: usize,
    pub steps_completed: usize,
    /// Track which pans have oil
    pub kapaow_has_oil: bool,
    pub egg_has_oil: bool,
    pub kpaow_has_guage: bool,
    pub egg_has_guage: bool,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            hp: 10,     // Increased to 4 for 8 ingredients ; change to 10
            max_hp: 10, // Increased max HP ; change to 10
            current_step: 0,
            steps_completed: 0,
            kapaow_has_oil: false,
            egg_has_oil: false,
            kpaow_has_guage: false,
            egg_has_guage: false,
        }
    }
}

/// Event fired when the game is won
#[derive(Message, Debug, Clone)]
pub struct GameWinEvent;

/// Event fired when the game is lost
#[derive(Message, Debug, Clone)]
pub struct GameLoseEvent;

/// Event fired when a cooking step is completed successfully
#[derive(Message, Debug, Clone)]
pub struct StepCompletedEvent {
    pub step_index: usize,
}

/// Event fired when an ingredient is dropped onto the pan
/// Moved from ingredients module to game_state for better organization
#[derive(Message, Debug, Clone)]
pub struct IngredientDroppedEvent {
    pub ingredient_entity: Entity,
    pub ingredient_type: crate::entities::IngredientType,
    pub drop_position: Vec3,
    pub target_pan: Option<Entity>,
}
