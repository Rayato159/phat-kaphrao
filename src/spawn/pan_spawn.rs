use bevy::{
    color::Color,
    ecs::{bundle::Bundle, name::Name},
    math::{Vec2, Vec3},
    picking::Pickable,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::entities::{Pan, PanEgg, PanKapaow};

pub fn pan_kapaow_spawn() -> impl Bundle {
    (
        Name::new("PanKapaow"),
        PanKapaow,
        Pan,
        Sprite {
            color: Color::srgb(0.4, 0.4, 0.4), // Dark grey pan
            custom_size: Some(Vec2::new(300.0, 150.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-150.0, -100.0, 0.0)),
        Pickable::default(),
    )
}

pub fn pan_egg_spawn() -> impl Bundle {
    (
        Name::new("PanEgg"),
        PanEgg,
        Pan,
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Slightly lighter grey for Egg pan
            custom_size: Some(Vec2::new(300.0, 150.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(150.0, -100.0, 0.0)),
        Pickable::default(),
    )
}
