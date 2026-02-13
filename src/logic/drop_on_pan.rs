use bevy::prelude::*;

use crate::{
    entities::{ingredient::DroppedIngredient, IngredientType},
    message::ingredient_message::IngredientDroppedMessage,
    resource::{cooking_state::KaprowCookingState, game_state::GameState},
};

/// Helper function to handle drop on pan egg area (x >= 0.0 && x <= 300.0)
pub fn handle_drop_on_pan_egg(
    event_writer: &mut MessageWriter<IngredientDroppedMessage>,
    ingredient_type: IngredientType,
    drop_position: Vec2,
    pan_entity: Entity,
) {
    info!("pan_entity {:?}", pan_entity);
    info!(
        "Dropped {:?} on pan egg at ({:.1}, {:.1})",
        ingredient_type, drop_position.x, drop_position.y
    );

    // Fire the ingredient dropped event
    event_writer.write(IngredientDroppedMessage {
        ingredient_type,
        target_pan: Some(pan_entity),
    });
}

/// Helper function to handle drop on pan kapoaw area (x >= -300.0 && x < 0.0)
pub fn handle_drop_on_pan_kapoaw(
    event_writer: &mut MessageWriter<IngredientDroppedMessage>,
    game_stats: &mut GameState,
    check_drop_ingredient_text: &mut Query<(&mut Text, &mut TextColor), With<DroppedIngredient>>,
    ingredient_type: IngredientType,
    drop_position: Vec2,
    pan_entity: Entity,
    kapow_state: &KaprowCookingState,
) -> bool {
    info!("kapow_state {:?}", kapow_state);
    info!(
        "compared {:?}",
        ingredient_type == kapow_state.clone().into()
    );
    let check_state_kapow_and_ingredient_drop = ingredient_type == kapow_state.clone().into();

    if !check_state_kapow_and_ingredient_drop {
        return false;
    }

    info!("pan_entity {:?}", pan_entity);
    game_stats.ingredient_dropped = true;

    for (mut text, mut color) in check_drop_ingredient_text.iter_mut() {
        *text = Text::new("Ingredient dropped!");
        *color = TextColor(Color::srgb(0.1, 1.0, 0.5));
    }

    info!(
        "Dropped {:?} on pan kapoaw at ({:.1}, {:.1})",
        ingredient_type, drop_position.x, drop_position.y
    );

    // Fire the ingredient dropped event
    event_writer.write(IngredientDroppedMessage {
        ingredient_type,
        target_pan: Some(pan_entity),
    });

    true
}
