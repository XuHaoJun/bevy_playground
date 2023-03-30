use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        fake_brick::FakeBrickBundle,
        nails_brick::NailsBrickBundle,
        normal_brick::NormalBrickBundle,
        player::PlayerBundle,
        wall::{WallBundle, WALL_HEIGHT, WALL_WIDTH},
    },
    resources::{FakeBrickAssets, NailsBrickAssets, NormalBrickAssets, PlayerAssets, WallAssets},
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_players(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let player1_transform = Transform::from_xyz(0.0, 200.0, 2.0);
    commands.spawn(PlayerBundle::new(player1_transform, &player_assets));
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

// pub fn spawn_walls(mut commands: Commands, windows: Res<Windows>, wall_assets: Res<WallAssets>) {
//     let window = windows.primary();
//     let height = window.height();
//     let width = window.width();

//     let transform = Transform::from_xyz(
//         width / 2.0 + -(WALL_WIDTH / 2.0),
//         height / 2.0 - (WALL_HEIGHT / 2.0),
//         0.0,
//     );
//     commands.spawn(WallBundle::new(transform, &wall_assets));
// }

pub fn play_background_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let dir = "sounds/background";
    audio
        .play(asset_server.load(format!("{dir}/run_amok.ogg")))
        .with_volume(0.2)
        .looped();
}
