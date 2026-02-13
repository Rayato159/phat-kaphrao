//! Egg Cooking Systems
//!
//! Contains systems related to egg pan cooking logic:
//! - next_step_egg_cooking: Advances egg cooking state when gauge is hit

use bevy::prelude::*;

use crate::{
    entities::ingredient::DroppedIngredient,
    message::gaug_message::GaugeEggHitMassage,
    resource::{cooking_state::EggCookingState, game_state::GameState},
};

pub fn next_step_egg_cooking(
    mut game_stats: ResMut<GameState>,
    egg_state: Res<State<EggCookingState>>,
    mut next_state: ResMut<NextState<EggCookingState>>,
    mut gauge_events: MessageReader<GaugeEggHitMassage>,
    mut query: Query<(&Name, &mut Text), Without<DroppedIngredient>>,
    mut check_drop_ingredient_text: Query<(&mut Text, &mut TextColor), With<DroppedIngredient>>,
) {
    for _ in gauge_events.read() {
        info!("enter egg cooking system");
        info!(
            "game_stats.ingredient_dropped {:?}",
            game_stats.ingredient_dropped
        );
        if !game_stats.ingredient_dropped {
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
            if !game_stats.kapaow_is_finished {
                info!("🎮 Egg finished first! Egg wins!");
                // You could trigger a GameWinMessage here with egg as winner
            }
        }

        // Update UI to show next ingredient
        for (name, mut text) in &mut query {
            if name.as_str() == "NextIngredient" {
                // Convert EggCookingState to string for display
                text.0 = match *egg_state.get() {
                    EggCookingState::Oil => "Egg".to_string(),
                    EggCookingState::Egg => "".to_string(),
                    EggCookingState::None => "".to_string(),
                };
            }
        }
        for (mut text, mut color) in &mut check_drop_ingredient_text {
            *text = Text::new("No");
            *color = TextColor(Color::srgb(1.0, 0.3, 0.5));
        }
        game_stats.ingredient_dropped = false;
    }
}
