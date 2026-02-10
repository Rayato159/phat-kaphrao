# 🦀 Advanced Skill: Bevy 0.17+ & Rust Game Dev

This document outlines the modern ECS patterns used in the "Pad Kaprao" project, focusing on features available in Bevy 0.17.

## 1. Modern Interaction: Bevy Picking API

Instead of manually calculating mouse-to-world coordinates, we use the built-in `bevy_picking` logic with Observers.

### Pointer Events
Bevy provides a comprehensive set of pointer events for interaction:
- **Movement**: `Pointer<DragStart>`, `Pointer<Drag>`, `Pointer<DragEnd>`
- **Drop Detection**: `Pointer<DragEnter>`, `Pointer<DragOver>`, `Pointer<DragDrop>`, `Pointer<DragLeave>`
- **Hover**: `Pointer<Over>`, `Pointer<Out>`

### Coordinate System Considerations
When implementing drag & drop, it's important to understand the difference between coordinate systems:

- **World Coordinates**: Bevy's default coordinate system where **Y increases UP** and the origin is at center (0, 0, 0)
- **Screen/Window Coordinates**: Pointer events typically use coordinates where **Y increases DOWN** and the origin is at top-left

When using pointer events for dragging, you need to convert from screen coordinates to world coordinates by negating the Y value:

```rust
// In on_drag_start and update_dragging_ingredient functions
let pointer_pos = Vec2::new(
    event.pointer_location.position.x,
    -event.pointer_location.position.y,  // Negate Y to convert to world coordinates
);
```

This ensures that when users drag ingredients visually up/down, the movement is correctly reflected in the game world.

### Example: Drag & Drop Implementation
```rust
// Make an entity draggable
commands.spawn((
    Ingredient,
    Pickable::default(),
))
.observe(on_drag_start)
.observe(on_drag_end);
```

### Drop Area Detection
/// Note: Actual implementation uses two specific pans (PanKapaow and PanEgg) positioned in center
```rust
// In the actual game: Two pans (PanKapaow and PanEgg) are used as main characters
commands.spawn((DropArea, PanKapaow))
    .observe(|event: On<Pointer<DragEnter>>, mut commands: Commands| {
        // Show visual feedback when ingredient enters drop area
    })
    .observe(|event: On<Pointer<DragDrop>>, mut commands: Commands| {
```
        // Handle successful drop on pan
        commands.spawn(CookingIngredient { 
            ingredient: event.dropped 
        });
    });
```

## 2. Observers & Triggers (Available in 0.17+)

Avoid "Frame-polling" (checking every frame). Use **Observers** to react instantly to events.

### Defining Events (Bevy 0.17+ Message System)
```rust
#[derive(Message)]
struct IngredientDroppedEvent {
    ingredient_entity: Entity,
    ingredient_type: IngredientType,
    drop_position: Vec3,
}

#[derive(Message)]
struct GaugeSpawnEvent {
    position: Vec2,
}
```

### Adding Observers
```rust
// Observer on specific entity
.on_drag_end
.observe(on_drag_end);
```
// Observer on specific entity
// In the actual game: Two pans (PanKapaow and PanEgg) are used as main characters
let pan_kapaow = commands.spawn(PanKapaow).id();
commands.entity(pan_kapaow).observe(|event: On<IngredientAddedToPan>, mut commands: Commands| {
    // Handle ingredient addition to this specific pan
});
```

### Triggering Events (MessageWriter)
```rust
// Trigger using MessageWriter
fn handle_ingredient_drop(
    mut event_writer: MessageWriter<IngredientDroppedEvent>,
    // ... other parameters
) {
    event_writer.write(IngredientDroppedEvent {
        ingredient_entity: entity,
        ingredient_type: ingredient.ingredient_type,
        drop_position: drop_world_pos,
    });
}
```

### Accessing Event Data in Observers
```rust
pub fn on_drag_end(
    trigger: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut event_writer: MessageWriter<IngredientDroppedEvent>,
    q_ingredients: Query<(&Ingredient, &OriginalPosition)>,
    mut q_transform: Query<&mut Transform, Without<Pan>>,
    q_pan: Query<&Transform, With<Pan>>,
) {
    let entity = trigger.target();
    let event = trigger.event();
    let drop_position = event.pointer_location.position;
    // ... handle drop logic
}
```

