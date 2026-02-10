//! UI Systems
//!
//! Contains all systems related to user interface elements:
//! - setup_hud: Spawns the HUD with 4 HP hearts and step indicator
//! - update_hud: Updates HUD based on current game state
//! - show_victory_screen: Shows victory screen when game is won
//! - show_game_over_screen: Shows game over screen when game is lost
//! - cleanup_game_end_screens: Cleans up game end screens
//! - handle_victory_screen_input: Handles keyboard input on victory screen
//! - handle_game_over_screen_input: Handles keyboard input on game over screen
//! - observe_game_state_changes: Observer for game state changes
//!
//! Uses Bevy 0.17+ patterns: Node-based UI, Observers, Computed States

use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::text::TextSpanAccess;

use crate::entities::{
    ButtonAction, GameEndScreen, HeartIcon, IngredientType, MainMenu, StepIndicator, HUD,
};
use crate::resource::game_state::{AppState, GameLoseEvent, GameStats, GameWinEvent};
use crate::spawn::game_over_screen_spawn::{
    game_over_screen_instructions_spawn, game_over_screen_parent_spawn,
    game_over_screen_subtitle_spawn, game_over_screen_title_spawn,
};
use crate::spawn::hp_group_spawn::{hp_container_child_spawn, hp_container_parent_spawn};
use crate::spawn::main_menu_spawn::{
    button_child_main_menu_spawn, button_parent_main_menu_spawn, main_menu_parent_spawn,
    main_menu_subtitle_spawn, main_menu_title_spawn,
};
use crate::spawn::setup_hud_spawn::setup_hud_spawn;
use crate::spawn::step_spawn::{step_child_current_spawn, step_child_spawn, step_parent_spawn};
use crate::spawn::victory_screen_spawn::{
    victory_screen_instructions_spawn, victory_screen_parent_spawn, victory_screen_subtitle_spawn,
    victory_screen_title_spawn,
};

/// Setup the HUD (Heads-Up Display)
/// Phase 4: Recipe & Progress
pub fn setup_hud(mut commands: Commands) {
    commands.spawn(setup_hud_spawn()).with_children(|parent| {
        spawn_hearts_container(parent);
        spawn_step_indicator(parent);
    });
    info!("HUD setup complete with 10 HP hearts");
}

/// Spawn the hearts container for HP display
fn spawn_hearts_container(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn(hp_container_parent_spawn())
        .with_children(|parent| {
            // Spawn 4 heart icons (max HP increased to 4 for 8 ingredients)
            for i in 0..4 {
                parent.spawn(hp_container_child_spawn(i));
            }
        });
}

/// Spawn the step indicator showing which ingredient to add next
fn spawn_step_indicator(builder: &mut ChildSpawnerCommands) {
    builder.spawn(step_parent_spawn()).with_children(|parent| {
        // "Next Ingredient:" label
        parent.spawn(step_child_spawn());
        parent.spawn(step_child_current_spawn());
    });
}

