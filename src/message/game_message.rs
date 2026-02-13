use bevy::prelude::*;

/// Event fired when the game is won
#[derive(Message, Debug, Clone)]
pub struct GameWinMessage;

/// Event fired when the game is lost
#[derive(Message, Debug, Clone)]
pub struct GameLoseMessage;