### System Ordering (Preventing Query Conflicts)
```rust
// Use .after() to ensure resources are initialized before use
.add_systems(
    Startup,
    (
        setup_camera_and_scene,
        setup_initial_game_state,
        spawn_ingredients.after(setup_initial_game_state),
        setup_frying_pan,
        // ...
    ),
)

// Use Without<T> to prevent query conflicts
pub fn on_drag_end(
    mut q_transform: Query<&mut Transform, Without<Pan>>,
    q_pan: Query<&Transform, With<Pan>>,
) {
    // These queries are now guaranteed to be disjoint
}
```

## 3. Required Components (Composition)

Bevy 0.17 style uses **Required Components** to ensure data integrity and minimize boilerplate.

### Basic Required Components
```rust
#[derive(Component)]
#[require(Transform, Visibility)]
struct Ingredient {
    ingredient_type: IngredientType,
}
```

### Advanced Required Component Syntax
```rust
#[derive(Component)]
#[require(
    // Default constructor
    A,
    // Tuple struct with value
    B(1),
    // Named struct
    C {
        x: 1.0,
        y: 0.0,
        ..default()
    },
    // Unit struct or variant
    D::Active,
    // Associated constant
    E::DEFAULT,
    // Constructor function
    F::new(100.0),
    // Arbitrary expression
    G = calculate_default()
)]
struct CookingStation {
    current_heat: f32,
}
```

### Recursive Required Components
```rust
/// Note: Actual implementation uses PanKapaow and PanEgg components
/// Example: Two pans as main characters in the center
#[derive(Component)]
#[require(HeatSource)]
struct PanKapaow {
    capacity: usize,
}

#[derive(Component, Default)]
#[require(Transform)]
struct HeatSource {
    temperature: f32,
}

// When spawning PanKapaow, HeatSource and Transform are automatically inserted
commands.spawn(PanKapaow { capacity: 4 });
// Implicitly also has HeatSource::default() and Transform::default()

// Similarly for PanEgg:
commands.spawn(PanEgg);
// In the actual game: Two pans (PanKapaow and PanEgg) are spawned as main characters in center
```

### Custom Initialization
```rust
#[derive(Component, Debug)]
#[require(Position = Position::center())]
struct Player;

fn center() -> Position {
    Position { x: 0.0, y: 0.0 }
}

#[derive(Component)]
#[require(Health = Health::max())]
struct Boss;
```

## 4. Computed States

Manage game flow using **Computed States** to automatically derive game states from other states.

### Defining Base States
```rust
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    InGame { paused: bool },
    GameOver { won: bool },
}
```

### Creating Computed States
```rust
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct InGame;

impl ComputedStates for InGame {
    // Define which states this depends on
    type SourceStates = AppState;

    // Compute the derived state
    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::InGame { .. } => Some(InGame),
            _ => None, // Remove state if not in game
        }
    }
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGame = InGame)]
enum GamePhase {
    #[default]
    Preparation,
    Cooking,
    Serving,
    Finished,
}
```

### Optional Source States
```rust
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct CookingActive;

impl ComputedStates for CookingActive {
    // Use Option to handle states that may not exist
    type SourceStates = Option<AppState>;

    fn compute(sources: Option<AppState>) -> Option<Self> {
        match sources {
            Some(AppState::InGame { paused: false }) => Some(CookingActive),
            _ => None,
        }
    }
}
```

### Multiple Source States
```rust
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct MiniGameAvailable;

impl ComputedStates for MiniGameAvailable {
    // Depend on multiple states
    type SourceStates = (AppState, GameSettings);

    fn compute(sources: (AppState, GameSettings)) -> Option<Self> {
        let (app_state, settings) = sources;
        match (app_state, settings.mini_games_enabled) {
            (AppState::InGame { .. }, true) => Some(MiniGameAvailable),
            _ => None,
        }
    }
}
```

