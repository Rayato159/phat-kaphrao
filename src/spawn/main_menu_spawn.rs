use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    prelude::*,
    text::{TextColor, TextFont},
    ui::{
        widget::{Button, Text},
        AlignItems, BackgroundColor, BorderRadius, FlexDirection, JustifyContent, Node, Val,
        ZIndex,
    },
    utils::default,
};

use crate::entities::ui::{ButtonAction, MainMenu};

pub fn main_menu_parent_spawn() -> impl Bundle {
    (
        Name::new("MainMenu"),
        MainMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(30.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.95, 0.92, 0.88)), // Same as background color
        ZIndex(150),
    )
}

pub fn main_menu_title_spawn() -> impl Bundle {
    (
        Text::new("Phat Kaphrao"),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 0.0, 0.0)),
    )
}

pub fn main_menu_subtitle_spawn() -> impl Bundle {
    (
        Text::new("A Thai Cooking Game"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 0.0, 0.0)),
    )
}

pub fn button_parent_main_menu_spawn(action: ButtonAction, color: Color) -> impl Bundle {
    (
        Name::new(format!("Button-{:?}", action)),
        Button,
        action,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::all(Val::Px(8.0)),
    )
}

pub fn button_child_main_menu_spawn(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
    )
}

pub fn completed_image_spawn(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("CompletedImage"),
        Node {
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        ImageNode::new(asset_server.load("ui/image/Completed.png")),
    )
}
