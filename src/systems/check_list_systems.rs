//! Checklist Systems
//!
//! Contains all systems related to the ingredient checklist:
//! - spawn_checklist: Spawns the checklist on the left side of the screen
//! - update_checklist_on_drop: Updates checklist status when ingredients are dropped
//! - update_checklist_on_gauge_hit: Updates checklist to checkmarks when gauge is hit correctly
//! - update_checklist_symbols: Updates symbol display (X or ✓) based on status

use bevy::prelude::*;

use crate::entities::{
    CheckListIngredientType, CheckListItem, CheckListItemIcon, CheckListStatus, CheckListSymbol,
    IngredientType,
};
use crate::message::gaug_message::{GaugeEggHitMassage, GaugeKapoawHitMassage};
use crate::message::ingredient_message::IngredientDroppedMessage;
use crate::resource::cooking_state::KaprowCookingState;
use crate::spawn::ingredient_spawn::ingredient_item_spawn;

/// Spawn the checklist on the left side of the screen
/// Displays all ingredients in a 2x4 grid layout
pub fn spawn_checklist(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    // === Window size (world space) ===
    let w = window.resolution.width();
    let h = window.resolution.height();

    // === Layout config ===
    let icon = 120.0; // INGREDIENT_SIZE
    let gap = 24.0;

    let cols: i32 = 2;
    let rows: i32 = 4;

    let spacing_x = icon + gap;
    let spacing_y = icon + gap;

    let grid_w = cols as f32 * icon + (cols as f32 - 1.0) * gap;
    let grid_h = rows as f32 * icon + (rows as f32 - 1.0) * gap;

    let left_margin = 16.0;
    let top_margin = 150.0; // Below the countdown timer

    // === Anchor: top-left of grid (on the left side of screen) ===
    let origin_x = -(w * 0.5) + left_margin;
    let origin_y = (h * 0.5) - top_margin;

    // === Ingredient order: row-major (top → bottom) ===
    // Sequence: Oil -> Garlic -> Chilli -> Pork -> OysterSauce -> MSG -> Basil -> Egg
    let ingredient_grid = [
        (IngredientType::Oil, 0, 0),
        (IngredientType::Garlic, 1, 0),
        (IngredientType::Chilli, 0, 1),
        (IngredientType::Pork, 1, 1),
        (IngredientType::OysterSauce, 0, 2),
        (IngredientType::MSG, 1, 2),
        (IngredientType::Basil, 0, 3),
        (IngredientType::Egg, 1, 3),
    ];

    for (ingredient_type, col, row) in ingredient_grid.iter() {
        let x = origin_x + *col as f32 * spacing_x;
        let y = origin_y - *row as f32 * spacing_y;

        let position = Vec3::new(x, y, 1.0);

        // Spawn checklist item container
        commands
            .spawn((
                Name::new(format!("CheckListItem-{}", ingredient_type.name())),
                CheckListItem,
                CheckListIngredientType {
                    ingredient_type: *ingredient_type,
                },
                CheckListStatus::NotDropped,
                ingredient_item_spawn(ingredient_type, position),
            ))
            .with_children(|parent| {
                // Spawn ingredient icon
                parent.spawn((
                    Name::new("CheckListItemIcon"),
                    CheckListItemIcon,
                    Sprite {
                        image: asset_server.load(ingredient_type.image_path()),
                        custom_size: Some(Vec2::new(120.0, 120.0)),
                        color: Color::WHITE,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                ));

                // Spawn symbol placeholder (will show X or ✓)
                parent.spawn((
                    Name::new("CheckListSymbol"),
                    CheckListSymbol,
                    Text::new(""),
                    TextFont {
                        font_size: 72.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                    Transform::from_translation(Vec3::new(40.0, 40.0, 0.2)),
                    GlobalTransform::default(),
                ));
            });
    }

    info!("Checklist spawned on the left side of the screen");
}

/// Update checklist status when ingredients are dropped
/// Sets status to DroppedIncorrect initially (will be updated to correct if gauge is hit)
pub fn update_checklist_on_drop(
    mut events: MessageReader<IngredientDroppedMessage>,
    mut checklist_query: Query<
        (&CheckListIngredientType, &mut CheckListStatus),
        With<CheckListItem>,
    >,
) {
    for event in events.read() {
        for (checklist_type, mut status) in checklist_query.iter_mut() {
            if checklist_type.ingredient_type == event.ingredient_type {
                // Only update if not already dropped correctly
                if *status != CheckListStatus::DroppedCorrect {
                    *status = CheckListStatus::DroppedIncorrect;
                    info!(
                        "Checklist updated for {:?}: DroppedIncorrect",
                        event.ingredient_type
                    );
                }
            }
        }
    }
}

/// Update checklist to checkmarks when gauge is hit correctly
pub fn update_checklist_on_gauge_hit(
    mut gauge_kapaow_events: MessageReader<GaugeKapoawHitMassage>,
    mut gauge_egg_events: MessageReader<GaugeEggHitMassage>,
    mut checklist_query: Query<
        (&CheckListIngredientType, &mut CheckListStatus),
        With<CheckListItem>,
    >,
    // Track which ingredient was last dropped
    cooking_state: Res<State<KaprowCookingState>>,
) {
    // Handle Kapaow gauge hits
    for _event in gauge_kapaow_events.read() {
        // Find the ingredient that was last dropped and update it to correct
        for (checklist_type, mut status) in checklist_query.iter_mut() {
            if *status == CheckListStatus::DroppedIncorrect {
                // Convert cooking state to ingredient type
                let ingredient_type = match *cooking_state.get() {
                    KaprowCookingState::Oil => IngredientType::Oil,
                    KaprowCookingState::Garlic => IngredientType::Garlic,
                    KaprowCookingState::Pork => IngredientType::Pork,
                    KaprowCookingState::OysterSauce => IngredientType::OysterSauce,
                    KaprowCookingState::Chilli => IngredientType::Chilli,
                    KaprowCookingState::MSG => IngredientType::MSG,
                    KaprowCookingState::Basil => IngredientType::Basil,
                    _ => continue,
                };

                if checklist_type.ingredient_type == ingredient_type {
                    *status = CheckListStatus::DroppedCorrect;
                    info!(
                        "Checklist updated for {:?}: DroppedCorrect",
                        ingredient_type
                    );
                }
            }
        }
    }

    // Handle Egg gauge hits
    for _event in gauge_egg_events.read() {
        for (checklist_type, mut status) in checklist_query.iter_mut() {
            if checklist_type.ingredient_type == IngredientType::Egg
                && *status == CheckListStatus::DroppedIncorrect
            {
                *status = CheckListStatus::DroppedCorrect;
                info!("Checklist updated for Egg: DroppedCorrect");
            }
        }
    }
}

/// Update checklist symbols and icon colors based on status
/// - NotDropped: White icon, empty symbol
/// - DroppedIncorrect: Gray icon, red X
/// - DroppedCorrect: Gray icon, green ✓
pub fn update_checklist_symbols(
    checklist_query: Query<(&CheckListStatus, Entity, &Children), With<CheckListItem>>,
    mut icon_query: Query<&mut Sprite, With<CheckListItemIcon>>,
    mut symbol_query: Query<&mut Text, With<CheckListSymbol>>,
) {
    for (status, parent_entity, children) in checklist_query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = icon_query.get_mut(child) {
                // Update icon color based on status
                match status {
                    CheckListStatus::NotDropped => {
                        sprite.color = Color::WHITE;
                    }
                    CheckListStatus::DroppedIncorrect | CheckListStatus::DroppedCorrect => {
                        sprite.color = Color::srgb(0.3, 0.3, 0.3); // Gray
                    }
                }
            } else if let Ok(mut text) = symbol_query.get_mut(child) {
                // Update symbol based on status
                match status {
                    CheckListStatus::NotDropped => {
                        text.0 = String::new();
                    }
                    CheckListStatus::DroppedIncorrect => {
                        text.0 = "✕".to_string();
                    }
                    CheckListStatus::DroppedCorrect => {
                        text.0 = "✓".to_string();
                    }
                }
            }
        }
    }
}
