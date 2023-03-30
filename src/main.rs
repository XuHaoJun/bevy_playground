use bevy::{
    prelude::*,
    time::FixedTimestep,
    window::{PresentMode, WindowResizeConstraints},
};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioPlugin;
use components::player::DamagingTimer;
use constants::PHYSICS_DELTA;
use events::physics_events::{CollisionEvent, FakeBrickTriggerEnterEvent, TriggerEnterEvent};
use resources::{FakeBrickAssets, NailsBrickAssets, NormalBrickAssets, PlayerAssets, WallAssets};
use systems::{
    animate_systems::animate_system,
    fake_brick_systems::{
        animate_fake_brick_system, fake_brick_flip_system, fake_brick_trigger_enter_system,
    },
    nails_brick_systems::player_nails_hitbox_system,
    physics_systems::{player_collision_system, velocity_system},
    player_systems::{
        animate_player_system, damaging_timer_system, enter_dead_system, enter_flying_system,
        enter_grounded_system, leave_flying_system, player_controller_system,
    },
    startup_systems::{play_background_sound, spawn_bricks, spawn_camera, spawn_players, spawn_walls},
    userinput_system::userinput_system,
};

mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}

fn main() {
    App::new()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Next)
                .with_collection::<PlayerAssets>()
                .with_collection::<NormalBrickAssets>()
                .with_collection::<FakeBrickAssets>()
                .with_collection::<NailsBrickAssets>()
                .with_collection::<WallAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "ns-shaft clone".to_string(),
                        width: 720.,
                        height: 1280.,
                        resizable: true,
                        resize_constraints: WindowResizeConstraints {
                            min_height: 0.0,
                            max_height: 1280.0,
                            min_width: 720.0,
                            max_width: 720.0,
                        },
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    },
                    ..default()
                }),
        )
        .register_type::<DamagingTimer>()
        .add_event::<CollisionEvent>()
        .add_event::<TriggerEnterEvent>()
        .add_event::<FakeBrickTriggerEnterEvent>()
        .add_plugin(AudioPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
        .add_system_set(
            SystemSet::on_enter(GameState::Next)
                .with_system(play_background_sound)
                .with_system(spawn_camera)
                .with_system(spawn_bricks.before(spawn_players))
                .with_system(spawn_walls.before(spawn_players))
                .with_system(spawn_players),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(animate_system)
                .with_system(animate_player_system.before(animate_system))
                .with_system(animate_fake_brick_system.before(animate_system)),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_run_criteria(FixedTimestep::step(PHYSICS_DELTA))
                .with_system(userinput_system)
                .with_system(player_controller_system)
                .with_system(velocity_system)
                .with_system(enter_grounded_system.after(player_collision_system))
                .with_system(enter_grounded_system.after(player_collision_system))
                .with_system(enter_flying_system.after(player_collision_system))
                .with_system(leave_flying_system.after(player_collision_system))
                .with_system(
                    player_collision_system
                        .after(player_controller_system)
                        .after(velocity_system),
                )
                .with_system(fake_brick_trigger_enter_system.after(player_collision_system))
                .with_system(fake_brick_flip_system)
                .with_system(damaging_timer_system)
                .with_system(player_nails_hitbox_system.after(damaging_timer_system))
                .with_system(enter_dead_system.after(player_nails_hitbox_system)),
        )
        .run();
}
