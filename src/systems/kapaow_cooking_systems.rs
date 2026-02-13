use bevy::prelude::*;

use crate::{
    entities::ingredient::DroppedIngredient,
    message::gaug_message::GaugeKapoawHitMassage,
    resource::{cooking_state::KaprowCookingState, game_state::GameState},
};

pub fn next_step_kapaow_cooking(
    mut game_stats: ResMut<GameState>,
    kapow_state: Res<State<KaprowCookingState>>,
    mut next_state: ResMut<NextState<KaprowCookingState>>,
    mut gauge_events: MessageReader<GaugeKapoawHitMassage>,
    mut query: Query<(&Name, &mut Text), Without<DroppedIngredient>>,
    mut check_drop_ingredient_text: Query<(&mut Text, &mut TextColor), With<DroppedIngredient>>,
) {
    for _ in gauge_events.read() {
        info!("enter cooking xxxxxxxxx");
        info!(
            "game_stats.ingredient_dropped {:?}",
            game_stats.ingredient_dropped
        );
        if !game_stats.ingredient_dropped {
            return;
        }
        let next = kapow_state.get().next_step();

        info!("update kapaow state {:?}", next);

        next_state.set(next.clone());

        // Check if kapaow cooking is finished
        if matches!(next, KaprowCookingState::None) {
            info!("🌶️ KAPOAW COOKING FINISHED!");
            game_stats.kapaow_is_finished = true;

            // Check if egg is not finished yet - kapaow wins!
            if !game_stats.egg_is_finished {
                info!("🎮 Kapaow finished first! Kapaow wins!");
                // You could trigger a GameWinMessage here with kapaow as winner
            }
        }

        for (name, mut text) in &mut query {
            if name.as_str() == "NextIngredient" {
                text.0 = next.to_string();
            }
        }
        for (mut text, mut color) in &mut check_drop_ingredient_text {
            *text = Text::new("No");
            *color = TextColor(Color::srgb(1.0, 0.3, 0.5));
        }
        game_stats.ingredient_dropped = false;
    }
}
