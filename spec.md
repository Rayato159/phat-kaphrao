# Pad Kaprao - Thai Cooking Game 🍳

A fun and interactive cooking game built with [Bevy 0.17](https://bevyengine.org/) where you must add ingredients in the correct order to make the perfect Pad Kaprao (Thai Basil Pork Stir-fry).

## 🎮 Features

- **Main Menu**: Interactive menu with Start and Quit game buttons
- **Two Cooking Pans**: Kapaow and Egg pans positioned in the center as main characters
- **Drag & Drop Mechanics**: Intuitive ingredient placement using Bevy's Picking API - drag ingredients from right side to the pans
- **Ball Gauge Mini-Game**: Separate gauges for Kapaow and Egg pans that follow their respective pans
- **Randomized Target Zones**: Each gauge has a randomly positioned target zone for varied gameplay
- **Strict Oil Rule**: First two ingredients must be oil to avoid game over
- **Optimized Queries**: All Query conflicts resolved using Bevy's `ParamSet` for better performance
- **HP System**: 4 hearts - lose one for each wrong ingredient
- **Step Tracking**: Progress through 8 cooking steps
- **Victory & Game Over Screens**: Clear win/lose conditions with restart/menu options
- **Modern Bevy 0.17+ Patterns**: Uses Observers, Required Components, and Computed States
- **Optimized Resolution**: Runs at 1920x1080 for the best gameplay experience

## 🧑‍🍳 How to Play

### Getting Started
1. Launch the game to see the main menu
2. Click "Start Game" to begin cooking
3. Follow the ingredient sequence shown in the HUD

### Objective
Add ingredients to the cooking pans (Kapaow and Egg) in the correct order:
1. **Oil** ⬇️ (triggers ball gauge mini-game - each pan has its own gauge)
2. **Garlic** 🧄
3. **Pork** 🥩
4. **Egg** 🥚
5. **Fish Sauce** 🐟
6. **Oyster Sauce** 🦪
7. **Thai Chilli** 🌶️
8. **Holy Basil Leaves** 🌿

### Controls
- **Mouse/Touch**: 
  - Click menu buttons to navigate
  - Drag ingredients from the right side and drop them on the cooking pans (Kapaow or Egg)
- **Spacebar**: Hit the ball gauge when it's in the green zone (when adding oil)
- **R**: Restart game (on victory/game over screens)
- **ESC**: Return to menu (on victory/game over screens)

### Rules
- Add ingredients in the correct order to progress
- Ingredients are displayed on the right side for easy dragging to the center pans
- First two ingredients must be oil (one for Kapaow pan, one for Egg pan)
- Each pan has its own ball gauge that follows it when oil is added
- If you add the wrong ingredient or don't put oil first, you lose 1 HP (out of 4 total)
- When HP reaches 0, it's Game Over
- Complete all 8 steps to achieve Victory!
- Use the menu to start new games or return from game end screens

## 🏗️ Build Instructions

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)
- Display capable of 1920x1080 resolution for optimal gameplay

### Building the Project

```bash
# Navigate to the project directory
cd pad-kaprow

# Build the project (debug mode)
cargo build

# Build with optimizations (release mode)
cargo build --release
```

### Running the Game

```bash
# Run in debug mode (faster compile)
cargo run

# Run in release mode (better performance)
cargo run --release
```

## 📁 Project Structure

