use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    entities::{
        ingredient::{DroppedIngredient, IngredientType},
        pan::{EggPanStepStateTag, EggStepSprite, PanEgg},
        ui::StepIndicatorEgg,
    },
    message::{
        game_message::{GameLoseMessage, GameWinMessage},
        gaug_message::{GaugeEggHitMassage, GaugeSpawnMassage},
        ingredient_message::IngredientDroppedMessage,
    },
    resource::{
        cooking_animations::EggCookingAnimations, cooking_state::EggCookingState,
        game_state::GameState,
    },
    spawn::{
        animation_spawn::insert_egg_cooking_animation, step_spawn::spawn_ingredient_animation,
    },
};

pub fn handle_egg_pan_ingredient_drop(
    mut commands: Commands,
    mut event_reader: MessageReader<IngredientDroppedMessage>,
    mut game_stats: ResMut<GameState>,
    mut gauge_spawn_events: MessageWriter<GaugeSpawnMassage>,
    mut game_over_events: MessageWriter<GameLoseMessage>,
    egg_cooking_state: Res<State<EggCookingState>>,
    q_egg_pans: Query<&Transform, With<PanEgg>>,
    q_egg_entities: Query<&PanEgg>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
    q_step_sprite: Query<&EggPanStepStateTag, With<EggStepSprite>>,
    mut egg_cooking_animation: ResMut<EggCookingAnimations>,
) {
    for event in event_reader.read() {
        let state = egg_cooking_state.get();

        let target_pan = event.target_pan;

        if target_pan.is_none() {
            continue;
        }

        let target_pan = target_pan.unwrap();

        let is_egg_pan = q_egg_entities.contains(target_pan);

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
                    let (image_path, row, col) = ("cooking_steps/image/Oil.png", 1, 3);

                    let image = asset_server.load(image_path);

                    let (sprite, spritesheet_animation) = spawn_ingredient_animation(
                        image,
                        row,
                        col,
                        500,
                        &mut atlas_layouts,
                        &mut animations,
                    );

                    let transform = Transform::from_translation(
                        egg_pan_transform.translation
                            + Vec3::new(0.0, 0.0, game_stats.count_tod_kai + 1.0),
                    );

                    commands.spawn((
                        EggPanStepStateTag(state.clone()),
                        EggStepSprite,
                        sprite,
                        spritesheet_animation,
                        transform,
                    ));

                    // Spawn timing gauge that follows the pan
                    gauge_spawn_events.write(GaugeSpawnMassage {
                        target_pan: Some(target_pan),
                    });
                }
            } else {
                for step_tag in q_step_sprite.iter() {
                    if step_tag.0 == *state {
                        continue;
                    }
                }

                let expected_ingredient = match *state {
                    EggCookingState::Egg => IngredientType::Egg,
                    _ => {
                        // Wrong ingredient or game over condition
                        game_over_events.write(GameLoseMessage);
                        return;
                    }
                };

                if event.ingredient_type != expected_ingredient {
                    game_stats.hp = game_stats.hp.saturating_sub(1);
                    return;
                }

                // Correct ingredient - spawn the step and gauge
                for egg_pan_transform in q_egg_pans.iter() {
                    let (image_path, row, col) = match *state {
                        EggCookingState::Egg => ("cooking_steps/image/Egg.png", 3, 3),
                        _ => continue,
                    };

                    let image = asset_server.load(image_path);
                    let duration = 500;

                    let (sprite, spritesheet_animation) = spawn_ingredient_animation(
                        image.clone(),
                        row,
                        col,
                        duration,
                        &mut atlas_layouts,
                        &mut animations,
                    );

                    let animations_cahce = insert_egg_cooking_animation(
                        state.clone(),
                        Spritesheet::new(&image, col, row),
                        duration,
                        &mut animations,
                    );

                    for animation in animations_cahce.iter() {
                        egg_cooking_animation.animations.insert(
                            (animation.0 .0.clone(), animation.0 .1),
                            animation.1.clone(),
                        );
                    }

                    let transform = Transform::from_translation(
                        egg_pan_transform.translation
                            + Vec3::new(0.0, 0.0, game_stats.count_tod_kai + 1.0),
                    );

                    commands.spawn((
                        EggPanStepStateTag(state.clone()),
                        EggStepSprite,
                        sprite,
                        spritesheet_animation,
                        transform,
                    ));

                    // Spawn timing gauge that follows the pan
                    gauge_spawn_events.write(GaugeSpawnMassage {
                        target_pan: Some(target_pan),
                    });
                }
            }
        }
    }
}

pub fn next_step_egg_cooking(
    mut game_stats: ResMut<GameState>,
    state: Res<State<EggCookingState>>,
    mut next_state: ResMut<NextState<EggCookingState>>,
    mut gauge_events: MessageReader<GaugeEggHitMassage>,
    mut game_win: MessageWriter<GameWinMessage>,
    mut query: Query<(&Name, &mut Text), (Without<DroppedIngredient>, With<StepIndicatorEgg>)>,
    mut check_drop_ingredient_text: Query<
        (&mut Text, &mut TextColor),
        (With<DroppedIngredient>, With<StepIndicatorEgg>),
    >,
    mut q_sprite_sheet_animation: Query<
        (&EggPanStepStateTag, &mut SpritesheetAnimation),
        With<EggStepSprite>,
    >,
    egg_cooking_animation: Res<EggCookingAnimations>,
) {
    for _ in gauge_events.read() {
        if !game_stats.ingredient_egg_dropped {
            return;
        }

        let state = state.get();

        for (step_tag, mut spritesheet_animation) in q_sprite_sheet_animation.iter_mut() {
            let state = state.clone();

            if state == EggCookingState::Oil {
                continue;
            }

            if step_tag.0 == state {
                if let Some(animation) = egg_cooking_animation.animations.get(&(state.clone(), 2)) {
                    spritesheet_animation.animation = animation.clone();
                }
            }
        }

        let next = state.next_step();
        next_state.set(next.clone());

        // Check if egg cooking is finished
        if matches!(next, EggCookingState::None) {
            game_stats.egg_is_finished = true;

            // Check if kapaow is not finished yet - egg wins!
            if game_stats.kapaow_is_finished && game_stats.egg_is_finished {
                game_win.write(GameWinMessage);
            }
        }

        // Update UI to show next ingredient
        for (name, mut text) in &mut query {
            if name.as_str() == "NextIngredient" {
                text.0 = next.to_string();
            }
        }

        for (mut text, mut color) in &mut check_drop_ingredient_text {
            *text = Text::new("No");
            *color = TextColor(Color::srgb(1.0, 0.3, 0.5));
        }

        game_stats.ingredient_egg_dropped = false;
    }
}
