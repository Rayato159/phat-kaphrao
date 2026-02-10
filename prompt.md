```md /home/moo-tu/code/cat_street_food/prompt.md
# Role & Context
Act as an expert Rust Game Developer specializing in Bevy Engine 0.17+ and ECS architecture. 
You are assisting with the "Pad Kaprao" Thai cooking game, which is currently fully functional with all core mechanics implemented.

# Pre-action Requirement (MCP/Search Context)
BEFORE generating any code, please search or retrieve the latest documentation/RFCs for Bevy 0.15 through 0.17 regarding:
1. "Observers" and "Triggers" (event-driven logic).
2. "Required Components" (component composition).
3. "Bevy Picking API" (the new built-in interaction system).
4. "Computed States" and any syntax changes in the App builder for 0.17+.

# Project Specifications
- Drag & Drop: Use the latest Picking API (`On<Pointer<Drag>>`, `On<Pointer<DragStart>>`, `On<Pointer<DragEnd>>`, `On<Pointer<DragDrop>>`).
- Cooking Logic: Use 'Observers' to detect when an Ingredient (Entity) is dropped onto the Pan.
- Mini-game: Trigger a 'Timing Gauge' system specifically when "Oil" is added.
- Stats Management: Resource-based HP and Step tracking.
- Modular Design: Code is organized into modular folders (entities/, systems/, game_state/).
- System Ordering: Use `.after()` to ensure correct execution order and avoid resource conflicts.
- Query Conflicts: Use `Without<T>` on queries to prevent conflicts between mutable and immutable queries of the same component.

# Coding Standards
- Strictly adhere to the latest Bevy 0.17 syntax (Avoid deprecated 0.13/0.14 patterns).
- Use 'Required Components' (`#[require(...)]`) to minimize boilerplate.
- Use 'Observers' for decoupling the "Drop" action from the "Gauge Spawn" logic.
- Use `event.event_target()` to access the entity that triggered the event.
- Use `event.dropped` to access the entity that was dropped during drag operations.
- Use `commands.trigger()` or `world.trigger()` to fire events.

# Current Implementation Status
✅ **COMPLETED**: All core features are implemented and working:
- Bevy 0.17 project with modern ECS patterns
- Two pans (PanKapaow and PanEgg) as main characters positioned in the center
- 8 ingredients arranged in 4x2 grid on right side for easy dragging to pans (Oil, Garlic, Pork, Egg, Fish Sauce, Oyster Sauce, Thai Chilli, Holy Basil)
- Window resolution configured to 1920x1080 for optimal gameplay
- Drag & Drop system using Bevy's Picking API
- Drop detection on pans with return-to-origin logic
- Ball Gauge mini-game (BallGaugeKapaow and BallGaugeEgg) triggered when Oil is added
- Each pan has its own dedicated gauge that follows it with randomized target zones
- Strict oil rule: first two ingredients must be oil (one for Kapaow, one for Egg)
- All Query conflicts (B0001) resolved using `ParamSet` for systems with conflicting Transform queries
- 8-step recipe sequence tracking
- HP system (4 hearts) with Game Over/Victory states
- UI HUD showing HP and current step
- Win/Lose screens with restart functionality
- Main menu with Start and Quit game buttons

# Known Issues Fixed
✅ Fixed: Query conflicts in ingredient_systems.rs by adding `Without<Pan>` to mutable Transform query
✅ Fixed: Runtime panic for missing GameStats resource by using `.after()` for system ordering
✅ Fixed: All compilation errors - project builds and runs successfully
✅ Fixed: Y-drag direction issue - ingredients moving opposite to mouse direction; solved by converting screen Y coordinates to world Y coordinates (negating Y) in drag functions
✅ Fixed: Bevy 0.17 API compatibility - updated `trigger.event().target` to `trigger.entity`
✅ Fixed: Time API changes - updated `time.delta_seconds()` to `time.delta().as_secs_f32()`
✅ Fixed: Entity despawning - updated from `despawn_recursive()` to `despawn()`
✅ Fixed: Separate ball gauges for Kapaow and Egg pans with randomized target zones
✅ Fixed: Gauge following pan system implemented with GaugeFollowsPan component
✅ Fixed: Borrow conflicts in gauge animation system
✅ Fixed: Query conflict (B0001) in `on_drag_end` - used `ParamSet` to resolve multiple Transform queries
✅ Fixed: Query conflict (B0001) in `update_gauge_follow_pan` - used `ParamSet` with two-pass approach

# Task
Current task is for polish and enhancements:
- Add sound effects (sizzling, cooking sounds)
- Add visual feedback (animations, particles, shake effects)
- Add visual icons/sprites for each ingredient type
- Optimize performance and add more "juice" to the game

- Improve timing gauge visual feedback
