use bevy::prelude::*;

pub const INGREDIENT_SIZE: f32 = 16.0 * 7.5; // 16 is original ingredient size, scaled up by 7.5

/// Marker component for ingredients
/// Uses Required Components to ensure ingredients always have Transform, Visibility, and Pickable
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Ingredient {
    pub ingredient_type: IngredientType,
}
#[derive(Component)]
pub struct IngredientNext;
/// Marker component for ingredient background sprite (Icon.png)
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct IngredientBackground;

/// Marker component for ingredient foreground sprite (specific ingredient image)
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct IngredientForeground;

/// Links foreground sprite to its parent ingredient entity
#[derive(Component, Debug)]
pub struct IngredientForegroundLink {
    pub parent_entity: Entity,
}

/// Stores original parent entity and local transform when dragging starts
/// Used to reparent foreground sprite back to its original position after drag
#[derive(Component, Debug)]
pub struct DraggingOriginalParent {
    pub parent_entity: Entity,
    pub original_transform: Transform,
}

/// Stores the original z-index for hover effects
/// Used to restore z-index after hover ends
#[derive(Component, Debug)]
pub struct HoverOriginalZ {
    pub z: f32,
}

/// Marker component for ingredient menu
/// Used to organize and display ingredients in the UI menu
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct IngredientMenu;

/// Ingredient types available in the game
/// 8 ingredients total with a specific cooking sequence
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IngredientType {
    #[default]
    Oil,
    Garlic,
    Pork,
    Egg,
    FishSauce,
    OysterSauce,
    ThaiChilli,
    HolyBasilLeaves,
}

impl IngredientType {
    /// Returns the ingredient type for a given step index
    /// Steps 0-7 correspond to the 8 ingredients
    /// Step 8+ returns None (completed state)
    pub fn from_step(step: usize) -> Option<Self> {
        match step {
            0 => Some(IngredientType::Oil),
            1 => Some(IngredientType::Garlic),
            2 => Some(IngredientType::Pork),
            3 => Some(IngredientType::Egg),
            4 => Some(IngredientType::FishSauce),
            5 => Some(IngredientType::OysterSauce),
            6 => Some(IngredientType::ThaiChilli),
            7 => Some(IngredientType::HolyBasilLeaves),
            _ => None,
        }
    }

    /// Returns the display name of the ingredient
    pub fn name(&self) -> &str {
        match self {
            IngredientType::Oil => "Oil",
            IngredientType::Garlic => "Garlic",
            IngredientType::Pork => "Pork",
            IngredientType::Egg => "Egg",
            IngredientType::FishSauce => "Fish Sauce",
            IngredientType::OysterSauce => "Oyster Sauce",
            IngredientType::ThaiChilli => "Thai Chilli",
            IngredientType::HolyBasilLeaves => "Holy Basil Leaves",
        }
    }

    /// Returns the color for visualizing this ingredient
    pub fn color(&self) -> Color {
        match self {
            IngredientType::Oil => Color::srgb(0.9, 0.8, 0.5), // Yellow-ish
            IngredientType::Garlic => Color::srgb(0.95, 0.95, 0.9), // White-ish
            IngredientType::Pork => Color::srgb(0.9, 0.7, 0.7), // Pink-ish
            IngredientType::Egg => Color::srgb(1.0, 0.9, 0.6), // Yellow-orange
            IngredientType::FishSauce => Color::srgb(0.6, 0.7, 0.8), // Brown-ish
            IngredientType::OysterSauce => Color::srgb(0.5, 0.4, 0.2), // Dark brown
            IngredientType::ThaiChilli => Color::srgb(0.9, 0.2, 0.1), // Red
            IngredientType::HolyBasilLeaves => Color::srgb(0.3, 0.5, 0.3), // Dark green
        }
    }

    /// Returns the image path for this ingredient
    pub fn image_path(&self) -> &'static str {
        match self {
            IngredientType::Oil => "ingradients/image/Oil.png",
            IngredientType::Garlic => "ingradients/image/Galic.png",
            IngredientType::Pork => "ingradients/image/Pork.png",
            IngredientType::Egg => "ingradients/image/Egg.png",
            IngredientType::FishSauce => "ingradients/image/MSG.png",
            IngredientType::OysterSauce => "ingradients/image/OysterSauce.png",
            IngredientType::ThaiChilli => "ingradients/image/Chili.png",
            IngredientType::HolyBasilLeaves => "ingradients/image/KaProw.png",
        }
    }
}

/// Original position of an ingredient (for return-to-origin logic)
#[derive(Component, Default, Debug)]
pub struct OriginalPosition {
    pub position: Vec3,
}

/// Current dragging state
#[derive(Component, Default, Debug)]
pub struct Dragging {
    pub offset: Vec2,
}

/// Marker component for the pan drop area
#[derive(Component)]
pub struct PanArea;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DroppedIngredient;
