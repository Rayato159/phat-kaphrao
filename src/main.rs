use bevy::{prelude::*, window::WindowMode};
use bevy_spritesheet_animation::prelude::*;
use pad_kaprao::{
    animate::gauge_animate::moving_ball_gauge_animation,
    entities::check_list::spawn_checklist,
    logic::check_lose::check_game_over,
    message::{
        game_message::{GameLoseMessage, GameWinMessage},
        gaug_message::{GaugeEggHitMassage, GaugeKaprowHitMassage, GaugeSpawnMassage},
        ingredient_message::IngredientDroppedMessage,
    },
    resource::{
        cooking_animations::{EggCookingAnimations, KaprowCookingAnimations},
        cooking_state::{EggCookingState, KaprowCookingState},
        time_state::{check_game_timer, start_timer},
    },
    systems::{
        check_list_systems::update_checklist_on_drop,
        egg_cooking_systems::{handle_egg_pan_ingredient_drop, next_step_egg_cooking},
        game_end_systems::{
            cleanup_game_end_screens, handle_game_over_screen_input, handle_victory_screen_input,
            show_game_over_screen, show_victory_screen,
        },
        gauge_systems::{
            check_gauge_hit_window, despawn_target_zone_on_hit, spawn_gauge_from_event,
        },
        heart_system::{cleanup_hud, setup_heart_atlas_ui, spawn_hud_and_hearts, update_hearts_ui},
        ingredient_systems::{spawn_ingredients, update_dragging_ingredient},
        init_game_systems::{reset_game_state, setup_camera_and_scene, setup_initial_game_state},
        kapaow_cooking_systems::{handle_kaprow_pan_ingredient_drop, next_step_kaprow_cooking},
        menu_systems::{cleanup_main_menu, handle_menu_button_click, setup_main_menu},
        observer_systems::observe_game_state_changes,
        pan_systems::setup_frying_pan,
        time_count_down_systems::{spawn_countdown_timer, update_countdown_timer},
    },
    AppState, InGame, GAME_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH,
};

fn main() {
    App::new()
        .insert_resource(KaprowCookingAnimations::default())
        .insert_resource(EggCookingAnimations::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_TITLE.to_string(),
                        resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SpritesheetAnimationPlugin)
        .init_state::<AppState>()
        .init_state::<KaprowCookingState>()
        .init_state::<EggCookingState>()
        .add_computed_state::<InGame>()
        .add_message::<GaugeSpawnMassage>()
        .add_message::<GameWinMessage>()
        .add_message::<GameLoseMessage>()
        .add_message::<IngredientDroppedMessage>()
        .add_message::<GaugeKaprowHitMassage>()
        .add_message::<GaugeEggHitMassage>()
        .add_systems(
            Startup,
            (
                setup_camera_and_scene,
                setup_initial_game_state,
                spawn_ingredients.after(setup_initial_game_state),
                setup_frying_pan,
                setup_heart_atlas_ui,
            ),
        )
        .add_systems(
            OnEnter(InGame),
            (
                start_timer,
                spawn_checklist,
                spawn_countdown_timer,
                spawn_hud_and_hearts,
            ),
        )
        .add_systems(OnExit(InGame), cleanup_hud)
        .add_systems(
            Update,
            (
                handle_kaprow_pan_ingredient_drop,
                handle_egg_pan_ingredient_drop,
                spawn_gauge_from_event,
                moving_ball_gauge_animation,
                check_gauge_hit_window,
                despawn_target_zone_on_hit,
                check_game_over.after(check_gauge_hit_window),
                check_game_timer,
                update_countdown_timer,
                update_checklist_on_drop
                    .after(handle_kaprow_pan_ingredient_drop)
                    .after(handle_egg_pan_ingredient_drop),
                update_hearts_ui,
            )
                .run_if(in_state(InGame)),
        )
        .add_systems(Update, update_dragging_ingredient.run_if(in_state(InGame)))
        .add_systems(Update, next_step_kaprow_cooking.run_if(in_state(InGame)))
        .add_systems(Update, next_step_egg_cooking.run_if(in_state(InGame)))
        .add_systems(OnEnter(InGame), (reset_game_state, setup_heart_atlas_ui))
        .add_systems(OnEnter(AppState::Victory), show_victory_screen)
        .add_systems(OnEnter(AppState::GameOver), show_game_over_screen)
        .add_systems(OnExit(AppState::Victory), cleanup_game_end_screens)
        .add_systems(OnExit(AppState::GameOver), cleanup_game_end_screens)
        .add_systems(
            Update,
            handle_victory_screen_input.run_if(in_state(AppState::Victory)),
        )
        .add_systems(
            Update,
            handle_game_over_screen_input.run_if(in_state(AppState::GameOver)),
        )
        .add_systems(OnEnter(AppState::Menu), setup_main_menu)
        .add_systems(OnExit(AppState::Menu), cleanup_main_menu)
        .add_systems(
            Update,
            handle_menu_button_click.run_if(in_state(AppState::Menu)),
        )
        .add_systems(Update, observe_game_state_changes)
        .run();
}
