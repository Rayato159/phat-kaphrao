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

use crate::entities::{
    Dragging, Ingredient, IngredientType, OriginalPosition, Pan, PanEgg, PanKapaow,
};
use crate::resource::game_state::{GameStats, IngredientDroppedEvent};
use crate::spawn::ingredient_spawn::ingredient_item_spawn;

/// Spawn all ingredients at their starting positions
/// Arranges 8 ingredients in a 2x4 grid for better screen space usage
///
/// Top row: Oil, Garlic, Pork, Egg
/// Bottom row: Fish Sauce, Oyster Sauce, Thai Chilli, Holy Basil Leaves
pub fn spawn_ingredients(mut commands: Commands, mut game_stats: ResMut<GameStats>) {
    // Grid layout parameters
    let start_x = 400.0; // Right side - positioned for 1920x1080 window
    let spacing_x = 300.0; // Horizontal spacing
    let spacing_y = 100.0; // Vertical spacing
    let start_y = -100.0; // Top row y position

    // Define all 8 ingredients with their grid positions (2 columns x 4 rows)
    let ingredient_data = [
        (IngredientType::Oil, 1, start_y),    // Row 0, right column
        (IngredientType::Garlic, 0, start_y), // Row 0, left column
        (IngredientType::Pork, 1, start_y - spacing_y), // Row 1, right column
        (IngredientType::Egg, 0, start_y - spacing_y), // Row 1, left column
        (IngredientType::FishSauce, 1, start_y - spacing_y * 2.0), // Row 2, right column
        (IngredientType::OysterSauce, 0, start_y - spacing_y * 2.0), // Row 2, left column
        (IngredientType::ThaiChilli, 1, start_y - spacing_y * 3.0), // Row 3, right column
        (
            IngredientType::HolyBasilLeaves,
            0,
            start_y - spacing_y * 3.0,
        ), // Row 3, left column
    ];

    for (ingredient_type, col_index, y_pos) in ingredient_data.iter() {
        let x_pos = start_x + (*col_index as f32 * spacing_x);
        let position = Vec3::new(x_pos, *y_pos, 1.0);

        // Spawn ingredient entity with all required components
        commands
            .spawn(ingredient_item_spawn(ingredient_type, position))
            // Use Observer for drag start event with Bevy 0.16+ syntax
            .observe(on_drag_start)
            // Use Observer for drag end event with Bevy 0.16+ syntax
            .observe(on_drag_end);

        info!(
            "Spawned ingredient: {:?} at ({:.1}, {:.1})",
            ingredient_type, x_pos, y_pos
        );
    }

    // Set initial step to 0 (Oil)
    game_stats.current_step = 0;

    info!("All 8 ingredients spawned in 4x2 grid layout on right side (optimized for 1920x1080)");
}

/// Observer for when dragging starts on an ingredient
/// Uses Bevy 0.16+ Observer pattern with On<Pointer<DragStart>>
pub fn on_drag_start(
    trigger: On<Pointer<DragStart>>,
    mut commands: Commands,
    q_ingredients: Query<&Ingredient>,
    mut q_transform: Query<&mut Transform>,
    q_dragging: Query<(), With<Dragging>>,
) {
    // Only allow one ingredient to be dragged at a time
    if !q_dragging.is_empty() {
        return;
    }

    let entity = trigger.entity;
    let event = trigger.event();

    // Get the ingredient and transform
    if let Ok(ingredient) = q_ingredients.get(entity) {
        if let Ok(mut transform) = q_transform.get_mut(entity) {
            // Calculate offset for smoother dragging
            // Convert screen Y (increases down) to world Y (increases up) by negating
            let pointer_pos = Vec2::new(
                event.pointer_location.position.x,
                -event.pointer_location.position.y,
            );
            let ingredient_pos = transform.translation.truncate();

            commands.entity(entity).insert(Dragging {
                offset: ingredient_pos - pointer_pos,
            });

            // Raise the z-index so the dragged ingredient appears on top
            transform.translation.z = 10.0;

            info!("Started dragging: {:?}", ingredient.ingredient_type);
        }
    }
}

/// Observer for when dragging ends on an ingredient
/// This is where we detect if the ingredient was dropped on the pan
/// Uses Bevy 0.16+ Observer pattern with On<Pointer<DragEnd>>
pub fn on_drag_end(
    trigger: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut event_writer: MessageWriter<IngredientDroppedEvent>,
    q_ingredients: Query<(&Ingredient, &OriginalPosition)>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut transform_queries: ParamSet<(
        Query<&mut Transform, Without<Pan>>,
        Query<(Entity, &Transform), With<PanKapaow>>,
        Query<(Entity, &Transform), With<PanEgg>>,
    )>,
) {
    let entity = trigger.entity;
    let event = trigger.event();

    // Get the drop position from the pointer event
    let drop_position = event.pointer_location.position;
    let drop_world_pos = Vec3::new(drop_position.x, drop_position.y, 1.0);

    if let Ok((ingredient, original_pos)) = q_ingredients.get(entity) {
        // Collect pan positions first (using immutable queries)
        let mut pans: Vec<(Entity, Vec2)> = Vec::new();
        let window = windows.single().unwrap();
        let cursor = window.cursor_position().unwrap();

        let (camera, camera_transform) = camera_q.single().unwrap();

        let position_drop = camera
            .viewport_to_world_2d(camera_transform, cursor)
            .unwrap();
        for (pan_entity, pan_transform) in transform_queries.p1().iter() {
            pans.push((pan_entity, pan_transform.translation.truncate()));
        }

        for (pan_entity, pan_transform) in transform_queries.p2().iter() {
            pans.push((pan_entity, pan_transform.translation.truncate()));
        }
        info!("Pan positions collected {:?}", pans);

        // Check if the ingredient was dropped on any pan
        let mut dropped_on_pan = false;
        let mut target_pan: Option<Entity> = None;

        // Check if dropped on any pan
        for (pan_entity, pan_center) in pans.iter() {
            // let drop_point = drop_position;

            // If dropped within 150 pixels of pan center, consider it dropped on the pan
            // let distance = pan_center.distance(drop_point); // <- can't use this because isn't codinate position
            // info!("distance {:?}", distance <= 150.0);
            // info!("distance {:?}", distance);
            info!("pan_center {:?}", pan_center);
            info!("position_drop {:?}", position_drop);
            // info!("drop_point {:?}", drop_point);

            // if distance <= 150.0 {
            if position_drop.x <= 150.0 {
                dropped_on_pan = true;
                target_pan = Some(*pan_entity);
                info!(
                    "Dropped {:?} on pan at ({:.1}, {:.1})",
                    ingredient.ingredient_type, drop_position.x, drop_position.y
                );
                break;
            }
        }

        // Fire the ingredient dropped event if dropped on a pan
        if dropped_on_pan {
            event_writer.write(IngredientDroppedEvent {
                ingredient_entity: entity,
                ingredient_type: ingredient.ingredient_type,
                drop_position: drop_world_pos,
                target_pan,
            });
        }

        // Remove dragging component
        commands.entity(entity).remove::<Dragging>();

        // Reset z-index and return to origin if not dropped on pan
        if let Ok(mut transform) = transform_queries.p0().get_mut(entity) {
            if !dropped_on_pan {
                // Animate back to original position (in a real game, you'd use a tween system)
                // For now, we'll just snap back
                transform.translation = original_pos.position;
                info!(
                    "Returned {:?} to original position",
                    ingredient.ingredient_type
                );
            }
            // Reset z-index
            transform.translation.z = 1.0;
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
