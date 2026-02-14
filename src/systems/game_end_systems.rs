//! Game End Systems
//!
//! Contains all systems related to game end screens:
//! - show_victory_screen: Shows victory screen when game is won
//! - show_game_over_screen: Shows game over screen when game is lost
//! - cleanup_game_end_screens: Cleans up game end screens
//! - handle_victory_screen_input: Handles keyboard input on victory screen
//! - handle_game_over_screen_input: Handles keyboard input on game over screen

use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::entities::ui::GameEndScreen;
use crate::resource::game_state::AppState;
use crate::spawn::game_over_screen_spawn::{
    game_over_screen_instructions_spawn, game_over_screen_parent_spawn,
    game_over_screen_subtitle_spawn, game_over_screen_title_spawn,
};
use crate::spawn::victory_screen_spawn::{
    victory_screen_instructions_spawn, victory_screen_parent_spawn, victory_screen_title_spawn,
};

/// Show victory screen when the player wins
pub fn show_victory_screen(mut commands: Commands) {
    commands
        .spawn(victory_screen_parent_spawn())
        .with_children(|parent| {
            parent.spawn(victory_screen_title_spawn());
            parent.spawn(victory_screen_instructions_spawn());
        });

    // Keyboard input is handled by a regular system
}

/// Show the game over screen when the player loses
/// Phase 5: Polish & Juice
pub fn show_game_over_screen(mut commands: Commands) {
    commands
        .spawn(game_over_screen_parent_spawn())
        .with_children(|parent| {
            // Game Over title
            parent.spawn(game_over_screen_title_spawn());

            // Subtitle
            parent.spawn(game_over_screen_subtitle_spawn());

            // Instructions
            parent.spawn(game_over_screen_instructions_spawn());
        });

    // Keyboard input is handled by a regular system
}

/// Cleanup game end screens when returning to menu
pub fn cleanup_game_end_screens(
    mut commands: Commands,
    q_screens: Query<Entity, With<GameEndScreen>>,
) {
    for entity in q_screens.iter() {
        commands.entity(entity).despawn();
    }
    info!("Cleaned up game end screens");
}

/// Handle keyboard input on victory screen
/// Regular system that runs only when in Victory state
pub fn handle_victory_screen_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        info!("Restarting game from victory screen");
        next_state.set(AppState::InGame);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        info!("Quitting to menu from victory screen");
        next_state.set(AppState::Menu);
    }
}

/// Handle keyboard input on game over screen
/// Regular system that runs only when in GameOver state
pub fn handle_game_over_screen_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        info!("Restarting game from game over screen");
        next_state.set(AppState::InGame);
    } else if keyboard.just_pressed(KeyCode::Escape) {
        info!("Quitting to menu from game over screen");
        next_state.set(AppState::Menu);
    }
}
