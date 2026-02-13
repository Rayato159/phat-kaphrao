use bevy::{
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    text::{TextColor, TextFont},
    ui::{widget::Text, AlignItems, FlexDirection, Node, Val},
    utils::default,
};

use crate::entities::{HPText, HeartIcon};

pub fn hp_container_parent_spawn() -> impl Bundle {
    (Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(10.0),
        align_items: AlignItems::Center,
        ..default()
    },)
}

pub fn hp_container_child_spawn(i: u8) -> impl Bundle {
    (
        Name::new(format!("Heart-{}", i)),
        HeartIcon { index: i },
        Text::new("❤️"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
    )
}

/// Spawn HP text display (e.g., "10/10")
pub fn hp_text_spawn(current_hp: u8, max_hp: u8) -> impl Bundle {
    (
        Name::new("HP-Text"),
        HPText,
        Text::new(format!("{}/{}", current_hp, max_hp)),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    )
}
