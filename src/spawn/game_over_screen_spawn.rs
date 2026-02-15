use bevy::{
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    text::{TextColor, TextFont},
    ui::{
        widget::Text, AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val, ZIndex,
    },
    utils::default,
};

use crate::entities::ui::GameEndScreen;

pub fn game_over_screen_parent_spawn() -> impl Bundle {
    (
        Name::new("GameOverScreen"),
        GameEndScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Semi-transparent overlay
        ZIndex(200),
    )
}

pub fn game_over_screen_title_spawn() -> impl Bundle {
    (
        Text::new("Game Over"),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.2, 0.2)),
    )
}

pub fn game_over_screen_subtitle_spawn() -> impl Bundle {
    (
        Text::new("The cooking went wrong..."),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    )
}

pub fn game_over_screen_instructions_spawn() -> impl Bundle {
    (
        Text::new("Press R to restart or ESC to quit"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
    )
}