/// Update the HUD based on current game state
/// Phase 4: Recipe & Progress
pub fn update_hud(
    game_stats: Res<GameStats>,
    q_hearts: Query<&HeartIcon>,
    mut q_heart_text: Query<(Entity, &mut Text, &mut TextColor), With<HeartIcon>>,
    mut q_current_ingredient: Query<(&mut Text, &mut TextColor), (With<Name>, Without<HeartIcon>)>,
) {
    // Update HP hearts visibility and color
    for (heart_entity, mut text, mut text_color) in q_heart_text.iter_mut() {
        if let Ok(heart_icon) = q_hearts.get(heart_entity) {
            let is_alive = heart_icon.index < game_stats.hp;
            *text.write_span() = if is_alive {
                "❤️".to_string()
            } else {
                "🖤".to_string()
            };

            // Update color - red for alive, gray for dead
            if is_alive {
                *text_color = TextColor(Color::srgb(1.0, 0.3, 0.3));
            } else {
                *text_color = TextColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }

    // Update the step indicator (current ingredient)
    if let Ok((mut ingredient_text, mut ingredient_color)) = q_current_ingredient.single_mut() {
        // Get the next ingredient based on current_step
        // If current_step is 0-7, show the corresponding ingredient
        // If current_step is 8 or more, show "Done!"
        if let Some(ingredient_type) = IngredientType::from_step(game_stats.current_step) {
            *ingredient_text.write_span() = ingredient_type.name().to_string();
            *ingredient_color = TextColor(Color::srgb(0.8, 0.6, 0.2)); // Gold color
        } else {
            *ingredient_text.write_span() = "Done!".to_string();
            *ingredient_color = TextColor(Color::srgb(0.3, 0.8, 0.3)); // Green color
        }
    }
}

/// Show the victory screen when the player wins
/// Phase 5: Polish & Juice
pub fn show_victory_screen(mut commands: Commands) {
    info!("Showing victory screen!");

    commands
        .spawn(victory_screen_parent_spawn())
        .with_children(|parent| {
            // Victory title
            parent.spawn(victory_screen_title_spawn());

            // Subtitle
            parent.spawn(victory_screen_subtitle_spawn());

            // Instructions
            parent.spawn(victory_screen_instructions_spawn());
        });

    // Keyboard input is handled by a regular system
}

/// Show the game over screen when the player loses
/// Phase 5: Polish & Juice
pub fn show_game_over_screen(mut commands: Commands) {
    info!("Showing game over screen!");

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

/// Setup the main menu with start and quit buttons
/// Creates a centered menu with game title and action buttons
pub fn setup_main_menu(mut commands: Commands) {
    info!("Setting up main menu!");

    commands
        .spawn(main_menu_parent_spawn())
        .with_children(|parent| {
            // Game title
            parent.spawn(main_menu_title_spawn());

            // Subtitle
            parent.spawn(main_menu_subtitle_spawn());

            // Start Game button
            spawn_menu_button(
                parent,
                "Start Game",
                Color::srgb(0.3, 0.7, 0.3),
                ButtonAction::StartGame,
            );

            // Quit Game button
            spawn_menu_button(
                parent,
                "Quit Game",
                Color::srgb(0.8, 0.3, 0.3),
                ButtonAction::QuitGame,
            );
        });

    info!("Main menu setup complete");
}

/// Spawn a menu button with the given text, color, and action
fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    color: Color,
    action: ButtonAction,
) {
    parent
        .spawn(button_parent_main_menu_spawn(action, color))
        .with_children(|parent| {
            parent.spawn(button_child_main_menu_spawn(text));
        });
}

/// Cleanup main menu when exiting the menu state
pub fn cleanup_main_menu(mut commands: Commands, q_menu: Query<Entity, With<MainMenu>>) {
    for entity in q_menu.iter() {
        commands.entity(entity).despawn();
    }
    info!("Cleaned up main menu");
}

/// Handle menu button clicks
/// Transition to appropriate game state based on button action
pub fn handle_menu_button_click(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::StartGame => {
                    info!("Start Game button clicked!");
                    next_state.set(AppState::InGame);
                }
                ButtonAction::QuitGame => {
                    info!("Quit Game button clicked!");
                    // In a web build, we might not be able to quit
                    // But we can at least log it or handle differently
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // For desktop, we could close the window or exit
                        // For now, we'll just stay in the menu
                        warn!("Quit requested - functionality to be implemented");
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        info!("Quit not available in web browser");
                    }
                }
            }
        }
    }
}

/// Observer for game state changes to update UI
/// This ensures the UI stays in sync with the game state
pub fn observe_game_state_changes(
    _commands: Commands,
    _game_stats: ResMut<GameStats>,
    mut game_win_events: MessageReader<GameWinEvent>,
    mut game_lose_events: MessageReader<GameLoseEvent>,
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
