use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        fake_brick::FakeBrickBundle,
        nails_brick::NailsBrickBundle,
        normal_brick::NormalBrickBundle,
        player::{Player, PlayerBundle},
        wall::{WallBundle, WALL_HEIGHT, WALL_WIDTH},
    },
    resources::{
        FakeBrickAssets, NailsBrickAssets, NormalBrickAssets, PlayerAssets, UiAssets, WallAssets,
    }, constants::IN_GAME_UI_APP_BAR_HEIGHT,
};

use super::ui::in_game_ui_systems::build_in_game_ui;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_players(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    ui_assets: Res<UiAssets>,
) {
    let player1_transform = Transform::from_xyz(0.0, 200.0, 2.0);
    commands.spawn(PlayerBundle::new(0, player1_transform, &player_assets));
    build_in_game_ui(&mut commands, &ui_assets, vec![0]);
}

pub fn spawn_bricks(
    mut commands: Commands,
    normal_brick_assets: Res<NormalBrickAssets>,
    nails_brick_assets: Res<NailsBrickAssets>,
    fake_brick_assets: Res<FakeBrickAssets>,
) {
    let normal1_transform = Transform::from_xyz(0.0, -100.0, 0.0);
    commands.spawn(NormalBrickBundle::new(
        normal1_transform,
        &normal_brick_assets,
    ));

    let nails1_transform = Transform::from_xyz(100.0, -200.0, 0.0);
    commands.spawn(NailsBrickBundle::new(nails1_transform, &nails_brick_assets));

    let fake1_transform = Transform::from_xyz(-100.0, -200.0, 0.0);
    commands.spawn(FakeBrickBundle::new(fake1_transform, &fake_brick_assets));
}

pub fn spawn_walls(
    mut commands: Commands,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    wall_assets: Res<WallAssets>,
) {
    if let Ok(window) = primary_query.get_single() {
        let height = window.height();
        let width = window.width();
        let right_transform = Transform::from_xyz(
            width / 2.0 - (WALL_WIDTH / 2.0),
            height / 2.0 - (WALL_HEIGHT / 2.0) - IN_GAME_UI_APP_BAR_HEIGHT,
            0.0,
        );
        commands.spawn(WallBundle::new(right_transform, &wall_assets));
        let left_transform = Transform::from_xyz(
            -1.0 * right_transform.translation.x,
             right_transform.translation.y,
            0.0,
        );
        commands.spawn(WallBundle::new(left_transform, &wall_assets));
    }
}

pub fn play_background_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let dir = "sounds/background";
    audio
        .play(asset_server.load(format!("{dir}/run_amok.ogg")))
        .with_volume(0.2)
        .looped();
}
