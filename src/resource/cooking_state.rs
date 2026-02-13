use bevy::prelude::*;

use crate::entities::IngredientType;

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
    None,
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
            KaprowCookingState::Basil => KaprowCookingState::None,
            KaprowCookingState::None => KaprowCookingState::None,
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
            KaprowCookingState::None => "".to_string(),
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
            KaprowCookingState::None => IngredientType::None,
        }
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum EggCookingState {
    #[default]
    Oil,
    Egg,
    None,
}

impl EggCookingState {
    pub fn next_step(&self) -> Self {
        match self {
            EggCookingState::Oil => EggCookingState::Egg,
            EggCookingState::Egg => EggCookingState::Oil,
            EggCookingState::None => EggCookingState::None,
        }
    }
}
