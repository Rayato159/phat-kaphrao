use bevy::{
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, UiRect, Val, ZIndex},
    utils::default,
};

use crate::entities::HUD;

pub fn setup_hud_spawn() -> impl Bundle {
    (
        Name::new("HUD"),
        HUD,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        ZIndex(100), // Ensure HUD is above everything
    )
}
