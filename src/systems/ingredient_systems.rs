//! Ingredient Systems
//!
//! Contains all systems related to ingredient mechanics:
//! - spawn_ingredients: Spawns all 8 ingredients in a 2x4 grid layout
//! - on_drag_start: Observer for when dragging starts on an ingredient
//! - on_drag_end: Observer for when dragging ends and detects drop on pan
//! - update_dragging_ingredient: Updates position of ingredients being dragged
//!
//! Uses Bevy 0.17+ patterns: Required Components, Picking API, and Observers

use bevy::prelude::*;

use crate::entities::ingredient::{
    Dragging, DroppedIngredient, HoverOriginalZ, Ingredient, IngredientForegroundLink,
    IngredientType, OriginalPosition, INGREDIENT_SIZE,
};
use crate::entities::pan::{PanEgg, PanKapaow};
use crate::entities::ui::{StepIndicatorEgg, StepIndicatorKapaow};
use crate::logic::drop_on_pan::{handle_drop_on_pan_egg, handle_drop_on_pan_kapoaw};
use crate::message::ingredient_message::IngredientDroppedMessage;
use crate::resource::cooking_state::{EggCookingState, KaprowCookingState};
use crate::resource::game_state::GameState;
use crate::spawn::ingredient_spawn::{
    ghost_ingredient_foreground_spawn, ingredient_background_spawn,
    ingredient_foreground_spawn_independent, ingredient_item_spawn,
};

/// Spawn all ingredients at their starting positions
/// Arranges 8 ingredients in a 2x4 grid, vertically centered on the right side
pub fn spawn_ingredients(
    mut commands: Commands,
    window: Single<&Window>,
    asset_server: Res<AssetServer>,
) {
    // === Window size (world space) ===
    let w = window.resolution.width();
    let h = window.resolution.height();

    // === Layout config ===
    let icon = INGREDIENT_SIZE;
    let gap = 24.0;

    let cols: i32 = 2;
    let rows: i32 = 4;

    let spacing_x = icon + gap;
    let spacing_y = icon + gap;

    let grid_w = cols as f32 * icon + (cols as f32 - 1.0) * gap;
    let grid_h = rows as f32 * icon + (rows as f32 - 1.0) * gap;

    let right_margin = 16.0;
    let bottom_margin = 16.0;

    // === Anchor: top-left of grid ===
    let origin_x = (w * 0.5) - right_margin - grid_w;
    let origin_y = -(h * 0.4) + bottom_margin + grid_h;

    // === Ingredient order: row-major (top → bottom) ===
    // Sequence: Oil -> Garlic -> Chilli -> Pork -> OysterSauce -> MSG -> Kaprow -> Egg
    let ingredient_grid = [
        (IngredientType::Oil, 0, 0),
        (IngredientType::Garlic, 1, 0),
        (IngredientType::Chilli, 0, 1),
        (IngredientType::Pork, 1, 1),
        (IngredientType::OysterSauce, 0, 2),
        (IngredientType::MSG, 1, 2),
        (IngredientType::Kaprow, 0, 3),
        (IngredientType::Egg, 1, 3),
    ];

    for (ingredient_type, col, row) in ingredient_grid.iter() {
        let x = origin_x + *col as f32 * spacing_x;
        let y = origin_y - *row as f32 * spacing_y;

        let position = Vec3::new(x, y, 1.0);

        // Parent (background)
        let parent_entity = commands
            .spawn(ingredient_item_spawn(ingredient_type, position))
            .with_children(|parent| {
                parent.spawn(ingredient_background_spawn(&asset_server));
            })
            .id();

        // Foreground (drag target)
        let foreground_entity = commands
            .spawn(ingredient_foreground_spawn_independent(
                parent_entity,
                ingredient_type,
                position,
                &asset_server,
            ))
            .id();

        commands.entity(foreground_entity).observe(on_drag_start);
        commands.entity(foreground_entity).observe(on_drag_end);
        commands.entity(foreground_entity).observe(on_hover_start);
        commands.entity(foreground_entity).observe(on_hover_end);

        info!(
            "Spawned ingredient {:?} at ({:.1}, {:.1})",
            ingredient_type, x, y
        );
    }

    info!("Ingredients spawned: 2x4 grid, centered vertically on right side");
}

