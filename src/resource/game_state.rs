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
pub struct GameState {
    pub hp: u8,
    pub max_hp: u8,
    pub ingredient_kapaow_dropped: bool,
    pub ingredient_egg_dropped: bool,
    // pub current_step: usize,
    pub steps_completed: usize,
    pub kapaow_has_oil: bool,
    pub egg_has_oil: bool,
    pub gauge_container: bool,
    pub gauge_container_entity: Option<Entity>,
    pub gauge_container_width: f32,
    pub gauge_container_height: f32,
    pub kpaow_has_guage: bool,
    pub egg_has_guage: bool,
    pub target_width: f32,
    pub target_egg_x: Option<f32>,
    pub target_kapaow_x: Option<f32>,
    pub count_pud_kapoaw: f32,
    pub count_tod_kai: f32,
    pub kapaow_is_finished: bool,
    pub egg_is_finished: bool,
}

/// Resource for tracking cooking audio timer
/// Counts down from 3.0 to 0.0 to limit audio playback duration
#[derive(Resource, Default)]
pub struct CookingAudioTimer {
    pub timer: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            hp: 5,
            max_hp: 5,
            ingredient_kapaow_dropped: true,
            ingredient_egg_dropped: true,
            steps_completed: 0,
            kapaow_has_oil: false,
            egg_has_oil: false,
            gauge_container: false,
            gauge_container_entity: None,
            gauge_container_width: 600.0,
            gauge_container_height: 40.0,
            kpaow_has_guage: false,
            egg_has_guage: false,
            target_width: 0.10,
            target_egg_x: None,
            target_kapaow_x: None,
            count_pud_kapoaw: 0.0,
            count_tod_kai: 0.0,
            kapaow_is_finished: false,
            egg_is_finished: false,
        }
    }
}
