use bevy::prelude::*;

use crate::entities::{
    ingredient::PanArea,
    pan::{PanEgg, PanKaphrao},
    spatula::Spatula,
};

pub fn pan_kaphrao_spawn(pan_sprite: Handle<Image>, transform: Transform) -> impl Bundle {
    (
        PanKaphrao,
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
