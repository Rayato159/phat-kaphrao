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

pub mod check_list;
pub mod gauge;
pub mod ingredient;
pub mod pan;
pub mod spatula;
pub mod time_count_down;
pub mod ui;

// Re-export commonly used items for convenience
pub use check_list::{
    CheckListCheckbox, CheckListIngredientType, CheckListItem, CheckListItemIcon, CheckListNumber,
    CheckListStatus, CheckListSymbol,
};
pub use gauge::{BallGauge, TimingGauge};
pub use ingredient::{
    Dragging, DraggingOriginalParent, HoverOriginalZ, Ingredient, IngredientBackground,
    IngredientForeground, IngredientForegroundLink, IngredientMenu, IngredientNext, IngredientType,
    OriginalPosition, PanArea,
};
pub use pan::{PanEgg, PanKapaow};
pub use time_count_down::CountDownTimer;
pub use ui::{
    ButtonAction, GameEndScreen, HPOverlay, HPText, HeartIcon, MainMenu, StepIndicator,
    StepIndicatorEgg, StepIndicatorKapaow, HUD,
};