### Registering in App
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_computed_state::<InGame>()
        .add_sub_state::<GamePhase>()
        .add_computed_state::<CookingActive>()
        .add_systems(Update, cooking_system.run_if(in_state(CookingActive)))
        .run();
}
```

### Using Computed States in Systems
```rust
fn cooking_system(
    mut phase: ResMut<NextState<GamePhase>>,
    ingredients: Query<&IngredientType>,
) {
    // Only runs when CookingActive state exists
    if ingredients.iter().count() > 0 {
        phase.set(GamePhase::Cooking);
    }
}
```

## 5. Optimized UI with Nodes

### Grid Layout for Ingredients (4x2)
/// Optimized for 1920x1080 window resolution
/// Ingredients positioned on right side for easy dragging to the two center pans (Kapaow and Egg)
/// 
/// Layout:
/// - Window: 1920x1080
/// - Two pans (PanKapaow, PanEgg) positioned in center as main characters
/// - 8 ingredients in 4x2 grid (4 rows, 2 columns) on right side
```rust
commands.spawn((
    Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::fr(2, 1.0),
        gap: Size::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(20.0)),
        width: Val::Percent(100.0),
        height: Val::Auto,
        ..default()
    },
    Pickable::IGNORE,
    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
))
.with_children(|parent| {
    for (index, ingredient) in ingredients.iter().enumerate() {
        let row = index / 2;
        let col = index % 2;
        
        parent.spawn((
            Node {
                width: Val::Px(80.0),
                height: Val::Px(80.0),
                grid_row: GridPlacement::start(row + 1),
                grid_column: GridPlacement::start(col + 1),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
            BackgroundColor(ingredient.color),
            Ingredient,
            ingredient.ingredient_type,
        ))
        .observe(drag_handler);
    }
});
```

### Z-Index Management
```rust
commands.spawn((
    Node {
        width: Val::Percent(100.0),
        height: Val::Auto,
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    },
    GlobalZIndex(i32::MAX), // HUD always on top
    HUD,
))
.with_children(|parent| {
    // HP Hearts
    for i in 0..max_hp {
        parent.spawn((
            Node {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                margin: UiRect::right(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            HeartIcon { index: i },
        ));
    }
});
```

## 6. UI Button Interactions & State Transitions

Bevy's UI system provides built-in button components for handling user interactions, which work seamlessly with the state management system.

### Button Component & Interaction Events
```rust
// Spawn a button with ButtonAction component
commands.spawn((
    Button,
    ButtonAction::StartGame,
    Node {
        width: Val::Px(250.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(Color::srgb(0.3, 0.7, 0.3)),
    BorderRadius::all(Val::Px(8.0)),
))
.with_children(|parent| {
    parent.spawn((
        Text::new("Start Game"),
        TextFont { font_size: 28.0, ..default() },
        TextColor(Color::WHITE),
    ));
});
```

### Button Action Component
```rust
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonAction {
    StartGame,
    QuitGame,
    Restart,
    ReturnToMenu,
}
```

### Handling Button Clicks
```rust
pub fn handle_menu_button_click(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::StartGame => {
                    info!("Start Game button clicked!");
                    next_state.set(AppState::InGame);
                }
                ButtonAction::QuitGame => {
                    info!("Quit Game button clicked!");
                    // Handle quit logic
                }
                ButtonAction::ReturnToMenu => {
                    next_state.set(AppState::Menu);
                }
            }
        }
    }
}
```

### State-Based UI Management
```rust
// Setup UI when entering a state
.add_systems(OnEnter(AppState::Menu), setup_main_menu)

// Cleanup UI when exiting a state
.add_systems(OnExit(AppState::Menu), cleanup_main_menu)

// Handle interactions only when in specific state
.add_systems(
    Update,
    handle_menu_button_click.run_if(in_state(AppState::Menu)),
)
```

### Main Menu with Multiple Buttons
```rust
pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.95, 0.92, 0.88)),
            ZIndex(150),
        ))
        .with_children(|parent| {
            // Game title
            parent.spawn((
                Text::new("🍳 Pad Kaprao 🍳"),
                TextFont { font_size: 64.0, ..default() },
                TextColor(Color::srgb(0.8, 0.6, 0.2)),
            ));

            // Start Game button
            spawn_menu_button(parent, "Start Game", 
                Color::srgb(0.3, 0.7, 0.3), ButtonAction::StartGame);

            // Quit Game button
            spawn_menu_button(parent, "Quit Game", 
                Color::srgb(0.8, 0.3, 0.3), ButtonAction::QuitGame);
        });
}

