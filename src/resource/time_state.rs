use bevy::{
    ecs::{
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    state::state::NextState,
    time::Time,
};

use crate::AppState;

pub const MAX_GAME_TIME: f64 = 142.0;

#[derive(Resource)]
pub struct StartTime(f64);

impl StartTime {
    pub fn get(&self) -> f64 {
        self.0
    }
}

pub fn start_timer(time: Res<Time>, mut commands: Commands) {
    commands.insert_resource(StartTime(time.elapsed_secs_f64()));
}

pub fn check_game_timer(
    time: Res<Time>,
    start: Res<StartTime>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let passed = time.elapsed_secs_f64() - start.get();

    if passed >= MAX_GAME_TIME {
        println!("หมดเวลา!");
        next_state.set(AppState::GameOver);
    }
}
