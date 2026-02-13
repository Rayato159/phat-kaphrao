//! UI Entities
//!
//! Contains all component definitions related to user interface elements:
//! - HUD: Marker component for the heads-up display container
//! - HeartIcon: Marker component for HP heart icons
//! - StepIndicator: Marker component for the current cooking step indicator
//! - GameEndScreen: Marker component for victory and game over screens
//! - HPOverlay: Marker component for HP overlay (gray/dark when HP == 0)

use bevy::prelude::*;

/// Marker component for the HUD (Heads-Up Display) container
/// This is the main UI element that displays game status at the top of the screen
#[derive(Component)]
pub struct HUD;

/// Marker component for HP heart icons
/// Each heart has an index to track which one it is (0, 1, 2, etc.)
#[derive(Component)]
pub struct HeartIcon {
    /// Index of this heart (0 = first heart, 1 = second heart, etc.)
    pub index: u8,
}

/// Marker component for the step indicator
/// This shows which ingredient should be added next in the cooking sequence
#[derive(Component)]
pub struct StepIndicator;

/// Marker component for the Kapaow pan step indicator
/// Shows the next ingredient for the Kapaow (Thai Basil Pork) cooking sequence
#[derive(Component)]
pub struct StepIndicatorKapaow;

/// Marker component for the Egg pan step indicator
/// Shows the next ingredient for the Egg cooking sequence
#[derive(Component)]
pub struct StepIndicatorEgg;

/// Marker component for game end screens
/// Used for both the victory screen and game over screen
#[derive(Component)]
pub struct GameEndScreen;

/// Marker component for HP overlay
/// Used for the gray/dark overlay when HP == 0
#[derive(Component)]
pub struct HPOverlay;

/// Marker component for HP text display
/// Used to display the current HP value as a number (e.g., "10/10")
#[derive(Component)]
pub struct HPText;

/// Marker component for the main menu
/// Used to identify the main menu UI elements
#[derive(Component)]
pub struct MainMenu;

/// Component for menu button actions
/// Defines what happens when a button is clicked
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonAction {
    StartGame,
    QuitGame,
}
