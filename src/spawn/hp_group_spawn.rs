use bevy::{
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    text::{TextColor, TextFont},
    ui::{widget::Text, AlignItems, FlexDirection, Node, Val},
    utils::default,
};

use crate::entities::HeartIcon;

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
