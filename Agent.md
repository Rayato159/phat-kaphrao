# 👨‍🍳 Project: Pad Kaprao - Development Plan

## Phase 1: Core Engine Setup ✅
- [x] Initial Bevy 0.17 project setup.
- [x] Modular folder structure (entities/, systems/, game_state/).
- [x] Basic Camera and Scene setup.

## Phase 2: Ingredient Mechanics (8 Ingredients) ✅
- [x] Implement Drag & Drop System for ingredients.
- [x] Logic for detecting "Drop on Pan" coordinates.
- [x] Return to origin logic if drop fails.
- [x] Arrange ingredients in 4x2 grid layout on right side (for easy dragging):
  - Row 0: Oil, Garlic
  - Row 1: Pork, Egg
  - Row 2: Fish Sauce, Oyster Sauce
  - Row 3: Thai Chilli, Holy Basil Leaves
- [x] Two pans (Kapaow and Egg) as main characters in center position
- [x] Window resolution set to 1920x1080 for optimal gameplay

## Phase 3: Cooking Mini-game (The Gauge) ✅
- [x] Create `GaugeSpawnEvent` system.
- [x] Implement `BallGaugeKapaow` and `BallGaugeEgg` components & movement logic.
- [x] Each pan has its own dedicated gauge that follows it.
- [x] Randomized target zones (position varies each gauge spawn).
- [x] Hit detection (Perfect/Miss) and HP deduction.
- [x] Trigger ball gauge when Oil is added to the pan.
- [x] Set up two cooking pans (PanKapaow and PanEgg) as main characters

## Phase 4: Recipe & Progress (8-Step Sequence) ✅
- [x] Ingredient sequence tracking:
  1. Oil → 2. Garlic → 3. Pork → 4. Egg
  5. Fish Sauce → 6. Oyster Sauce → 7. Thai Chilli → 8. Holy Basil Leaves
- [x] HP System (4 hearts for 8 ingredients).
- [x] Game Over state (0 HP).
- [x] UI HUD (HP Hearts, Current Step display).
- [x] Victory condition after completing all 8 ingredients.

## Phase 5: Polish & Juice ✅
- [x] Win/Lose screens.
- [x] Restart functionality.
- [x] Main menu with Start and Quit buttons.
- [x] Window configuration (1920x1080 resolution).
- [ ] Sound effects (Sizzling sound).
- [ ] Visual feedback (Shake effect on miss).

## Known Issues Fixed ✅
- ✅ Fixed: Query conflict (B0001) in `on_drag_end` - used `ParamSet` to resolve multiple Transform queries
- ✅ Fixed: Project compiles and builds successfully with no errors

## Phase 6: Advanced Features (Optional) 🚀
- [ ] Visual icons/sprites for each ingredient type.
- [ ] Animations for ingredient transitions.
- [ ] Multiple difficulty levels (faster gauge, smaller target zones).
- [ ] Scoring system based on timing accuracy.