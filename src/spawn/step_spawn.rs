use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::entities::{
    ingredient::DroppedIngredient, IngredientNext, IngredientType, StepIndicator,
};

pub fn step_parent_spawn() -> impl Bundle {
    (
        Name::new("StepIndicator"),
        StepIndicator,
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            row_gap: Val::Px(5.0),
            ..default()
        },
    )
}

pub fn step_child_spawn() -> impl Bundle {
    (
        Text::new("Next Ingredient:"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.2, 0.2)),
    )
}

pub fn step_child_current_spawn() -> impl Bundle {
    (
        Name::new("NextIngredient"),
        IngredientNext,
        Text::new("Oil"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.6, 0.2)),
    )
}

pub fn step_child_drop_spawn() -> impl Bundle {
    (
        Name::new("DropIngredient"),
        DroppedIngredient,
        Text::new("No"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.5)),
    )
}

pub fn spawn_oil_step(
    sprite: Sprite,
    animation: Handle<Animation>,
    transform: Transform,
) -> impl Bundle {
    (sprite, SpritesheetAnimation::new(animation), transform)
}
