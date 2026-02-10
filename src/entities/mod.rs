//! Entities Module
//!
//! Contains all component definitions and marker structs for the game.
//! These are the data structures that define what entities are and what properties they have.
//!
//! Modules:
//! - ingredient: Ingredient components (Ingredient, IngredientType, OriginalPosition, Dragging, Pan)
//! - gauge: Gauge components and events (TimingGauge, BarGauge, BallGaugeKapaow, BallGaugeEgg, GaugeSpawnEvent)
//! - pan: Pan marker components (FryingPan, PanKapaow, PanEgg)
//! - ui: UI components (HUD, HeartIcon, StepIndicator, GameEndScreen)

pub mod gauge;
pub mod ingredient;
pub mod pan;
pub mod ui;

// Re-export commonly used items for convenience
pub use gauge::{BallGaugeEgg, BallGaugeKapaow, BarGauge, GaugeSpawnEvent, TimingGauge};
pub use ingredient::{
    Dragging, DraggingOriginalParent, HoverOriginalZ, Ingredient, IngredientBackground,
    IngredientForeground, IngredientForegroundLink, IngredientMenu, IngredientType,
    OriginalPosition, Pan,
};
pub use pan::{FryingPan, PanEgg, PanKapaow};
pub use ui::{ButtonAction, GameEndScreen, HPOverlay, HeartIcon, MainMenu, StepIndicator, HUD};
