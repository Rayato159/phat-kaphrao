pub mod cooking_state;
pub mod game_state;
pub mod heart;
pub mod time_state;

use crate::entities::ingredient::IngredientType;
use bevy::prelude::*;

/// Resource to track the last ingredient that was dropped
/// This is used to update the checklist status when the gauge is hit
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct LastDroppedIngredient {
    pub ingredient_type: Option<IngredientType>,
}

impl LastDroppedIngredient {
    pub fn set(&mut self, ingredient_type: IngredientType) {
        self.ingredient_type = Some(ingredient_type);
    }

    pub fn get(&self) -> Option<IngredientType> {
        self.ingredient_type
    }

    pub fn clear(&mut self) {
        self.ingredient_type = None;
    }
}
