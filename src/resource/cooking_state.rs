use bevy::prelude::*;

use crate::entities::ingredient::IngredientType;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum KaphraoCookingState {
    #[default]
    Oil,
    Garlic,
    Chili,
    Pork,
    OysterSauce,
    MSG,
    Kaphrao,
    None,
}

impl KaphraoCookingState {
    pub fn next_step(&self) -> Self {
        match self {
            KaphraoCookingState::Oil => KaphraoCookingState::Garlic,
            KaphraoCookingState::Garlic => KaphraoCookingState::Chili,
            KaphraoCookingState::Chili => KaphraoCookingState::Pork,
            KaphraoCookingState::Pork => KaphraoCookingState::OysterSauce,
            KaphraoCookingState::OysterSauce => KaphraoCookingState::MSG,
            KaphraoCookingState::MSG => KaphraoCookingState::Kaphrao,
            KaphraoCookingState::Kaphrao => KaphraoCookingState::None,
            KaphraoCookingState::None => KaphraoCookingState::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            KaphraoCookingState::Oil => "Oil".to_string(),
            KaphraoCookingState::Garlic => "Garlic".to_string(),
            KaphraoCookingState::Chili => "Chili".to_string(),
            KaphraoCookingState::Pork => "Pork".to_string(),
            KaphraoCookingState::OysterSauce => "Oyster Sauce".to_string(),
            KaphraoCookingState::MSG => "MSG".to_string(),
            KaphraoCookingState::Kaphrao => "Kaphrao".to_string(),
            KaphraoCookingState::None => "".to_string(),
        }
    }
}

impl From<KaphraoCookingState> for IngredientType {
    fn from(state: KaphraoCookingState) -> Self {
        match state {
            KaphraoCookingState::Oil => IngredientType::Oil,
            KaphraoCookingState::Garlic => IngredientType::Garlic,
            KaphraoCookingState::Chili => IngredientType::Chili,
            KaphraoCookingState::Pork => IngredientType::Pork,
            KaphraoCookingState::OysterSauce => IngredientType::OysterSauce,
            KaphraoCookingState::MSG => IngredientType::MSG,
            KaphraoCookingState::Kaphrao => IngredientType::Kaphrao,
            KaphraoCookingState::None => IngredientType::None,
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
