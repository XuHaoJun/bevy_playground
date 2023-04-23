use std::time::Duration;

use bevy::{
    ecs::schedule::ScheduleLabel,
    prelude::*,
    window::{PresentMode, WindowResizeConstraints, WindowResolution},
};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::{GGRSPlugin, GGRSSchedule};
use bevy_kira_audio::AudioPlugin;

use components::{player::Health, userinput::Userinput};
use constants::{AppState, GgrsConfig, PHYSICS_DELTA};
use events::{
    physics_events::{
        CollisionEvent, ConveyorBrickTriggerEnterEvent, ConveyorBrickTriggerLeaveEvent,
        FakeBrickTriggerEnterEvent, NormalBrickTriggerEnterEvent, SpringBrickTriggerEnterEvent,
        TriggerEvent,
    },
    player_events::{PlayerEnterDeadEvent, PlayerLeaveDeadEvent},
};
use resources::{
    scoreboard::{ScoreTimer, Scoreboard},
    AppConfig, AppConfigAssets, CeilingAssets, ConveyorBrickAssets, FakeBrickAssets, InGameMode,
    InGameSetting, NailsBrickAssets, NormalBrickAssets, PlayerAssets, SpringBrickAssets, UiAssets,
    WallAssets,
};
use systems::{
    animate_systems::animate_system,
    ceiling_systems::{celling_hurting_player_system, player_ceiling_hitbox_system},
    conveyor_brick_systems::player_on_conveyor_system,
    fake_brick_systems::{
        animate_fake_brick_system, fake_brick_flip_system, fake_brick_trigger_enter_system,
    },
    in_game_once_systems::*,
    nails_brick_systems::player_nails_hitbox_system,
    network_systems::{
        close_matchbox_socket, network_input_system, start_matchbox_socket, wait_for_players,
    },
    normal_brick_systems::normal_brick_trigger_enter_system,
    physics_systems::{player_collision_system, velocity_system},
    player_systems::{
        animate_player_system, damaging_timer_system, enter_dead_system, enter_flying_system,
        enter_grounded_system, jumping_timer_system, leave_flying_system, leave_grounded_system,
        player_controller_system, player_out_window_die_system,
    },
    scoreboard_systems::{add_score, init_score},
    spring_brick_systems::{animate_spring_brick_system, spring_brick_trigger_enter_system},
    ui::{
        in_game_ui_systems::{update_health_text, update_score_text},
        main_menu_ui_systems::{
            despawn_main_menu_ui_all, interact_with_online_play_button, interact_with_quit_button,
            interact_with_single_play_button, spawn_main_menu_ui_all,
        },
        matchmaking_ui_systems::{
            despawn_matchmaking_ui_all, interact_with_back_main_menu_button,
            spawn_matchmaking_ui_all, update_matching_elapsed_time,
        },
    },
    userinput_system::userinput_system_2,
    wall_systems::wall_reset_position_system,
};

mod components;
mod constants;
mod events;
mod resources;
mod systems;
mod utils;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct OnlineSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set()]
pub struct OfflineSet;

