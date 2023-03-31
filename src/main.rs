use bevy::{
    prelude::*,
    window::{PresentMode, WindowResizeConstraints},
};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioPlugin;

use components::player::DamagingTimer;
use constants::PHYSICS_DELTA;
use events::physics_events::{CollisionEvent, FakeBrickTriggerEnterEvent, TriggerEnterEvent};
use resources::{
    scoreboard::{ScoreTimer, Scoreboard},
    FakeBrickAssets, NailsBrickAssets, NormalBrickAssets, PlayerAssets, WallAssets,
};
use systems::{
    animate_systems::animate_system,
    fake_brick_systems::{
        animate_fake_brick_system, fake_brick_flip_system, fake_brick_trigger_enter_system,
    },
    nails_brick_systems::player_nails_hitbox_system,
    physics_systems::{player_collision_system, velocity_system},
    player_systems::{
        animate_player_system, damaging_timer_system, enter_dead_system, enter_flying_system,
        enter_grounded_system, leave_flying_system, leave_grounded_system,
        player_controller_system,
    },
    scoreboard_systems::add_score,
    startup_systems::{play_background_sound, spawn_bricks, spawn_camera, spawn_players},
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
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ns-shaft clone".to_string(),
                        resolution: (720., 1280.).into(),
                        resizable: true,
                        resize_constraints: WindowResizeConstraints {
                            min_height: 0.0,
                            max_height: 1280.0,
                            min_width: 720.0,
                            max_width: 720.0,
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
        .add_plugin(AudioPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .insert_resource(Scoreboard::default())
        .insert_resource(ScoreTimer::default())
        .add_systems(
            (
                play_background_sound,
                spawn_camera,
                spawn_bricks.before(spawn_players),
                spawn_players,
            )
                .in_schedule(OnEnter(AppState::InGame)),
        )
        .add_systems(
            (
                animate_system,
                animate_player_system.before(animate_system),
                animate_fake_brick_system.before(animate_system),
                add_score,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                userinput_system,
                player_controller_system,
                velocity_system,
                enter_grounded_system.after(player_collision_system),
                leave_grounded_system.after(player_collision_system),
                enter_flying_system.after(player_collision_system),
                leave_flying_system.after(player_collision_system),
                player_collision_system
                    .after(player_controller_system)
                    .after(velocity_system),
                fake_brick_trigger_enter_system.after(player_collision_system),
                fake_brick_flip_system,
                damaging_timer_system,
                player_nails_hitbox_system.after(damaging_timer_system),
                enter_dead_system.after(player_nails_hitbox_system),
            )
                .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame)
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(PHYSICS_DELTA as f32))
        .run();
}
