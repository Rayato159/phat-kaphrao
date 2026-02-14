use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::entities::ingredient::IngredientType;

/// Marker component for a checklist item container
/// Each checklist item represents one ingredient that needs to be dropped
#[derive(Component)]
pub struct CheckListItem;

/// Component storing which ingredient type this checklist item represents
#[derive(Component, Clone, Copy)]
pub struct CheckListIngredientType {
    /// The ingredient type for this checklist item
    pub ingredient_type: IngredientType,
}

/// The current status of an ingredient drop in the checklist
#[derive(Component, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckListStatus {
    /// Ingredient has not been dropped yet
    #[default]
    NotDropped,
    /// Ingredient was dropped but gauge was not hit correctly
    DroppedIncorrect,
    /// Ingredient was dropped and gauge was hit correctly
    DroppedCorrect,
}

/// Marker component for the checklist icon (ingredient background)
/// This is the grayed-out ingredient icon
#[derive(Component)]
pub struct CheckListItemIcon;

/// Marker component for the checklist symbol
/// This displays X (red) or ✓ (green) based on drop status
#[derive(Component)]
pub struct CheckListSymbol;

/// Marker component for the checklist number label
/// This displays the item number (1#, 2#, etc.)
#[derive(Component)]
pub struct CheckListNumber;

/// Marker component for the checklist checkbox
/// This displays ☐ (unchecked) or ☑ (checked) based on drop status
#[derive(Component)]
pub struct CheckListCheckbox;

/// Bundle for spawning a number label (1-8)
pub fn check_list_number(asset_server: AssetServer, item_number: u32) -> impl Bundle {
    (
        CheckListNumber,
        Sprite {
            image: asset_server.load(format!("check_list/image/{}.png", item_number)),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_translation(Vec3::new(-40.0, 0.0, 101.0)),
    )
}

/// Bundle for spawning a checkbox (unchecked)
pub fn check_list_checkbox(asset_server: AssetServer) -> impl Bundle {
    (
        CheckListCheckbox,
        Sprite {
            image: asset_server.load("check_list/image/check_box.png"),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_translation(Vec3::new(40.0, 0.0, 101.0)),
    )
}

/// Bundle for spawning a hashtag symbol (status indicator)
pub fn check_list_symbol(asset_server: AssetServer) -> impl Bundle {
    (
        CheckListSymbol,
        Sprite {
            image: asset_server.load("check_list/image/hashtag.png"),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            color: Color::srgba(1.0, 0.0, 0.0, 0.0), // Start invisible
            ..default()
        },
        Transform::from_translation(Vec3::new(-70.0, 0.0, 102.0)),
    )
}

/// Spawn the checklist on the left side of the screen
/// Displays all ingredients in a single column layout
pub fn spawn_checklist(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // === Window size (world space) ===
    let window = window_query.single().unwrap();
    let w = window.resolution.width();

    // === Layout config ===
    let icon = 70.0; // INGREDIENT_SIZE
    let gap = 1.0;

    let rows: i32 = 8;

    let spacing_x = icon + gap;
    let spacing_y = icon + gap;

    let grid_h = rows as f32 * icon + (rows as f32 - 1.0) * gap;

    let left_margin = 200.0;

    // === Anchor: top-left of grid (on the left side of screen) ===
    let origin_x = -(w * 0.5) + left_margin;
    let origin_y = grid_h / 3.0; // Centered vertically

    // === Ingredient order: single column with 3 visual columns (Number | Icon | Checkbox) ===
    // Sequence: Oil -> Garlic -> Chilli -> Pork -> OysterSauce -> MSG -> Kaprow -> Egg
    let ingredient_grid = [
        (IngredientType::Oil, 0, 0),
        (IngredientType::Garlic, 0, 1),
        (IngredientType::Chilli, 0, 2),
        (IngredientType::Pork, 0, 3),
        (IngredientType::OysterSauce, 0, 4),
        (IngredientType::MSG, 0, 5),
        (IngredientType::Kaprow, 0, 6),
        (IngredientType::Egg, 0, 7),
    ];

    for (ingredient_type, col, row) in ingredient_grid.iter() {
        let x = origin_x + *col as f32 * spacing_x;
        let y = origin_y - *row as f32 * spacing_y;

        let position = Vec3::new(x, y, 1.0);

        let item_number = *row + 1;

        // Spawn checklist item container
        commands
            .spawn((
                Name::new(format!("CheckListItem-{}", ingredient_type.name())),
                CheckListItem,
                CheckListIngredientType {
                    ingredient_type: *ingredient_type,
                },
                CheckListStatus::NotDropped,
                Transform::from_translation(position),
            ))
            .with_children(|parent| {
                // Spawn number label (column 1) - on the left
                parent.spawn(check_list_number(asset_server.clone(), item_number));

                // Spawn ingredient icon (column 2) - in the center
                parent.spawn((
                    Name::new("CheckListItemIcon"),
                    CheckListItemIcon,
                    Sprite {
                        image: asset_server.load(ingredient_type.image_path()),
                        custom_size: Some(Vec2::new(60.0, 60.0)),
                        color: Color::WHITE,
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
                ));

                // Spawn checkbox (column 3) - on the right
                parent.spawn(check_list_checkbox(asset_server.clone()));

                // Spawn hashtag symbol (status indicator) - on the far left
                parent.spawn(check_list_symbol(asset_server.clone()));
            });
    }

    info!("Checklist spawned on the left side of the screen");
}
