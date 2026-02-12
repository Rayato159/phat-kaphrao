//! Cooking Systems
//!
//! Contains all systems related to cooking game logic:
//! - handle_ingredient_drop: Validates ingredient drops and manages cooking sequence
//!
//! Uses Bevy 0.17+ patterns: Event-driven logic, Message system

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::entities::pan::PAN_SIZE;
use crate::entities::{IngredientType, PanKapaow};
use crate::resource::game_state::{
    GameLoseEvent, GameStats, GameWinEvent, IngredientDroppedEvent, StepCompletedEvent,
};
use crate::spawn::step_spawn::{spawn_oil_step, KaprowCookingState};
use crate::GaugeSpawnEvent;

pub fn handle_kaprow_pan_ingredient_drop(
    mut commands: Commands,
    mut event_reader: MessageReader<IngredientDroppedEvent>,
    mut game_stats: ResMut<GameStats>,
    mut gauge_spawn_events: MessageWriter<GaugeSpawnEvent>,
    mut game_over_events: MessageWriter<GameLoseEvent>,
    kaprow_cooking_state: Res<State<KaprowCookingState>>,
    mut next_kaprow_cooking_state: ResMut<NextState<KaprowCookingState>>,
    q_kaprow_pans: Query<&Transform, With<PanKapaow>>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
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
                for kaprow_pan_transform in q_kaprow_pans.iter() {
                    info!("Kaprow Pan Transform: {:?}", kaprow_pan_transform);

                    // Spawn ingredient step
                    match *kaprow_cooking_state.get() {
                        KaprowCookingState::Oil => {
                            if !game_stats.kapaow_has_oil {
                                game_stats.kapaow_has_oil = true;

                                let image = asset_server.load("cooking_steps/image/OilOnPan.png");

                                let row = 3;
                                let col = 1;

                                let spritesheet = Spritesheet::new(&image, row, col);

                                let animation = spritesheet
                                    .create_animation()
                                    .add_row(0)
                                    .set_duration(AnimationDuration::PerFrame(300))
                                    // Get the final animation
                                    .build();

                                // Register the animation as an asset
                                let animation = animations.add(animation);

                                let sprite = spritesheet
                                    .with_size_hint(PAN_SIZE as u32 * row as u32, PAN_SIZE as u32)
                                    .sprite(&mut atlas_layouts);

                                let transform = Transform::from_translation(
                                    kaprow_pan_transform.translation + Vec3::new(0.0, 0.0, 1.0),
                                );

                                commands.spawn(spawn_oil_step(sprite, animation, transform));
                            }
                        }
                        _ => {}
                    };

                    // Spawn timing gauge that follows the pan
                    let gauge_position =
                        kaprow_pan_transform.translation + Vec3::new(0.0, 150.0, 10.0);

                    gauge_spawn_events.write(crate::entities::GaugeSpawnEvent {
                        position: gauge_position,
                        target_pan: Some(target_pan),
                    });

                    next_kaprow_cooking_state.set(kaprow_cooking_state.next_step());

                    info!(
                        "Timing gauge spawned to follow PanKapaow at {:?}",
                        gauge_position
                    );
                }
            }
        }
    }
}