/// Observer for when dragging starts on an ingredient foreground sprite
/// Uses Bevy 0.16+ Observer pattern with On<Pointer<DragStart>>
pub fn on_drag_start(
    trigger: On<Pointer<DragStart>>,
    mut commands: Commands,
    q_foreground_link: Query<&IngredientForegroundLink>,
    q_ingredients: Query<&Ingredient>,
    q_transform: Query<&Transform>,
    q_dragging: Query<(), With<Dragging>>,
    asset_server: Res<AssetServer>,
) {
    // ลากได้ทีละชิ้น
    if !q_dragging.is_empty() {
        return;
    }

    let source_fg = trigger.entity;
    let event = trigger.event();

    let Ok(fg_link) = q_foreground_link.get(source_fg) else {
        return;
    };

    let Ok(ingredient) = q_ingredients.get(fg_link.parent_entity) else {
        return;
    };

    let Ok(transform) = q_transform.get(source_fg) else {
        return;
    };

    // screen → world
    let pointer_position = Vec2::new(
        event.pointer_location.position.x,
        -event.pointer_location.position.y,
    );

    let world_position = transform.translation.truncate();
    let offset = world_position - pointer_position;

    let cloned_fg = commands
        .spawn(ghost_ingredient_foreground_spawn(
            &ingredient.ingredient_type,
            &world_position,
            offset,
            &asset_server,
            transform,
            fg_link,
        ))
        .id();

    info!(
        "Cloned & dragging ingredient: {:?} → {:?}",
        ingredient.ingredient_type, cloned_fg
    );
}

/// Observer for when dragging ends on an ingredient foreground sprite
/// Detects drop on pan and delegates to appropriate handler
pub fn on_drag_end(
    trigger: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut event_writer: MessageWriter<IngredientDroppedMessage>,
    mut game_stats: ResMut<GameState>,
    kapow_state: Res<State<KaprowCookingState>>,
    egg_state: Res<State<EggCookingState>>,
    q_foreground_link: Query<&IngredientForegroundLink>,
    q_ingredients: Query<&Ingredient>,
    q_original_position: Query<&OriginalPosition>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    q_dragging: Query<Entity, With<Dragging>>,
    q_pan_kapoaw: Query<(Entity, &Transform), With<PanKapaow>>,
    q_pan_egg: Query<(Entity, &Transform), With<PanEgg>>,
    mut check_drop_ingredient_text: ParamSet<(
        Query<(&mut Text, &mut TextColor), (With<DroppedIngredient>, With<StepIndicatorKapaow>)>,
        Query<(&mut Text, &mut TextColor), (With<DroppedIngredient>, With<StepIndicatorEgg>)>,
    )>,
) {
    // Despawn all dragging entities
    for e in q_dragging.iter() {
        commands.entity(e).despawn();
    }

    let entity = trigger.entity;
    let event = trigger.event();

    // Get the drop position from the pointer event
    let drop_position = event.pointer_location.position;

    // Get the foreground link to access parent entity
    if let Ok(foreground_link) = q_foreground_link.get(entity) {
        // Get ingredient info from parent entity
        if let Ok(ingredient) = q_ingredients.get(foreground_link.parent_entity) {
            // Get original position of foreground sprite
            if let Ok(original_pos) = q_original_position.get(entity) {
                let window = windows.single().unwrap();
                let cursor = window.cursor_position().unwrap();

                let (camera, camera_transform) = camera_q.single().unwrap();

                let position_drop = camera
                    .viewport_to_world_2d(camera_transform, cursor)
                    .unwrap();

                // Get pan entities
                let (pan_kapoaw_entity, _) = q_pan_kapoaw.single().unwrap();
                let (pan_egg_entity, _) = q_pan_egg.single().unwrap();

                info!("position_drop {:?}", position_drop);

                // Create separate queries for kapaow and egg step indicators
                // Use ParamSet to access queries for kapaow and egg step indicators

                let mut handled = false;

                // Check pan kapoaw first (left side: -300.0 to 0.0)
                if position_drop.x >= -300.0 && position_drop.x < 0.0 {
                    if handle_drop_on_pan_kapoaw(
                        &mut event_writer,
                        &mut game_stats,
                        &mut check_drop_ingredient_text.p0(),
                        ingredient.ingredient_type,
                        drop_position,
                        pan_kapoaw_entity,
                        kapow_state.get(),
                    ) {
                        handled = true;
                    }
                }
                // Check pan egg (right side: 0.0 to 300.0)
                else if position_drop.x >= 0.0 && position_drop.x <= 300.0 {
                    handle_drop_on_pan_egg(
                        &mut event_writer,
                        &mut game_stats,
                        &mut check_drop_ingredient_text.p1(),
                        ingredient.ingredient_type,
                        drop_position,
                        pan_egg_entity,
                        egg_state.get(),
                    );
                    handled = true;
                }

                // If not handled (not dropped on valid pan or wrong ingredient), reset to original position
                if !handled {
                    commands
                        .entity(entity)
                        .insert(Transform::from_translation(original_pos.position));
                    info!(
                        "Returned {:?} foreground to original position",
                        ingredient.ingredient_type
                    );
                }

                // Remove dragging component
                commands.entity(entity).remove::<Dragging>();
            }
        }
    }
}

