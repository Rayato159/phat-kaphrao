use bevy::prelude::*;

use crate::entities::check_list::{
    CheckListCheckbox, CheckListIngredientType, CheckListItem, CheckListItemIcon, CheckListStatus,
    CheckListSymbol,
};
use crate::entities::ingredient::IngredientType;
use crate::message::gaug_message::{GaugeEggHitMassage, GaugeKaprowHitMassage};
use crate::message::ingredient_message::IngredientDroppedMessage;

use crate::resource::LastDroppedIngredient;

/// Update checklist status when ingredients are dropped
/// Sets status to DroppedIncorrect initially (will be updated to correct if gauge is hit)
pub fn update_checklist_on_drop(
    mut events: MessageReader<IngredientDroppedMessage>,
    mut checklist_query: Query<
        (&CheckListIngredientType, &mut CheckListStatus),
        With<CheckListItem>,
    >,
    mut last_dropped: ResMut<LastDroppedIngredient>,
) {
    for event in events.read() {
        // Track the last dropped ingredient
        last_dropped.set(event.ingredient_type);
        info!(
            "Last dropped ingredient set to: {:?}",
            event.ingredient_type
        );

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
    mut gauge_kapaow_events: MessageReader<GaugeKaprowHitMassage>,
    mut gauge_egg_events: MessageReader<GaugeEggHitMassage>,
    mut checklist_query: Query<
        (&CheckListIngredientType, &mut CheckListStatus),
        With<CheckListItem>,
    >,
    // Track which ingredient was last dropped
    last_dropped: Res<LastDroppedIngredient>,
) {
    // Handle Kapaow gauge hits
    for _event in gauge_kapaow_events.read() {
        // Get the last dropped ingredient
        let ingredient_type = match last_dropped.get() {
            Some(ingredient) => ingredient,
            None => {
                info!("Gauge hit: No last dropped ingredient, skipping");
                continue;
            }
        };

        info!("Gauge hit for Kapaow: last_dropped = {:?}", ingredient_type);

        // Find the matching checklist item and update it
        for (checklist_type, mut status) in checklist_query.iter_mut() {
            if checklist_type.ingredient_type == ingredient_type {
                *status = CheckListStatus::DroppedCorrect;
                info!(
                    "Checklist updated for {:?}: DroppedCorrect",
                    ingredient_type
                );
                break; // Exit loop after finding the match
            }
        }
    }

    // Handle Egg gauge hits
    for _event in gauge_egg_events.read() {
        info!("Gauge hit for Egg: last_dropped = {:?}", last_dropped.get());

        // Find the Egg checklist item and update it
        for (checklist_type, mut status) in checklist_query.iter_mut() {
            if checklist_type.ingredient_type == IngredientType::Egg {
                *status = CheckListStatus::DroppedCorrect;
                info!("Checklist updated for Egg: DroppedCorrect");
                break; // Exit loop after finding the match
            }
        }
    }
}

/// Update checklist symbols, icon colors, and checkbox state based on status
/// - NotDropped: White icon, empty symbol, unchecked checkbox
/// - DroppedIncorrect: Gray icon, red X, unchecked checkbox
/// - DroppedCorrect: Gray icon, green ✓, checked checkbox
pub fn update_checklist_symbols(
    checklist_query: Query<(&CheckListStatus, &Children), With<CheckListItem>>,
    mut sprite_queries: ParamSet<(
        Query<&mut Sprite, With<CheckListItemIcon>>,
        Query<&mut Sprite, With<CheckListCheckbox>>,
        Query<&mut Sprite, With<CheckListSymbol>>,
    )>,
    asset_server: Res<AssetServer>,
) {
    for (status, children) in checklist_query.iter() {
        for child in children.iter() {
            if let Ok(mut sprite) = sprite_queries.p0().get_mut(child) {
                // Update icon color based on status
                match status {
                    CheckListStatus::NotDropped => {
                        sprite.color = Color::WHITE;
                    }
                    CheckListStatus::DroppedIncorrect | CheckListStatus::DroppedCorrect => {
                        sprite.color = Color::srgb(0.3, 0.3, 0.3); // Gray
                    }
                }
            }

            if let Ok(mut sprite) = sprite_queries.p1().get_mut(child) {
                // Update checkbox based on status
                match status {
                    CheckListStatus::NotDropped | CheckListStatus::DroppedIncorrect => {
                        sprite.image = asset_server.load("check_list/image/check_box.png");
                    }
                    CheckListStatus::DroppedCorrect => {
                        sprite.image = asset_server.load("check_list/image/check_box_correct.png");
                    }
                }
            }

            if let Ok(mut sprite) = sprite_queries.p2().get_mut(child) {
                // Update hashtag symbol based on status
                match status {
                    CheckListStatus::NotDropped => {
                        sprite.color = Color::srgba(1.0, 0.0, 0.0, 0.0); // Invisible
                    }
                    CheckListStatus::DroppedIncorrect => {
                        sprite.color = Color::srgba(1.0, 0.0, 0.0, 1.0); // Red, visible
                    }
                    CheckListStatus::DroppedCorrect => {
                        sprite.color = Color::srgba(0.0, 1.0, 0.0, 1.0); // Green, visible
                    }
                }
            }
        }
    }
}
