use bevy::{
    ecs::{bundle::Bundle, name::Name},
    math::{Vec2, Vec3},
    picking::Pickable,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::entities::{Ingredient, IngredientType, OriginalPosition};

pub fn ingredient_item_spawn(ingredient_type: &IngredientType, position: Vec3) -> impl Bundle {
    (
        Name::new(format!("Ingredient-{}", ingredient_type.name())),
        Ingredient {
            ingredient_type: *ingredient_type,
        },
        Sprite {
            color: ingredient_type.color(),
            custom_size: Some(Vec2::new(120.0, 120.0)),
            ..default()
        },
        Transform::from_translation(position),
        OriginalPosition { position },
        // Make the ingredient interactive using Bevy's Picking API
        Pickable::default(),
    )
}