/// Observer for when hover starts on an ingredient foreground sprite
/// Raises the z-index so the hovered ingredient appears on top
pub fn on_hover_start(
    trigger: On<Pointer<Over>>,
    mut commands: Commands,
    q_transform: Query<&Transform>,
) {
    let entity = trigger.entity;

    // Get current transform to store original z-index
    if let Ok(transform) = q_transform.get(entity) {
        // Store original z-index
        commands.entity(entity).insert(HoverOriginalZ {
            z: transform.translation.z,
        });

        // Set z-index to high value to appear on top
        commands.entity(entity).insert(Transform {
            translation: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                100.0, // High z-index for hover
            ),
            rotation: transform.rotation,
            scale: transform.scale,
        });
    }
}

/// Observer for when hover ends on an ingredient foreground sprite
/// Restores the original z-index
pub fn on_hover_end(
    trigger: On<Pointer<Out>>,
    mut commands: Commands,
    q_hover_z: Query<&HoverOriginalZ>,
    q_transform: Query<&Transform>,
) {
    let entity = trigger.entity;

    // Get original z-index and current transform
    if let Ok(hover_z) = q_hover_z.get(entity) {
        if let Ok(transform) = q_transform.get(entity) {
            // Restore original z-index
            commands.entity(entity).insert(Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, hover_z.z),
                rotation: transform.rotation,
                scale: transform.scale,
            });

            // Remove the hover z-index component
            commands.entity(entity).remove::<HoverOriginalZ>();
        }
    }
}

/// Update the position of ingredients being dragged
/// This system runs every frame to follow the pointer
/// Uses On<Pointer<Drag>> event from Bevy's Picking API
pub fn update_dragging_ingredient(
    mut q_dragging: Query<(&mut Transform, &Dragging)>,
    mut pointer_events: MessageReader<Pointer<Drag>>,
) {
    for event in pointer_events.read() {
        // Convert screen Y (increases down) to world Y (increases up) by negating
        let pointer_pos = Vec2::new(
            event.pointer_location.position.x,
            -event.pointer_location.position.y,
        );

        // Update the position of all dragging entities
        for (mut transform, dragging) in q_dragging.iter_mut() {
            transform.translation.x = pointer_pos.x + dragging.offset.x;
            transform.translation.y = pointer_pos.y + dragging.offset.y;
        }
    }
}