fn main() {
    let mut app = App::new();

    GGRSPlugin::<GgrsConfig>::new()
        .with_input_system(network_input_system)
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Userinput>()
        .register_rollback_component::<Health>()
        .build(&mut app);

    app.add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, PlayerAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, NormalBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, FakeBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, NailsBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, WallAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, UiAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, CeilingAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, SpringBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, ConveyorBrickAssets>(AppState::AssetLoading)
        .add_collection_to_loading_state::<_, AppConfigAssets>(AppState::AssetLoading)
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
        .add_plugin(bevy_common_assets::toml::TomlAssetPlugin::<AppConfig>::new(
            &["app_config.toml"],
        ))
        // .register_type::<DamagingTimer>()
        .add_event::<CollisionEvent>()
        .add_event::<TriggerEvent>()
        .add_event::<NormalBrickTriggerEnterEvent>()
        .add_event::<FakeBrickTriggerEnterEvent>()
        .add_event::<SpringBrickTriggerEnterEvent>()
        .add_event::<ConveyorBrickTriggerEnterEvent>()
        .add_event::<ConveyorBrickTriggerLeaveEvent>()
        .add_event::<PlayerEnterDeadEvent>()
        .add_event::<PlayerLeaveDeadEvent>()
        .add_plugin(AudioPlugin)
        // .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .insert_resource(Scoreboard::default())
        .insert_resource(ScoreTimer::default())
        .insert_resource(InGameSetting::new_offline_1p())
        .insert_resource(FixedTime::new(Duration::from_secs_f64(PHYSICS_DELTA)))
        .add_system(spawn_main_menu_ui_all.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(despawn_main_menu_ui_all.in_schedule(OnExit(AppState::MainMenu)))
        .add_systems(
            (
                interact_with_single_play_button,
                interact_with_online_play_button,
                interact_with_quit_button,
            )
                .in_set(OnUpdate(AppState::MainMenu)),
        )
        .add_systems(
            (spawn_matchmaking_ui_all, start_matchbox_socket)
                .in_schedule(OnEnter(AppState::Matchmaking)),
        )
        .add_system(
            close_matchbox_socket
                .run_if(|next_state: Res<NextState<AppState>>| match &next_state.0 {
                    Some(state) => *state != AppState::InGame,
                    None => true,
                })
                .in_schedule(OnExit(AppState::Matchmaking)),
        )
        .add_system(despawn_matchmaking_ui_all.in_schedule(OnExit(AppState::Matchmaking)))
        .add_systems(
            (
                wait_for_players,
                update_matching_elapsed_time,
                interact_with_back_main_menu_button,
            )
                .in_set(OnUpdate(AppState::Matchmaking)),
        )
        .add_systems(
            (
                play_background_sound,
                spawn_camera,
                // spawn_bricks,
                spawn_bricks_2,
                spawn_players,
                spawn_walls,
                spawn_ceiling,
                init_score,
            )
                .in_schedule(OnEnter(AppState::InGame)),
        )
        .add_systems(
            (despawn_in_game_all, stop_background_sound).in_schedule(OnExit(AppState::InGame)),
        )
        .add_systems(
            (
                animate_system,
                animate_player_system.before(animate_system),
                animate_fake_brick_system.before(animate_system),
                animate_spring_brick_system.before(animate_system),
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                add_score,
                update_score_text.after(add_score),
                update_health_text,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                systems::ui::in_game_ui_systems::update_game_result_score_text,
                systems::ui::in_game_ui_systems::interact_play_again_button,
                systems::ui::in_game_ui_systems::interact_back_to_main_menu_button,
                systems::ui::in_game_ui_systems::spawn_in_game_result_menu_if_end,
            )
                .in_set(OnUpdate(AppState::InGame)),
        );

    // app.configure_set(
    //     OnlineSet
    //         .run_if(|in_game_setting: Res<InGameSetting>| {
    //             in_game_setting.mode == InGameMode::Online
    //         })
    //         .in_set(OnUpdate(AppState::InGame)),
    // );

    // app.configure_set(
    //     OfflineSet
    //         .run_if(|in_game_setting: Res<InGameSetting>| {
    //             in_game_setting.mode == InGameMode::Offline
    //         })
    //         .in_set(OnUpdate(AppState::InGame)),
    // );

    // add_in_game_systems(&mut app, CoreSchedule::FixedUpdate, OfflineSet);
    // add_in_game_systems(&mut app, GGRSSchedule, OnlineSet);
    add_in_game_systems(&mut app, CoreSchedule::FixedUpdate, 0);
    add_in_game_systems(&mut app, GGRSSchedule, 1);

    app.run();
}

fn add_in_game_systems(
    app: &mut App,
    schedule: impl ScheduleLabel + Clone,
    mode: u32,
    // set: impl FreeSystemSet + Clone,
) {
    let systems = vec![
        (
            // userinput_system,
            userinput_system_2,
            velocity_system,
            fake_brick_trigger_enter_system.after(player_collision_system),
            fake_brick_flip_system,
            player_collision_system.after(velocity_system),
            player_nails_hitbox_system
                .after(damaging_timer_system)
                .ambiguous_with(player_on_conveyor_system),
            player_ceiling_hitbox_system
                .after(damaging_timer_system)
                .after(player_collision_system)
                .before(player_nails_hitbox_system)
                .ambiguous_with(player_on_conveyor_system),
            celling_hurting_player_system
                .before(player_ceiling_hitbox_system)
                .before(player_collision_system),
            enter_dead_system
                .after(player_nails_hitbox_system)
                .after(player_ceiling_hitbox_system)
                .after(player_on_conveyor_system),
        )
            .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame),
        (
            player_controller_system
                .before(userinput_system_2)
                // .after(userinput_system)
                .before(player_collision_system)
                .before(velocity_system),
            damaging_timer_system,
            jumping_timer_system,
            enter_grounded_system
                .after(player_collision_system)
                .after(player_ceiling_hitbox_system),
            leave_grounded_system
                .after(player_collision_system)
                .after(player_ceiling_hitbox_system),
            enter_flying_system
                .after(player_collision_system)
                .after(player_ceiling_hitbox_system),
            leave_flying_system
                .after(player_collision_system)
                .after(player_ceiling_hitbox_system),
        )
            .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame),
        (
            normal_brick_trigger_enter_system
                .after(player_collision_system)
                .before(player_nails_hitbox_system)
                .before(player_ceiling_hitbox_system)
                .ambiguous_with(player_on_conveyor_system),
            spring_brick_trigger_enter_system
                .after(player_collision_system)
                .before(player_nails_hitbox_system)
                .before(normal_brick_trigger_enter_system)
                .before(player_ceiling_hitbox_system)
                .ambiguous_with(player_on_conveyor_system),
            player_on_conveyor_system.after(player_collision_system),
            wall_reset_position_system.ambiguous_with_all(),
            player_out_window_die_system
                .before(enter_dead_system)
                .ambiguous_with_all(),
        )
            .distributive_run_if(|state: Res<State<AppState>>| state.0 == AppState::InGame),
    ];
    for x in systems {
        // app.add_systems(x.in_set(set.clone()).in_schedule(schedule.clone()));
        if mode == 0 {
            app.add_systems(
                x.distributive_run_if(|in_game_setting: Res<InGameSetting>| {
                    in_game_setting.mode == InGameMode::Offline
                })
                .in_schedule(schedule.clone()),
            );
        } else {
            app.add_systems(
                x.distributive_run_if(|in_game_setting: Res<InGameSetting>| {
                    in_game_setting.mode == InGameMode::Online
                })
                .in_schedule(schedule.clone()),
            );
        }
    }
}
