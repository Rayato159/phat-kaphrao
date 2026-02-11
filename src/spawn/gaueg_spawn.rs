use crate::entities::{gauge::GaugeFollowsPan, BallGaugeEgg, BallGaugeKapaow};
use bevy::{
    color::Color,
    ecs::{component::Component, entity::Entity, name::Name, system::Commands},
    log::info,
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

pub fn gaueg_bar_spawn(
    commands: &mut Commands,
    position: Vec3,
    gauge_data: impl Component,
    name: &str,
    ball_color: Color,
    target_zone_start: f32,
    target_zone_end: f32,
) -> Entity {
    let gauge_width = 600.0;
    let gauge_height = 40.0;

    info!(
        "Spawning {} gauge with target zone: {:.2} to {:.2}",
        name, target_zone_start, target_zone_end
    );

    // Spawn the gauge container
    let gauge_entity = commands
        .spawn((
            Name::new(format!("{}Gauge", name)),
            gauge_data,
            GaugeFollowsPan {
                pan_entity: Entity::PLACEHOLDER, // Will be set by spawn_gauge_from_event
            },
            Transform::from_translation(position),
        ))
        .id();

    // Add children for gauge visualization
    let mut entity_ref = commands.entity(gauge_entity);
    entity_ref.with_children(|parent| {
        // Background bar
        parent.spawn((
            Name::new("BackgroundBar"),
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(gauge_width, gauge_height)),
                ..default()
            },
            Transform::default(),
        ));

        // Target zone (green area)
        let target_width = target_zone_end - target_zone_start;
        let target_x = (target_zone_start + target_width / 2.0 - 0.5) * gauge_width;
        parent.spawn((
            Name::new("TargetZone"),
            Sprite {
                color: Color::srgb(0.3, 0.8, 0.3),
                custom_size: Some(Vec2::new(target_width * gauge_width, gauge_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(target_x, 0.0, 0.1)),
        ));

        // Moving ball
        parent.spawn((
            Name::new("MovingBall"),
            Sprite {
                color: ball_color,
                custom_size: Some(Vec2::new(20.0, gauge_height + 10.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.2)),
        ));
    });

    gauge_entity
}

pub fn spawn_ball_gauge_kapaow(
    commands: &mut Commands,
    position: Vec3,
    target_zone_start: f32,
    target_zone_end: f32,
) -> Entity {
    let gauge_data = BallGaugeKapaow {
        target_zone_start,
        target_zone_end,
        ..default()
    };
    gaueg_bar_spawn(
        commands,
        position,
        gauge_data,
        "Kapaow",
        Color::srgb(1.0, 0.4, 0.0), // Orange for Kapaow
        target_zone_start,
        target_zone_end,
    )
}

pub fn spawn_ball_gauge_egg(
    commands: &mut Commands,
    position: Vec3,
    target_zone_start: f32,
    target_zone_end: f32,
) -> Entity {
    let gauge_data = BallGaugeEgg {
        target_zone_start,
        target_zone_end,
        ..default()
    };
    gaueg_bar_spawn(
        commands,
        position,
        gauge_data,
        "Egg",
        Color::srgb(1.0, 0.9, 0.6), // Light yellow for Egg
        target_zone_start,
        target_zone_end,
    )
}
