//! Egg Cooking Systems
//!
//! Contains systems related to egg pan cooking logic:
//! - next_step_egg_cooking: Advances egg cooking state when gauge is hit

use bevy::prelude::*;

use crate::{
    entities::{ingredient::DroppedIngredient, StepIndicatorEgg},
    message::gaug_message::GaugeEggHitMassage,
    resource::{cooking_state::EggCookingState, game_state::GameState},
    GameWinMessage,
};

pub fn next_step_egg_cooking(
    mut game_stats: ResMut<GameState>,
    egg_state: Res<State<EggCookingState>>,
    mut next_state: ResMut<NextState<EggCookingState>>,
    mut gauge_events: MessageReader<GaugeEggHitMassage>,
    mut game_win: MessageWriter<GameWinMessage>,
    mut query: Query<(&Name, &mut Text), (Without<DroppedIngredient>, With<StepIndicatorEgg>)>,
    mut check_drop_ingredient_text: Query<
        (&mut Text, &mut TextColor),
        (With<DroppedIngredient>, With<StepIndicatorEgg>),
    >,
) {
    for _ in gauge_events.read() {
        info!("enter egg cooking system");
        info!(
            "game_stats.ingredient_dropped {:?}",
            game_stats.ingredient_egg_dropped
        );
        if !game_stats.ingredient_egg_dropped {
            return;
        }
        let next = egg_state.get().next_step();

        info!("update egg state {:?}", next);

        next_state.set(next.clone());

        // Check if egg cooking is finished
        if matches!(next, EggCookingState::None) {
            info!("🥚 EGG COOKING FINISHED!");
            game_stats.egg_is_finished = true;

            // Check if kapaow is not finished yet - egg wins!
            if game_stats.kapaow_is_finished && game_stats.egg_is_finished {
                info!("🎮 Egg finished first! Egg wins!");
                game_win.write(GameWinMessage);
                // You could trigger a GameWinMessage here with egg as winner
            }
        }

        // Update UI to show next ingredient
        let mut found_entities = 0;
        for (name, mut text) in &mut query {
            found_entities += 1;
            if name.as_str() == "NextIngredient" {
                // Convert EggCookingState to string for display
                // Use next state (what we just set) instead of current state
                text.0 = next.to_string();
                info!("Updated NextIngredient text to: {}", text.0);
            }
        }
        info!(
            "Total entities found with StepIndicatorKapaow (Without<DroppedIngredient>): {}",
            found_entities
        );
        let mut found_drop_entities = 0;
        for (mut text, mut color) in &mut check_drop_ingredient_text {
            found_drop_entities += 1;
            *text = Text::new("No");
            *color = TextColor(Color::srgb(1.0, 0.3, 0.5));
            info!("Updated DroppedIngredient text to: No");
        }
        info!(
            "Total DroppedIngredient entities found: {}",
            found_drop_entities
        );
        game_stats.ingredient_egg_dropped = false;
    }
}
