use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::entities::pan::INGREDIENT_SIZE;

pub fn spawn_ingredient_animation(
    image: Handle<Image>,
    row: usize,
    col: usize,
    duration_ms: u32,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    animations: &mut ResMut<Assets<Animation>>,
) -> (Sprite, SpritesheetAnimation) {
    let spritesheet = Spritesheet::new(&image, col, row);

    let animation = spritesheet
        .create_animation()
        .add_row(0)
        .set_duration(AnimationDuration::PerFrame(duration_ms))
        .build();

    let animation_handle = animations.add(animation);

    let sprite = spritesheet
        .with_size_hint(
            INGREDIENT_SIZE as u32 * col as u32,
            INGREDIENT_SIZE as u32 * row as u32,
        )
        .sprite(atlas_layouts);

    (sprite, SpritesheetAnimation::new(animation_handle))
}
