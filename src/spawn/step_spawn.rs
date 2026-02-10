use bevy::{
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    text::{TextColor, TextFont},
    ui::{widget::Text, AlignItems, FlexDirection, Node, Val},
    utils::default,
};

use crate::entities::StepIndicator;

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
        Name::new("CurrentIngredient"),
        Text::new("Oil"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.6, 0.2)),
    )
}
