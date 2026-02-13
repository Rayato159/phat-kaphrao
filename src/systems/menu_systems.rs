//! Menu Systems
//!
//! Contains all systems related to the main menu:
//! - setup_main_menu: Setup the main menu with start and quit buttons
//! - spawn_menu_button: Spawn a menu button with the given text, color, and action
//! - cleanup_main_menu: Cleanup main menu when exiting the menu state
//! - handle_menu_button_click: Handle menu button clicks

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::entities::{ButtonAction, MainMenu};
use crate::resource::game_state::AppState;
use crate::spawn::main_menu_spawn::{
    button_child_main_menu_spawn, button_parent_main_menu_spawn, main_menu_parent_spawn,
    main_menu_subtitle_spawn, main_menu_title_spawn,
};

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
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::StartGame => {
                    info!("Start Game button clicked!");
                    next_state.set(AppState::InGame);
                }
                ButtonAction::QuitGame => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}
