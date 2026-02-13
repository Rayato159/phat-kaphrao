//! Check Lose Logic
//!
//! Contains game over logic:
//! - check_game_over: Checks if HP reaches 0 and triggers game over

use bevy::prelude::*;

use crate::message::game_message::GameLoseMessage;
use crate::resource::game_state::GameState;

/// Check if game is over (HP reaches 0) and trigger game over message
pub fn check_game_over(game_stats: Res<GameState>, mut game_over: MessageWriter<GameLoseMessage>) {
    if game_stats.hp == 0 {
        game_over.write(GameLoseMessage);
    }
}
