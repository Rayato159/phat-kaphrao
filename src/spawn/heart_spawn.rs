use bevy::prelude::*;

use crate::{entities::heart::Heart, resource::heart::HeartAtlasUi};

#[derive(Component)]
pub struct HeartContainer;

pub fn hp_container_parent_spawn() -> impl Bundle {
    (
        HeartContainer,
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            column_gap: Val::Px(8.0),
            margin: UiRect {
                left: Val::Px(32.0),
                top: Val::Px(32.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::NONE),
    )
}

pub fn setup_hud_spawn() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),

            width: Val::Percent(100.0),
            height: Val::Px(120.0),

            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(16.0)),
            column_gap: Val::Px(8.0),
            ..default()
        },
        BackgroundColor(Color::NONE),
        ZIndex(100),
    )
}

pub fn spawn_heart_ui(atlas: &HeartAtlasUi, frame_index: usize, slot_index: u8) -> impl Bundle {
    (
        Node {
            width: Val::Px(120.0),
            height: Val::Px(120.0),
            ..default()
        },
        ImageNode::from_atlas_image(
            atlas.image.clone(),
            TextureAtlas {
                layout: atlas.layout.clone(),
                index: frame_index,
            },
        ),
        Heart {
            slot_index,
            frame_index,
        },
    )
}
