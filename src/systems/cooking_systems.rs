//! Cooking Systems
//!
//! Contains all systems related to cooking game logic:
//! - handle_ingredient_drop: Validates ingredient drops and manages cooking sequence
//!
//! Uses Bevy 0.17+ patterns: Event-driven logic, Message system

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::entities::pan::PAN_SIZE;
use crate::entities::{IngredientType, PanEgg, PanKapaow};
use crate::message::{
    game_message::GameLoseMessage, gaug_message::GaugeSpawnMassage,
    ingredient_message::IngredientDroppedMessage,
};
use crate::resource::cooking_state::KaprowCookingState;
use crate::resource::game_state::GameState;
use crate::spawn::step_spawn::spawn_oil_step;

pub fn handle_kaprow_pan_ingredient_drop(
    mut commands: Commands,
    mut event_reader: MessageReader<IngredientDroppedMessage>,
    mut game_stats: ResMut<GameState>,
    mut gauge_spawn_events: MessageWriter<GaugeSpawnMassage>,
    mut game_over_events: MessageWriter<GameLoseMessage>,
    kaprow_cooking_state: Res<State<KaprowCookingState>>,
    mut next_kaprow_cooking_state: ResMut<NextState<KaprowCookingState>>,
    egg_cooking_state: Res<State<crate::resource::cooking_state::EggCookingState>>,
    mut next_egg_cooking_state: ResMut<NextState<crate::resource::cooking_state::EggCookingState>>,
    q_kaprow_pans: Query<&Transform, With<PanKapaow>>,
    q_egg_pans: Query<&Transform, With<PanEgg>>,
    q_pan_entities: Query<&PanKapaow>,
    q_egg_entities: Query<&PanEgg>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    for event in event_reader.read() {
        // Determine which pan the ingredient was dropped on
        let target_pan = event.target_pan;
        if target_pan.is_none() {
            continue;
        }
        let target_pan = target_pan.unwrap();

        let is_egg_pan = q_egg_entities.contains(target_pan);
        let is_kapoaw_pan = q_pan_entities.contains(target_pan);

        info!(
            "Ingredient {:?} dropped on pan - Egg: {}, Kapoaw: {}",
            event.ingredient_type, is_egg_pan, is_kapoaw_pan
        );

        // Handle Egg Pan
        if is_egg_pan {
            if !game_stats.egg_has_oil {
                if event.ingredient_type != IngredientType::Oil {
                    // Game over - first ingredient on egg pan is not oil!
                    game_over_events.write(GameLoseMessage);
                    return;
                }

                // Oil added to egg pan
                game_stats.egg_has_oil = true;

                for egg_pan_transform in q_egg_pans.iter() {
                    info!("Egg Pan Transform: {:?}", egg_pan_transform);

                    let image = asset_server.load("cooking_steps/image/OilOnPan.png");
                    let row = 3;
                    let col = 1;

                    let spritesheet = Spritesheet::new(&image, row, col);

                    let animation = spritesheet
                        .create_animation()
                        .add_row(0)
                        .set_duration(AnimationDuration::PerFrame(300))
                        .build();

                    let animation = animations.add(animation);

                    let sprite = spritesheet
                        .with_size_hint(PAN_SIZE as u32 * row as u32, PAN_SIZE as u32)
                        .sprite(&mut atlas_layouts);

                    let transform = Transform::from_translation(
                        egg_pan_transform.translation + Vec3::new(0.0, 0.0, 1.0),
                    );

                    commands.spawn(spawn_oil_step(sprite, animation, transform));

                    // Spawn timing gauge that follows the pan
                    gauge_spawn_events.write(GaugeSpawnMassage {
                        target_pan: Some(target_pan),
                    });

                    next_egg_cooking_state.set(egg_cooking_state.next_step());

                    info!(
                        "Timing gauge spawned to follow PanEgg at {:?}",
                        egg_pan_transform.translation
                    );
                }
            }
        }
        // Handle Kapoaw Pan
        else if is_kapoaw_pan {
            if !game_stats.kapaow_has_oil {
                if event.ingredient_type != IngredientType::Oil {
                    // Game over - first ingredient on kapoaw pan is not oil!
                    game_over_events.write(GameLoseMessage);
                    return;
                }

                // Oil added to kapoaw pan - spawn oil step and gauge
                game_stats.kapaow_has_oil = true;

                for kaprow_pan_transform in q_kaprow_pans.iter() {
                    info!("Kaprow Pan Transform: {:?}", kaprow_pan_transform);

                    // Spawn ingredient step
                    match *kaprow_cooking_state.get() {
                        KaprowCookingState::Oil => {
                            let image = asset_server.load("cooking_steps/image/OilOnPan.png");

                            let row = 3;
                            let col = 1;

                            let spritesheet = Spritesheet::new(&image, row, col);

                            let animation = spritesheet
                                .create_animation()
                                .add_row(0)
                                .set_duration(AnimationDuration::PerFrame(300))
                                .build();

                            let animation = animations.add(animation);

                            let sprite = spritesheet
                                .with_size_hint(PAN_SIZE as u32 * row as u32, PAN_SIZE as u32)
                                .sprite(&mut atlas_layouts);

                            let transform = Transform::from_translation(
                                kaprow_pan_transform.translation + Vec3::new(0.0, 0.0, 1.0),
                            );

                            commands.spawn(spawn_oil_step(sprite, animation, transform));
                        }
                        _ => {}
                    };

                    // Spawn timing gauge that follows the pan
                    gauge_spawn_events.write(GaugeSpawnMassage {
                        target_pan: Some(target_pan),
                    });

                    next_kaprow_cooking_state.set(kaprow_cooking_state.next_step());

                    info!(
                        "Timing gauge spawned to follow PanKapaow at {:?}",
                        kaprow_pan_transform.translation
                    );
                }
            }
        }
    }
}
