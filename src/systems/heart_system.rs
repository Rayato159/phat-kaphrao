use bevy::prelude::*;

use crate::{
    entities::heart::{Heart, HudRoot},
    resource::{game_state::GameState, heart::HeartAtlasUi},
    spawn::heart_spawn::{hp_container_parent_spawn, setup_hud_spawn, spawn_heart_ui},
};

pub fn setup_heart_atlas_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image: Handle<Image> = asset_server.load("ui/image/Heart.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(120, 120), // cell size
        2,                    // columns
        1,                    // rows
        None,
        None,
    );
    let layout = layouts.add(layout);

    commands.insert_resource(HeartAtlasUi { image, layout });
}

fn spawn_hearts_container(
    builder: &mut ChildSpawnerCommands,
    current_hp: u8,
    max_hp: u8,
    atlas: &HeartAtlasUi,
) {
    builder
        .spawn(hp_container_parent_spawn())
        .with_children(|parent| {
            let hp_per_heart = (max_hp as f32) / 5.0;

            for i in 0..5u8 {
                let hp_in_slot = current_hp as f32 - (i as f32 * hp_per_heart);

                let frame_index = if hp_in_slot >= hp_per_heart {
                    1 // full
                } else {
                    0 // empty
                };

                parent.spawn(spawn_heart_ui(atlas, frame_index, i));
            }

            info!("Spawned 5 hearts with {}/{} HP", current_hp, max_hp);
        });
}
pub fn update_hearts_ui(
    game_state: ResMut<GameState>,
    mut q: Query<(&mut Heart, &mut ImageNode), With<Heart>>,
    atlas: Res<HeartAtlasUi>,
) {
    let hp_per_heart = (game_state.max_hp as f32) / 5.0;

    for (mut heart, mut image_node) in q.iter_mut() {
        let i = heart.slot_index as f32;
        let hp_in_slot = game_state.hp as f32 - (i * hp_per_heart);

        let new_index = if hp_in_slot >= hp_per_heart { 1 } else { 0 };

        if heart.frame_index != new_index {
            heart.frame_index = new_index;
            *image_node = ImageNode::from_atlas_image(
                atlas.image.clone(),
                TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: new_index,
                },
            );
        }
    }
}

pub fn spawn_hud_and_hearts(
    mut commands: Commands,
    game_state: Res<GameState>,
    atlas: Option<Res<HeartAtlasUi>>, // กัน resource ยังไม่มา
) {
    let Some(atlas) = atlas else {
        warn!("HeartAtlasUi not ready yet; skip spawning HUD this frame.");
        return;
    };

    // HUD root
    commands
        .spawn((setup_hud_spawn(), HudRoot))
        .with_children(|builder| {
            spawn_hearts_container(builder, game_state.hp, game_state.max_hp, &atlas);
        });
}

pub fn cleanup_hud(mut commands: Commands, q: Query<Entity, With<HudRoot>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
