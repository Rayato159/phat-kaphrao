use bevy::prelude::*;

use crate::entities::{spatula::Spatula, PanArea, PanEgg, PanKapaow};

pub fn pan_kaprow_spawn(pan_sprite: Handle<Image>, transform: Transform) -> impl Bundle {
    (
        PanKapaow,
        PanArea,
        Sprite {
            image: pan_sprite,
            ..default()
        },
        transform,
        Pickable::default(),
    )
}

pub fn pan_egg_spawn(pan_sprite: Handle<Image>, transform: Transform) -> impl Bundle {
    (
        PanEgg,
        PanArea,
        Sprite {
            image: pan_sprite,
            ..default()
        },
        transform,
        Pickable::default(),
    )
}

pub fn spatula_spawn(spatula_sprite: Handle<Image>, transform: Transform) -> impl Bundle {
    (
        Spatula,
        Sprite {
            image: spatula_sprite,
            ..default()
        },
        transform,
        Pickable::default(),
    )
}
