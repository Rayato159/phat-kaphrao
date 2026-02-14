//! Checklist Entities
//!
//! Contains all component definitions related to the ingredient checklist:
//! - CheckListItem: Marker component for a checklist item container
//! - CheckListItemIcon: The ingredient icon/background
//! - CheckListSymbol: The symbol (X or ✓) showing drop status
//! - CheckListNumber: The number label (1#, 2#, etc.) for the checklist item
//! - CheckListCheckbox: The checkbox (☐/☑) showing drop status
//! - CheckListIngredientType: Stores which ingredient this checklist item represents
//! - CheckListStatus: The current status of the ingredient drop

use bevy::prelude::*;

use crate::entities::IngredientType;

/// Marker component for a checklist item container
/// Each checklist item represents one ingredient that needs to be dropped
#[derive(Component)]
pub struct CheckListItem;

/// Component storing which ingredient type this checklist item represents
#[derive(Component, Clone, Copy)]
pub struct CheckListIngredientType {
    /// The ingredient type for this checklist item
    pub ingredient_type: IngredientType,
}

/// The current status of an ingredient drop in the checklist
#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckListStatus {
    /// Ingredient has not been dropped yet
    #[default]
    NotDropped,
    /// Ingredient was dropped but gauge was not hit correctly
    DroppedIncorrect,
    /// Ingredient was dropped and gauge was hit correctly
    DroppedCorrect,
}

/// Marker component for the checklist icon (ingredient background)
/// This is the grayed-out ingredient icon
#[derive(Component)]
pub struct CheckListItemIcon;

/// Marker component for the checklist symbol
/// This displays X (red) or ✓ (green) based on drop status
#[derive(Component)]
pub struct CheckListSymbol;

/// Marker component for the checklist number label
/// This displays the item number (1#, 2#, etc.)
#[derive(Component)]
pub struct CheckListNumber;

/// Marker component for the checklist checkbox
/// This displays ☐ (unchecked) or ☑ (checked) based on drop status
#[derive(Component)]
pub struct CheckListCheckbox;
