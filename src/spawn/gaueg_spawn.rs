use crate::entities::{gauge::GaugeFollowsPan, BallGauge};
use bevy::prelude::{ColorMaterial, MeshMaterial2d};
use bevy::{
    asset::Assets,
    color::Color,
    ecs::{bundle::Bundle, entity::Entity, name::Name},
    math::{primitives::Circle, Vec2, Vec3},
    mesh::{Mesh, Mesh2d},
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
    color: Color,
    world_width: f32,
    gauge_height: f32,
    world_x: f32,
) -> impl Bundle {
    (
        Name::new(zone_name),
        Sprite {
            color: color,
            custom_size: Some(Vec2::new(world_width, gauge_height)),
            ..default()
        },
        Transform::from_translation(Vec3::new(world_x, 0.0, 0.1)),
    )
}
pub fn gauge_ball_spawn(
    ball_color: Color,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    (
        Name::new("MovingBall"),
        BallGauge::default(),
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(ball_color))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.2)),
    )
}

pub fn guage_perfect_spawn(name: &'static str, gauge_container_height: f32) -> impl Bundle {
    (
        Name::new(name),
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(4.0, gauge_container_height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.2), // 🔥 0.0 = กลาง zone
    )
}
