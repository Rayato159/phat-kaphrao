//! Cooking Systems
//!
//! Contains all systems related to cooking game logic:
//! - handle_ingredient_drop: Validates ingredient drops and manages cooking sequence
//!
//! Uses Bevy 0.17+ patterns: Event-driven logic, Message system

use bevy::prelude::*;

use crate::entities::{IngredientType, PanEgg, PanKapaow};
use crate::resource::game_state::{
    GameLoseEvent, GameStats, GameWinEvent, IngredientDroppedEvent, StepCompletedEvent,
};

/// Handle ingredient drops on the pan
/// Phase 2: Ingredient Mechanics & Phase 3: Cooking Mini-game
/// Validates the cooking sequence and manages game state
///
/// This system:
/// - Checks if the first and second ingredients are oil (game over if not)
/// - Triggers the timing gauge mini-game when oil is added to a pan
/// - Tracks which pans have oil
/// - Increments the step counter for correct ingredients
/// - Deducts HP for wrong ingredients (after first two steps)
/// - Detects game over and victory conditions
pub fn handle_ingredient_drop(
    mut commands: Commands,
    mut event_reader: MessageReader<IngredientDroppedEvent>,
    mut game_stats: ResMut<GameStats>,
    mut gauge_spawn_events: MessageWriter<crate::entities::GaugeSpawnEvent>,
    mut step_completed_events: MessageWriter<StepCompletedEvent>,
    mut game_over_events: MessageWriter<GameLoseEvent>,
    mut win_events: MessageWriter<GameWinEvent>,
    q_kapaow_pans: Query<&Transform, With<crate::entities::PanKapaow>>,
    q_egg_pans: Query<&Transform, With<crate::entities::PanEgg>>,
) {
    for event in event_reader.read() {
        let expected_ingredient = IngredientType::from_step(game_stats.current_step);

        info!(
            "Ingredient dropped: {:?}, Expected: {:?}, Step: {}/8",
            event.ingredient_type, expected_ingredient, game_stats.current_step
        );

        // Special rule: First and second ingredients MUST be oil
        if game_stats.current_step < 2 {
            if event.ingredient_type != IngredientType::Oil {
                // Game over - first or second ingredient is not oil!
                error!("Game Over - Step {} must be Oil!", game_stats.current_step);
                game_over_events.write(GameLoseEvent);
                return;
            }

            // Oil added to a pan - track which pan and spawn gauge
            if let Some(target_pan) = event.target_pan {
                // Determine which pan was targeted and spawn gauge
                if let Ok(pan_transform) = q_kapaow_pans.get(target_pan) {
                    // Track oil in Kapaow pan
                    if !game_stats.kapaow_has_oil {
                        game_stats.kapaow_has_oil = true;
                        info!("Oil added to PanKapaow");
                    }

                    // Spawn timing gauge that follows the pan
                    let gauge_position = pan_transform.translation + Vec3::new(0.0, 150.0, 10.0);
                    gauge_spawn_events.write(crate::entities::GaugeSpawnEvent {
                        position: gauge_position,
                        target_pan: Some(target_pan),
                    });
                    info!(
                        "Timing gauge spawned to follow PanKapaow at {:?}",
                        gauge_position
                    );
                } else if let Ok(pan_transform) = q_egg_pans.get(target_pan) {
                    // Track oil in Egg pan
                    if !game_stats.egg_has_oil {
                        game_stats.egg_has_oil = true;
                        info!("Oil added to PanEgg");
                    }

                    // Spawn timing gauge that follows the pan
                    let gauge_position = pan_transform.translation + Vec3::new(0.0, 150.0, 10.0);
                    gauge_spawn_events.write(crate::entities::GaugeSpawnEvent {
                        position: gauge_position,
                        target_pan: Some(target_pan),
                    });
                    info!(
                        "Timing gauge spawned to follow PanEgg at {:?}",
                        gauge_position
                    );
                }
            }

            // Increment step and fire completion event
            game_stats.current_step += 1;
            game_stats.steps_completed += 1;

            step_completed_events.write(StepCompletedEvent {
                step_index: event.ingredient_type as usize,
            });

            // Check if all 8 steps are completed (Victory condition)
            if game_stats.current_step >= 8 {
                info!("All 8 ingredients added - Victory!");
                win_events.write(GameWinEvent);
            }
        } else {
            // Normal gameplay after oil phase
            // Check if this is the correct ingredient for the current step
            if Some(event.ingredient_type) == expected_ingredient {
                // Correct ingredient!

                // Increment step and fire completion event
                game_stats.current_step += 1;
                game_stats.steps_completed += 1;

                step_completed_events.write(StepCompletedEvent {
                    step_index: event.ingredient_type as usize,
                });

                // Check if all 8 steps are completed (Victory condition)
                if game_stats.current_step >= 8 {
                    info!("All 8 ingredients added - Victory!");
                    win_events.write(GameWinEvent);
                }
            } else {
                // Wrong ingredient!
                warn!(
                    "Wrong ingredient! Expected {:?}, got {:?}",
                    expected_ingredient, event.ingredient_type
                );

                // Deduct HP
                if game_stats.hp > 0 {
                    game_stats.hp -= 1;
                    warn!("HP reduced to {}/{}", game_stats.hp, game_stats.max_hp);
                }

                // Check for game over
                if game_stats.hp == 0 {
                    error!("Game Over - HP reached 0");
                    game_over_events.write(GameLoseEvent);
                }
            }
        }
    }
}
