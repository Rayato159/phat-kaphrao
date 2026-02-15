use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct HeartAtlasUi {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
