use bevy::prelude::*;

use crate::{
    entities::{ingredient::DroppedIngredient, StepIndicatorKapaow},
    message::gaug_message::GaugeKapoawHitMassage,
    resource::{cooking_state::KaprowCookingState, game_state::GameState},
    GameWinMessage,
};

pub fn next_step_kapaow_cooking(
    mut game_stats: ResMut<GameState>,
    kapow_state: Res<State<KaprowCookingState>>,
    mut next_state: ResMut<NextState<KaprowCookingState>>,
    mut gauge_events: MessageReader<GaugeKapoawHitMassage>,
    mut game_win: MessageWriter<GameWinMessage>,
    mut query: Query<(&Name, &mut Text), (Without<DroppedIngredient>, With<StepIndicatorKapaow>)>,
    mut check_drop_ingredient_text: Query<
        (&mut Text, &mut TextColor),
        (With<DroppedIngredient>, With<StepIndicatorKapaow>),
    >,
) {
    for _ in gauge_events.read() {
        info!("enter cooking xxxxxxxxx");
        info!(
            "game_stats.ingredient_dropped {:?}",
            game_stats.ingredient_kapaow_dropped
        );
        if !game_stats.ingredient_kapaow_dropped {
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
            if game_stats.kapaow_is_finished && game_stats.egg_is_finished {
                info!("🎮 Kapaow finished first! Kapaow wins!");
                game_win.write(GameWinMessage);
                // You could trigger a GameWinMessage here with kapaow as winner
            }
        }

        // Update UI to show next ingredient
        let mut found_entities = 0;
        for (name, mut text) in &mut query {
            found_entities += 1;
            info!("Found entity: {:?}", name.as_str());
            if name.as_str() == "NextIngredient" {
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
        game_stats.ingredient_kapaow_dropped = false;
    }
}
