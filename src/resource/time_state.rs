use bevy::{
    ecs::{
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    state::state::NextState,
    time::Time,
};

use crate::AppState;

#[derive(Resource)]
pub struct StartTime(f64);

pub fn start_timer(time: Res<Time>, mut commands: Commands) {
    commands.insert_resource(StartTime(time.elapsed_secs_f64()));
}

pub fn check_game_timer(
    time: Res<Time>,
    start: Res<StartTime>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let passed = time.elapsed_secs_f64() - start.0;

    if passed >= 180.0 {
        println!("หมดเวลา!");
        next_state.set(AppState::GameOver);
    }
}
