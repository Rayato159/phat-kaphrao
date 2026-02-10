use bevy::prelude::*;

use crate::entities::{
    Ingredient, IngredientBackground, IngredientForeground, IngredientForegroundLink,
    IngredientType, OriginalPosition,
};

/// Spawns the parent ingredient entity (for game logic)
/// This entity stays fixed in place and contains the background sprite as a child
pub fn ingredient_item_spawn(ingredient_type: &IngredientType, position: Vec3) -> impl Bundle {
    (
        Name::new(format!("Ingredient-{}", ingredient_type.name())),
        Ingredient {
            ingredient_type: *ingredient_type,
        },
        Transform::from_translation(position),
        OriginalPosition { position },
        // Children will be spawned separately
    )
}

/// Spawns the background sprite (Icon.png) as a child of the parent ingredient entity
/// The background stays fixed in its original position
pub fn ingredient_background_spawn(
    asset_server: &AssetServer,
) -> (Name, IngredientBackground, Sprite, Transform) {
    (
        Name::new("IngredientBackground"),
        IngredientBackground,
        Sprite {
            image: asset_server.load("ingradients/image/Icon.png"),
            custom_size: Some(Vec2::new(120.0, 120.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    )
}

/// Spawns the foreground sprite (specific ingredient image) as an independent entity
/// The foreground can be dragged freely while the background stays fixed
pub fn ingredient_foreground_spawn_independent(
    parent_entity: Entity,
    ingredient_type: &IngredientType,
    world_position: Vec3,
    asset_server: &AssetServer,
) -> impl Bundle {
    (
        Name::new("IngredientForeground"),
        IngredientForeground,
        IngredientForegroundLink { parent_entity },
        OriginalPosition {
            position: world_position,
        },
        Sprite {
            image: asset_server.load(ingredient_type.image_path()),
            custom_size: Some(Vec2::new(120.0, 120.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            world_position.x,
            world_position.y,
            world_position.z + 0.5,
        )),
        Pickable::default(), // Foreground sprite is draggable
    )
}
