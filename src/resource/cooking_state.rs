use bevy::prelude::*;

use crate::entities::ingredient::IngredientType;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum KaprowCookingState {
    #[default]
    Oil,
    Garlic,
    Chili,
    Pork,
    OysterSauce,
    MSG,
    Kaprow,
    None,
}

impl KaprowCookingState {
    pub fn next_step(&self) -> Self {
        match self {
            KaprowCookingState::Oil => KaprowCookingState::Garlic,
            KaprowCookingState::Garlic => KaprowCookingState::Chili,
            KaprowCookingState::Chili => KaprowCookingState::Pork,
            KaprowCookingState::Pork => KaprowCookingState::OysterSauce,
            KaprowCookingState::OysterSauce => KaprowCookingState::MSG,
            KaprowCookingState::MSG => KaprowCookingState::Kaprow,
            KaprowCookingState::Kaprow => KaprowCookingState::None,
            KaprowCookingState::None => KaprowCookingState::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            KaprowCookingState::Oil => "Oil".to_string(),
            KaprowCookingState::Garlic => "Garlic".to_string(),
            KaprowCookingState::Chili => "Chili".to_string(),
            KaprowCookingState::Pork => "Pork".to_string(),
            KaprowCookingState::OysterSauce => "Oyster Sauce".to_string(),
            KaprowCookingState::MSG => "MSG".to_string(),
            KaprowCookingState::Kaprow => "Kaprow".to_string(),
            KaprowCookingState::None => "".to_string(),
        }
    }
}

impl From<KaprowCookingState> for IngredientType {
    fn from(state: KaprowCookingState) -> Self {
        match state {
            KaprowCookingState::Oil => IngredientType::Oil,
            KaprowCookingState::Garlic => IngredientType::Garlic,
            KaprowCookingState::Chili => IngredientType::Chili,
            KaprowCookingState::Pork => IngredientType::Pork,
            KaprowCookingState::OysterSauce => IngredientType::OysterSauce,
            KaprowCookingState::MSG => IngredientType::MSG,
            KaprowCookingState::Kaprow => IngredientType::Kaprow,
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
            EggCookingState::Egg => EggCookingState::None,
            EggCookingState::None => EggCookingState::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EggCookingState::Oil => "Oil".to_string(),
            EggCookingState::Egg => "Egg".to_string(),
            EggCookingState::None => "".to_string(),
        }
    }
}

impl From<EggCookingState> for IngredientType {
    fn from(state: EggCookingState) -> Self {
        match state {
            EggCookingState::Oil => IngredientType::Oil,
            EggCookingState::Egg => IngredientType::Egg,
            EggCookingState::None => IngredientType::None,
        }
    }
}
