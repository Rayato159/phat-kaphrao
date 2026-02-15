use bevy::prelude::*;

use crate::entities::ingredient::IngredientType;

#[derive(Message, Debug, Clone)]
pub struct IngredientDroppedMessage {
    pub ingredient_type: IngredientType,
    pub target_pan: Option<Entity>,
}
