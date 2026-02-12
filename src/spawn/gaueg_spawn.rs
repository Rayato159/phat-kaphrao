use crate::entities::{gauge::GaugeFollowsPan, BallGauge};
use bevy::{
    color::Color,
    ecs::{bundle::Bundle, component::Component, entity::Entity, name::Name, system::Commands},
    log::info,
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

pub fn gauge_container_spawn(position: Vec3) -> impl Bundle {
    (
        Name::new("GaugeContainer"),
        GaugeFollowsPan {
            pan_entity: Entity::PLACEHOLDER, // Will be set by spawn_gauge_from_event
        },
        Transform::from_translation(position),
    )
}

pub fn gauge_container_background_spawn(gauge_width: f32, gauge_height: f32) -> impl Bundle {
    (
        Name::new("BackgroundBar"),
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(gauge_width, gauge_height)),
            ..default()
        },
        Transform::default(),
    )
}

pub fn gauge_target_zone_spawn(
    zone_name: &'static str,
    target_width: f32,
    gauge_width: f32,
    gauge_height: f32,
    target_x: f32,
) -> impl Bundle {
    (
        Name::new(zone_name),
        Sprite {
            color: Color::srgb(0.3, 0.8, 0.3),
            custom_size: Some(Vec2::new(target_width * gauge_width, gauge_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(target_x, 0.0, 0.1)),
    )
}

pub fn gauge_ball_spawn(gauge_height: f32, ball_color: Color) -> impl Bundle {
    (
        Name::new("MovingBall"),
        BallGauge::default(),
        Sprite {
            color: ball_color,
            custom_size: Some(Vec2::new(20.0, gauge_height + 10.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.2)),
    )
}
