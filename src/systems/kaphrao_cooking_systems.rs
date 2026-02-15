use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    entities::{
        ingredient::{DroppedIngredient, IngredientType},
        pan::{KaphraoPanStepStateTag, KaphraoStepSprite, PanKaphrao},
        ui::StepIndicatorKaphrao,
    },
    message::{
        game_message::{GameLoseMessage, GameWinMessage},
        gaug_message::{GaugeKaphraoHitMassage, GaugeSpawnMassage},
        ingredient_message::IngredientDroppedMessage,
    },
    resource::{
        cooking_animations::KaphraoCookingAnimations, cooking_state::KaphraoCookingState,
        game_state::GameState,
    },
    spawn::{
        animation_spawn::insert_kaphrao_cooking_animation, step_spawn::spawn_ingredient_animation,
    },
};

pub fn handle_kaphrao_pan_ingredient_drop(
    mut commands: Commands,
    mut event_reader: MessageReader<IngredientDroppedMessage>,
    mut game_stats: ResMut<GameState>,
    mut gauge_spawn_events: MessageWriter<GaugeSpawnMassage>,
    mut game_over_events: MessageWriter<GameLoseMessage>,
    kaphrao_cooking_state: Res<State<KaphraoCookingState>>,
    q_kaphrao_pans: Query<&Transform, With<PanKaphrao>>,
    q_pan_entities: Query<&PanKaphrao>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
    q_step_sprite: Query<(&KaphraoPanStepStateTag, Entity), With<KaphraoStepSprite>>,
    mut kaphrao_cooking_animation: ResMut<KaphraoCookingAnimations>,
) {
    for event in event_reader.read() {
        let state = kaphrao_cooking_state.get();
        // Determine which pan the ingredient was dropped on
        let target_pan = event.target_pan;

        if target_pan.is_none() {
            continue;
        }

        let target_pan = target_pan.unwrap();

        let is_kaphrao_pan = q_pan_entities.contains(target_pan);

        // Handle Egg Pan
        if is_kaphrao_pan {
            if !game_stats.kaphrao_has_oil {
                if event.ingredient_type != IngredientType::Oil {
                    // Game over - first ingredient on kaphrao pan is not oil!
                    game_over_events.write(GameLoseMessage);
                    return;
                }

                // Oil added to kaphrao pan - spawn oil step and gauge
                game_stats.kaphrao_has_oil = true;

                for kaphrao_pan_transform in q_kaphrao_pans.iter() {
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
                        kaphrao_pan_transform.translation
                            + Vec3::new(0.0, 0.0, game_stats.count_phat_kaphrao + 1.0),
                    );

                    commands.spawn((
                        KaphraoPanStepStateTag(state.clone()),
                        KaphraoStepSprite,
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
                for (step_tag, _) in q_step_sprite.iter() {
                    if step_tag.0 == *state {
                        continue;
                    }
                }

                let expected_ingredient = match *kaphrao_cooking_state.get() {
                    KaphraoCookingState::Garlic => IngredientType::Garlic,
                    KaphraoCookingState::Chili => IngredientType::Chili,
                    KaphraoCookingState::Pork => IngredientType::Pork,
                    KaphraoCookingState::OysterSauce => IngredientType::OysterSauce,
                    KaphraoCookingState::MSG => IngredientType::MSG,
                    KaphraoCookingState::Kaphrao => IngredientType::Kaphrao,
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
                for kaphrao_pan_transform in q_kaphrao_pans.iter() {
                    let (image_path, row, col) = match *kaphrao_cooking_state.get() {
                        KaphraoCookingState::Garlic => ("cooking_steps/image/Garlic.png", 3, 3),
                        KaphraoCookingState::Chili => ("cooking_steps/image/Chili.png", 3, 3),
                        KaphraoCookingState::Pork => ("cooking_steps/image/Pork.png", 3, 3),
                        KaphraoCookingState::OysterSauce => {
                            for (step_tag, entity) in q_step_sprite.iter() {
                                if step_tag.0 == KaphraoCookingState::Pork {
                                    commands.entity(entity).despawn();
                                }
                            }
                            ("cooking_steps/image/PorkWithOysterSauce.png", 3, 3)
                        }
                        KaphraoCookingState::MSG => {
                            for (step_tag, entity) in q_step_sprite.iter() {
                                if step_tag.0 == KaphraoCookingState::OysterSauce {
                                    commands.entity(entity).despawn();
                                }
                            }
                            ("cooking_steps/image/PorkWithMSG.png", 3, 3)
                        }
                        KaphraoCookingState::Kaphrao => ("cooking_steps/image/Kaphrao.png", 3, 3),
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

                    let animations_cahce = insert_kaphrao_cooking_animation(
                        state.clone(),
                        Spritesheet::new(&image, col, row),
                        duration,
                        &mut animations,
                    );

                    for animation in animations_cahce.iter() {
                        kaphrao_cooking_animation
                            .animations
                            .insert((animation.0.0.clone(), animation.0.1), animation.1.clone());
                    }

                    let transform = Transform::from_translation(
                        kaphrao_pan_transform.translation
                            + Vec3::new(0.0, 0.0, game_stats.count_phat_kaphrao + 1.0),
                    );

                    commands.spawn((
                        KaphraoPanStepStateTag(state.clone()),
                        KaphraoStepSprite,
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

pub fn next_step_kaphrao_cooking(
    mut game_stats: ResMut<GameState>,
    state: Res<State<KaphraoCookingState>>,
    mut next_state: ResMut<NextState<KaphraoCookingState>>,
    mut gauge_events: MessageReader<GaugeKaphraoHitMassage>,
    mut game_win: MessageWriter<GameWinMessage>,
    mut query: Query<(&Name, &mut Text), (Without<DroppedIngredient>, With<StepIndicatorKaphrao>)>,
    mut check_drop_ingredient_text: Query<
        (&mut Text, &mut TextColor),
        (With<DroppedIngredient>, With<StepIndicatorKaphrao>),
    >,
    mut q_sprite_sheet_animation: Query<
        (&KaphraoPanStepStateTag, &mut SpritesheetAnimation),
        With<KaphraoStepSprite>,
    >,
    kaphrao_cooking_animation: Res<KaphraoCookingAnimations>,
) {
    for _ in gauge_events.read() {
        if !game_stats.ingredient_kaphrao_dropped {
            return;
        }

        let state = state.get();

        for (step_tag, mut spritesheet_animation) in q_sprite_sheet_animation.iter_mut() {
            let state = state.clone();

            if state == KaphraoCookingState::Oil {
                continue;
            }

            if step_tag.0 == state {
                if let Some(animation) = kaphrao_cooking_animation
                    .animations
                    .get(&(state.clone(), 2))
                {
                    spritesheet_animation.animation = animation.clone();
                }
            }
        }

        let next = state.next_step();
        next_state.set(next.clone());

        // Check if kaphrao cooking is finished
        if matches!(next, KaphraoCookingState::None) {
            game_stats.kaphrao_is_finished = true;

            if game_stats.kaphrao_is_finished && game_stats.egg_is_finished {
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

        game_stats.ingredient_kaphrao_dropped = false;
    }
}
