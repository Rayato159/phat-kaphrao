use bevy::{
    camera::{Camera, Camera2d, ClearColorConfig},
    color::Color,
    ecs::bundle::Bundle,
    utils::default,
};

pub fn camera_2d_spawn() -> impl Bundle {
    (
        Camera2d,
        Camera {
            // Clear to a nice kitchen-like background color
            clear_color: ClearColorConfig::Custom(Color::srgb(0.95, 0.92, 0.88)),
            ..default()
        },
    )
}
