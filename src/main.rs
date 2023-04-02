use std::time::Duration;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResizeConstraints, WindowResolution},
};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioPlugin;

use components::player::DamagingTimer;
use constants::PHYSICS_DELTA;
use events::physics_events::{
    CollisionEvent, FakeBrickTriggerEnterEvent, SpringBrickTriggerEnterEvent, TriggerEnterEvent,
};
use resources::{
    scoreboard::{ScoreTimer, Scoreboard},
    CeilingAssets, FakeBrickAssets, NailsBrickAssets, NormalBrickAssets, PlayerAssets,
    SpringBrickAssets, UiAssets, WallAssets,
};
use systems::{
    animate_systems::animate_system,
    ceiling_systems::{celling_hurting_player_system, player_ceiling_hitbox_system},
    fake_brick_systems::{
        animate_fake_brick_system, fake_brick_flip_system, fake_brick_trigger_enter_system,
    },
    nails_brick_systems::player_nails_hitbox_system,
    physics_systems::{player_collision_system, velocity_system},
    player_systems::{
        animate_player_system, damaging_timer_system, enter_dead_system, enter_flying_system,
        enter_grounded_system, jumping_timer_system, leave_flying_system, leave_grounded_system,
        player_controller_system,
    },
    scoreboard_systems::add_score,
    spring_brick_systems::{animate_spring_brick_system, spring_brick_trigger_enter_system},
    startup_systems::{
        play_background_sound, spawn_bricks, spawn_camera, spawn_ceiling, spawn_players,
        spawn_walls,
    },
    ui::in_game_ui_systems::{update_health_text, update_score_text},
    userinput_system::userinput_system,
};

mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    AssetLoading,
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::InGame),
        )
        .add_collection_to_loading_state::<_, PlayerAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, NormalBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, FakeBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, NailsBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, WallAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, CeilingAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, SpringBrickAssets>(AppState::AssetLoading)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ns-shaft clone".to_string(),
                        resolution: WindowResolution::new(540.0, 960.0),
                        resizable: true,
                        resize_constraints: WindowResizeConstraints {
                            min_height: 0.0,
                            max_height: 960.0,
                            min_width: 540.0,
                            max_width: 540.0,
                        },
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .register_type::<DamagingTimer>()
        .add_event::<CollisionEvent>()
        .add_event::<TriggerEnterEvent>()
        .add_event::<FakeBrickTriggerEnterEvent>()
        .add_event::<SpringBrickTriggerEnterEvent>()
        .add_plugin(AudioPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .insert_resource(Scoreboard::default())
        .insert_resource(ScoreTimer::default())
        .add_systems(
            (
                play_background_sound,
                spawn_camera,
                spawn_bricks,
                spawn_players,
                spawn_walls,
                spawn_ceiling,
            )
                .in_schedule(OnEnter(AppState::InGame)),
        )
        .add_systems(
            (
                animate_system,
                animate_player_system.before(animate_system),
                animate_fake_brick_system.before(animate_system),
                animate_spring_brick_system.before(animate_system),
                add_score,
                update_score_text.after(add_score),
                update_health_text,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                userinput_system,
                velocity_system,
                fake_brick_trigger_enter_system.after(player_collision_system),
                fake_brick_flip_system,
                player_collision_system
                    .after(player_controller_system)
                    .after(velocity_system),
                player_nails_hitbox_system.after(damaging_timer_system),
                player_ceiling_hitbox_system
                    .after(damaging_timer_system)
                    .after(player_collision_system),
                celling_hurting_player_system
                    .before(player_ceiling_hitbox_system)
                    .before(player_collision_system),
                enter_dead_system
                    .after(player_nails_hitbox_system)
                    .after(player_ceiling_hitbox_system),
            )
                .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame)
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_systems(
            (
                player_controller_system,
                damaging_timer_system,
                jumping_timer_system,
                enter_grounded_system.after(player_collision_system),
                leave_grounded_system.after(player_collision_system),
                enter_flying_system.after(player_collision_system),
                leave_flying_system.after(player_collision_system),
            )
                .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame)
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_systems(
            (spring_brick_trigger_enter_system.after(player_collision_system),)
                .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame)
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new(Duration::from_secs_f64(PHYSICS_DELTA)))
        .run();
}