fn spawn_menu_button(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    color: Color,
    action: ButtonAction,
) {
    parent
        .spawn((
            Name::new(format!("Button-{:?}", action)),
            Button,
            action,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}
```

### Cleanup UI on State Exit
```rust
pub fn cleanup_main_menu(
    mut commands: Commands, 
    q_menu: Query<Entity, With<MainMenu>>
) {
    for entity in q_menu.iter() {
        commands.entity(entity).despawn();
    }
    info!("Cleaned up main menu");
}
```

### Marker Components for UI Elements
```rust
#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct GameEndScreen;

#[derive(Component)]
pub struct HUD;
```

These marker components help organize and query UI elements for cleanup and interaction handling.

## 7. Efficient Memory (Rust Specifics)

### Zero-cost Abstractions
```rust
// Efficient filtering without overhead
fn update_ingredients(
    mut ingredients: Query<&mut Transform, With<Ingredient>>,
    time: Res<Time>,
) {
    for mut transform in ingredients.iter_mut() {
        // Only iterate over Ingredient entities
        transform.translation.y += 100.0 * time.delta_secs();
    }
}

// Filter by multiple components
fn cooking_ingredients(
    query: Query<(&IngredientType, &Transform), (With<Ingredient>, Without<Burned>)>,
) {
    for (ingredient_type, transform) in query.iter() {
        // Only ingredients that aren't burned
    }
}
```

### Resource Management
```rust
// Efficient asset handling
fn spawn_sprite(
    commands: &mut Commands,
    assets: &Res<Assets<Image>>,
    texture_handle: Handle<Image>,
) {
    // Check if asset is loaded
    if let Some(_texture) = assets.get(&texture_handle) {
        commands.spawn(SpriteBundle {
            texture: texture_handle,
            ..default()
        });
    }
}

// Asset event handling
fn asset_loaded(
    mut events: EventReader<AssetEvent<Image>>,
    assets: Res<Assets<Image>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Added { id } => {
                if let Some(image) = assets.get(*id) {
                    println!("Image loaded: {:?}", image.size());
                }
            }
            AssetEvent::Removed { id } => {
                println!("Image removed");
            }
            _ => {}
        }
    }
}
```

## 8. Advanced Observer Patterns

### Entity-Specific Observers
```rust
// In the actual game: Two pans (PanKapaow and PanEgg) are used as main characters
let pan_entity = commands.spawn(PanKapaow).id();

// Observer only fires for this specific pan
commands.entity(pan_entity).observe(
    |event: Trigger<IngredientAdded>, mut commands: Commands| {
        commands.spawn(SizzleEffect::new(event.target));
    }
);
```

### Conditional Event Propagation
```rust
.observe(|mut event: On<Pointer<DragStart>>, mut query: Query<&mut Outline>| {
    if let Ok(mut outline) = query.get_mut(event.event_target()) {
        outline.color = Color::WHITE;
    }
    // Stop event from propagating to parents
    event.propagate(false);
})
```

### Observer System Parameters
```rust
world.add_observer(
    |event: Trigger<IngredientDropped>,
     mut commands: Commands,
     ingredients: Query<&IngredientType>,
     pan_query: Query<&PanKapaow>| {
        
        // Access multiple system parameters
        let ingredient_type = ingredients.get(event.ingredient).unwrap();
        let pan = pan_query.get(event.pan).unwrap();
        // In the actual game: Two pans (PanKapaow and PanEgg) are used as main characters
        
        // Complex logic combining event data and world state
        if pan.current_heat < ingredient_type.required_heat {
            commands.trigger(CookingFailed {
                reason: "Too cold".to_string(),
            });
        }
    }
);
```

## 9. Bevy 0.17 App Builder Patterns

### App Builder (Modular Systems)
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_computed_state::<InGame>()
        // Register events using add_message for Bevy 0.17+
        .add_message::<GaugeSpawnEvent>()
        .add_message::<GameWinEvent>()
        .add_message::<GameLoseEvent>()
        .add_message::<StepCompletedEvent>()
        .add_message::<IngredientDroppedEvent>()
        // Add systems with proper ordering
        .add_systems(Startup, setup_initial_game_state)
        .add_systems(Startup, spawn_ingredients.after(setup_initial_game_state))
        .add_systems(Update, handle_ingredient_drop.run_if(in_state(InGame)))
        .run();
}
```

### Schedule Configuration
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }
        ))
        .add_plugins(PadKapraoGamePlugin)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .run();
}
```

## 10. Debugging & Testing

### Observer Debugging
```rust
#[derive(Component)]
#[require(DebugLabel)]
struct DebugLabel {
    label: String,
}

world.add_observer(
    |event: Trigger<AnyEvent>,
     query: Query<&DebugLabel>| {
        
        if let Ok(label) = query.get(event.target) {
            println!("Event on {}: {:?}", label.label, event.event);
        }
    }
);
```

### State Transitions
```rust
fn log_state_transitions(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(new_state) = next_state.0.take() {
        println!("State transition: {:?} -> {:?}", state.get(), new_state);
    }
}