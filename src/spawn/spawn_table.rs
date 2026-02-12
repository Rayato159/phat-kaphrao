use bevy::prelude::*;

pub fn table_spawn(table_sprite: Handle<Image>, transform: Transform) -> impl Bundle {
    (
        Sprite {
            image: table_sprite,
            ..default()
        },
        transform,
    )
}
