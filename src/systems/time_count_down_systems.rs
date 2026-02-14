//! Countdown Timer Systems
//!
//! Contains all systems related to the countdown timer display:
//! - spawn_countdown_timer: Spawns the countdown timer in the center of the screen
//! - update_countdown_timer: Updates the countdown timer text every frame

use bevy::prelude::*;

use crate::{
    entities::time_count_down::CountDownTimer,
    resource::time_state::{StartTime, MAX_GAME_TIME},
};

/// Spawn the countdown timer in the center of the screen
/// This system should be called when entering the InGame state
pub fn spawn_countdown_timer(mut commands: Commands) {
    commands
        .spawn((
            Name::new("CountdownTimerContainer"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(180.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            ZIndex(100),      // Below HUD but above game elements
            Pickable::IGNORE, // Don't block pointer events to ingredients
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("CountdownTimer"),
                CountDownTimer,
                Text::new("1:00"),
                TextFont {
                    font_size: 32.0 * 3.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 0.0)),
            ));
        });

    info!("Countdown timer spawned at top of screen");
}

/// Update the countdown timer text every frame
/// Calculates remaining time and updates the display in MM:SS format
pub fn update_countdown_timer(
    time: Res<Time>,
    start_time: Res<StartTime>,
    mut countdown_query: Query<&mut Text, With<CountDownTimer>>,
) {
    // Calculate elapsed time
    let elapsed = time.elapsed_secs_f64() - start_time.get();

    // Calculate remaining time (ensure it doesn't go below 0)
    let remaining = (MAX_GAME_TIME - elapsed).max(0.0);

    // Format as MM:SS
    let minutes = (remaining / 60.0) as u32;
    let seconds = (remaining % 60.0) as u32;

    // Update the text
    for mut text in countdown_query.iter_mut() {
        text.0 = format!("{}:{:02}", minutes, seconds);
    }
}
