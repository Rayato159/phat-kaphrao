//! Observer Systems
//!
//! Contains observers for game state changes:
//! - observe_game_state_changes: Observer for game state changes

use bevy::prelude::*;

use crate::message::game_message::{GameLoseMessage, GameWinMessage};
use crate::resource::game_state::{AppState, GameState};

/// Observer for game state changes to update UI
/// This ensures the UI stays in sync with the game state
pub fn observe_game_state_changes(
    _commands: Commands,
    _game_stats: ResMut<GameState>,
    mut game_win_events: MessageReader<GameWinMessage>,
    mut game_lose_events: MessageReader<GameLoseMessage>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Handle victory event
    if !game_win_events.is_empty() {
        for _event in game_win_events.read() {
            info!("Victory event received!");
            next_state.set(AppState::Victory);
        }
    }

    // Handle game over event
    if !game_lose_events.is_empty() {
        for _event in game_lose_events.read() {
            info!("Game over event received!");
            next_state.set(AppState::GameOver);
        }
    }
}
