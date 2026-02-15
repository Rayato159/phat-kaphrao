use crate::{entities::gauge::RectGauge, resource::game_state::GameState};
use bevy::prelude::*;

pub fn moving_ball_gauge_animation(
    game_stats: Res<GameState>,
    q_gauge: Single<&mut RectGauge>,
    mut q_rect: Query<&mut Transform, With<RectGauge>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    let mut gauge = q_gauge.into_inner();
    let mut new_position = gauge.position + gauge.direction * gauge.speed * delta;
    if new_position >= 1.0 {
        new_position = 1.0;
        gauge.direction = -1.0;
    } else if new_position <= 0.0 {
        new_position = 0.0;
        gauge.direction = 1.0;
    }
    gauge.position = new_position;

    for mut transform in q_rect.iter_mut() {
        transform.translation.x = (new_position - 0.5) * game_stats.gauge_container_width;
    }
}
