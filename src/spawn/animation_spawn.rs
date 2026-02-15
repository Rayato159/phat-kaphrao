use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::resource::cooking_state::{EggCookingState, KaphraoCookingState};

pub fn insert_kaphrao_cooking_animation(
    state: KaphraoCookingState,
    spritesheet: Spritesheet,
    duration_ms: u32,
    animations: &mut ResMut<Assets<Animation>>,
) -> Vec<((KaphraoCookingState, u8), Handle<Animation>)> {
    let raw_animation = spritesheet
        .create_animation()
        .add_row(0)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let cooked_animation = spritesheet
        .create_animation()
        .add_row(1)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let burned_animation = spritesheet
        .create_animation()
        .add_row(2)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let raw_animation_handle = animations.add(raw_animation);
    let cooked_animation_handle = animations.add(cooked_animation);
    let burned_animation_handle = animations.add(burned_animation);

    vec![
        ((state.clone(), 1), raw_animation_handle),
        ((state.clone(), 2), cooked_animation_handle),
        ((state.clone(), 3), burned_animation_handle),
    ]
}

pub fn insert_egg_cooking_animation(
    state: EggCookingState,
    spritesheet: Spritesheet,
    duration_ms: u32,
    animations: &mut ResMut<Assets<Animation>>,
) -> Vec<((EggCookingState, u8), Handle<Animation>)> {
    let raw_animation = spritesheet
        .create_animation()
        .add_row(0)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let cooked_animation = spritesheet
        .create_animation()
        .add_row(1)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let burned_animation = spritesheet
        .create_animation()
        .add_row(2)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let raw_animation_handle = animations.add(raw_animation);
    let cooked_animation_handle = animations.add(cooked_animation);
    let burned_animation_handle = animations.add(burned_animation);

    vec![
        ((state.clone(), 1), raw_animation_handle),
        ((state.clone(), 2), cooked_animation_handle),
        ((state.clone(), 3), burned_animation_handle),
    ]
}