```
pad-kaprow/
├── src/
│   ├── main.rs           # Main entry point and app builder
│   ├── lib.rs            # Library module exports
│   ├── entities/
│   │   ├── mod.rs        # Entities module exports
│   │   ├── ingredient.rs # Ingredient components (Ingredient, IngredientType, Dragging)
│   │   ├── pan.rs        # Pan marker components (FryingPan, PanKapaow, PanEgg)
│   │   ├── gauge.rs      # Gauge components (TimingGauge, BarGauge, BallGaugeKapaow, BallGaugeEgg, GaugeFollowsPan)
│   │   └── ui.rs         # UI components (HUD, HeartIcon, ButtonAction)
│   ├── resource/
│   │   ├── mod.rs        # Resource module exports
│   │   └── game_state.rs # Game states (AppState, InGame computed state, events)
│   ├── spawn/
│   │   ├── mod.rs        # Spawn module exports
│   │   ├── camera_spawn.rs
│   │   ├── game_over_screen_spawn.rs
│   │   ├── gaueg_spawn.rs
│   │   ├── hp_group_spawn.rs
│   │   ├── ingredient_spawn.rs
│   │   ├── main_menu_spawn.rs
│   │   ├── pan_spawn.rs
│   │   ├── setup_hud_spawn.rs
│   │   ├── step_spawn.rs
│   │   └── victory_screen_spawn.rs
│   ├── systems/
│   │   ├── mod.rs        # Systems module exports
│   │   ├── init_game_systems.rs    # Initial game setup (camera, game state)
│   │   ├── cooking_systems.rs      # Cooking logic and game mechanics
│   │   ├── pan_systems.rs          # Pan setup and management
│   │   ├── gauge_systems.rs        # Timing gauge mini-game
│   │   ├── ingredient_systems.rs   # Ingredient spawning, dragging, dropping
│   │   └── ui_systems.rs           # HUD, menus, game end screens
│   ├── animate/        # (placeholder for future animations)
│   ├── helper/         # (placeholder for future helper functions)
│   └── logic/          # (placeholder for future game logic)
```
├── assets/               # Game assets (placeholder)
├── Cargo.toml            # Project dependencies
├── prompt.md             # Project prompt and specifications
├── Agent.md              # Development plan and phases
├── skill.md              # Bevy 0.17+ skills and patterns
└── README.md             # This file
```

## 🦀 Technical Details

### Bevy 0.17+ Features Used

1. **Observers & Triggers**
   - Event-driven architecture for drag/drop and game events
   - Decouples logic (e.g., drop detection from gauge spawning)
   - Usage: `.observe(system)` on entities

2. **Required Components**
   - Ensures data integrity with `#[require(...)]` attribute
   - Example: `#[require(Transform, Visibility)]` for ingredients

3. **Computed States**
   - Automatic state derivation from source states
   - `InGame` computed state derived from `AppState`
   - Usage: `.add_computed_state::<InGame>()`

4. **Bevy Picking API**
   - Built-in interaction system for drag & drop
   - Event types: `On<Pointer<Drag>>`, `On<Pointer<DragEnd>>`, etc.
   - Component: `Pickable` for interactivity

### Design Patterns

- **ECS Architecture**: Entity-Component-System for efficient game logic
- **Modular Organization**: Separate modules for Entities, Game State, and Systems
- **Event-Driven**: Events for decoupled communication between systems
- **State Management**: Clear game state transitions with Bevy's State API

## 🚧 Development Phases

### Phase 1: Core Engine Setup ✅
- Initial Bevy project setup
- Modular folder structure
- Basic Camera and Scene setup

### Phase 2: Ingredient Mechanics ✅
- Drag & Drop System for ingredients
- Logic for detecting "Drop on Pan" coordinates
- Return to origin logic if drop fails

### Phase 3: Cooking Mini-game ✅
- `GaugeSpawnEvent` system
- `BallGaugeKapaow` and `BallGaugeEgg` components & movement logic
- Each pan has its own gauge that follows it
- Randomized target zones for varied gameplay
- Hit detection (Perfect/Miss) and HP deduction
- Query conflicts resolved using `ParamSet` for Transform access

### Phase 4: Recipe & Progress ✅
- Ingredient sequence tracking (8-step sequence)
- HP System (Game Over state)
- UI HUD (HP Hearts, Current Step display)

### Phase 5: Polish & Juice ✅
- Main menu with Start and Quit buttons
- Victory & Game Over screens with restart/menu navigation
- 🚧 Future additions:
  - Sound effects (Sizzling sound)
  - Visual feedback (Shake effect on miss)
  - Animations and particles
  - Timer-based gauge despawn
  - Performance optimizations
  - More visual feedback and juice

## 🔧 Troubleshooting

### Common Issues

1. **Game doesn't start**
   - Ensure you're using Rust 1.70 or later
   - Run `cargo update` to update dependencies

3. **Drag & drop not working**
   - Ensure you're in the InGame state
   - Check that ingredients are within the screen bounds
   - Verify that the Picking API is correctly initialized

4. **Query conflict errors**
   - All Query conflicts (B0001) have been resolved using `ParamSet`
   - If you encounter new Query conflicts, check for multiple Transform queries in the same system
   - Use `ParamSet` to group conflicting queries and access them sequentially

3. **Runtime panic about missing resource**
   - Ensure system ordering is correct (use `.after()` for dependencies)
   - Check that resources are initialized before use

3. **Performance issues**
   - Run in release mode: `cargo run --release`
   - Close other applications to free up resources

## 📄 License

This project is open source and available for educational purposes.

## 🙏 Acknowledgments

- Built with [Bevy Engine](https://bevyengine.org/)
- Inspired by traditional Thai cooking
- Created as an educational project for learning Bevy 0.17+ features

## 📞 Support

For issues or questions about the game:
1. Check the troubleshooting section above
2. Review the code comments in the source files
3. Refer to the [Bevy Documentation](https://docs.rs/bevy/)

---

Happy Cooking! 🍳✨
