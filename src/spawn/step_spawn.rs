use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::entities::{
    ingredient::DroppedIngredient, IngredientNext, IngredientType, StepIndicator,
};

pub fn step_parent_spawn() -> impl Bundle {
    (
        Name::new("StepIndicator"),
        StepIndicator,
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            row_gap: Val::Px(5.0),
            ..default()
        },
    )
}

pub fn step_child_spawn() -> impl Bundle {
    (
        Text::new("Next Ingredient:"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.2, 0.2)),
    )
}

pub fn step_child_current_spawn() -> impl Bundle {
    (
        Name::new("NextIngredient"),
        IngredientNext,
        Text::new("Oil"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.6, 0.2)),
    )
}

pub fn step_child_drop_spawn() -> impl Bundle {
    (
        Name::new("DropIngredient"),
        DroppedIngredient,
        Text::new("No"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.5)),
    )
}

pub fn spawn_oil_step(
    sprite: Sprite,
    animation: Handle<Animation>,
    transform: Transform,
) -> impl Bundle {
    (sprite, SpritesheetAnimation::new(animation), transform)
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum KaprowCookingState {
    #[default]
    Oil,
    Garlic,
    Chilli,
    Pork,
    OysterSauce,
    MSG,
    Basil,
}

impl KaprowCookingState {
    pub fn next_step(&self) -> Self {
        match self {
            KaprowCookingState::Oil => KaprowCookingState::Garlic,
            KaprowCookingState::Garlic => KaprowCookingState::Chilli,
            KaprowCookingState::Chilli => KaprowCookingState::Pork,
            KaprowCookingState::Pork => KaprowCookingState::OysterSauce,
            KaprowCookingState::OysterSauce => KaprowCookingState::MSG,
            KaprowCookingState::MSG => KaprowCookingState::Basil,
            KaprowCookingState::Basil => KaprowCookingState::Oil,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            KaprowCookingState::Oil => "Oil".to_string(),
            KaprowCookingState::Garlic => "Garlic".to_string(),
            KaprowCookingState::Chilli => "Chilli".to_string(),
            KaprowCookingState::Pork => "Pork".to_string(),
            KaprowCookingState::OysterSauce => "Oyster Sauce".to_string(),
            KaprowCookingState::MSG => "MSG".to_string(),
            KaprowCookingState::Basil => "Basil".to_string(),
        }
    }
}

impl From<KaprowCookingState> for IngredientType {
    fn from(state: KaprowCookingState) -> Self {
        match state {
            KaprowCookingState::Oil => IngredientType::Oil,
            KaprowCookingState::Garlic => IngredientType::Garlic,
            KaprowCookingState::Chilli => IngredientType::ThaiChilli,
            KaprowCookingState::Pork => IngredientType::Pork,
            KaprowCookingState::OysterSauce => IngredientType::OysterSauce,
            KaprowCookingState::MSG => IngredientType::FishSauce, // หรือแก้ตาม logic คุณ
            KaprowCookingState::Basil => IngredientType::HolyBasilLeaves,
        }
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum EggCookingState {
    #[default]
    Oil,
    Egg,
}

impl EggCookingState {
    pub fn next_step(&self) -> Self {
        match self {
            EggCookingState::Oil => EggCookingState::Egg,
            EggCookingState::Egg => EggCookingState::Oil,
        }
    }
}
