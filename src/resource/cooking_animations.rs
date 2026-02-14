use std::collections::HashMap;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::resource::cooking_state::{EggCookingState, KaprowCookingState};

#[derive(Resource, Clone, Debug, Default)]
pub struct KaprowCookingAnimations {
    pub animations: HashMap<(KaprowCookingState, u8), Handle<Animation>>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct EggCookingAnimations {
    pub animations: HashMap<(EggCookingState, u8), Handle<Animation>>,
}
