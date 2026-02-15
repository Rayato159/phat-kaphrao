use bevy::prelude::*;

use crate::entities::ingredient::IngredientType;

#[derive(Component)]
pub struct CheckListPaper;

#[derive(Component)]
pub struct CheckListItem(pub IngredientType, pub f32);

/// Component to indicate that a checklist item has been marked
/// Contains the ingredient type to identify which item is marked
/// and the paper entity to track which checklist it belongs to
#[derive(Component)]
pub struct CheckListMark(pub IngredientType, pub Entity);

/// Spawn the checklist on the left side of the screen
/// Displays all ingredients in a single column layout
pub fn spawn_checklist(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    // // === Window size (world space) ===
    let w = window.width();

    let paper_image = asset_server.load("check_list/image/CheckListPaper.png");

    let image_w = 96.0 * 4.0;
    let total_h = 180.0 * 4.0;

    let num_items = 8;

    let left_margin = 86.0;

    let items = [
        IngredientType::Oil,
        IngredientType::Garlic,
        IngredientType::Chili,
        IngredientType::Pork,
        IngredientType::OysterSauce,
        IngredientType::MSG,
        IngredientType::Kaphrao,
        IngredientType::Egg,
    ];

    commands
        .spawn((
            CheckListPaper,
            Sprite {
                image: paper_image,
                color: Color::WHITE,
                ..default()
            },
            Transform::from_translation(Vec3::new(
                -(w * 0.5) + image_w * 0.5 + left_margin,
                -48.0,
                10.0,
            )),
        ))
        .with_children(|p| {
            for (i, ingradient_type) in items.iter().enumerate() {
                p.spawn((CheckListItem(
                    ingradient_type.clone(),
                    total_h * 0.5 - i as f32 * (total_h / num_items as f32) - 48.0,
                ),));
            }
        });
}
