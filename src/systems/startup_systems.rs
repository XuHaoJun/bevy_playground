use std::ops::RangeInclusive;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        camera::MainCamera,
        ceiling::CeilingBundle,
        conveyor_brick::{ConveyorBrickBundle, ConveyorDirection},
        fake_brick::FakeBrickBundle,
        nails_brick::NailsBrickBundle,
        normal_brick::NormalBrickBundle,
        player::PlayerBundle,
        spring_brick::SpringBrickBundle,
        wall::WallBundle,
    },
    constants::{CELLING_HEIGHT, IN_GAME_UI_APP_BAR_HEIGHT, WALL_HEIGHT, WALL_WIDTH},
    resources::{
        CeilingAssets, ConveyorBrickAssets, FakeBrickAssets, NailsBrickAssets, NormalBrickAssets,
        PlayerAssets, SpringBrickAssets, UiAssets, WallAssets,
    },
};

use super::ui::in_game_ui_systems::build_in_game_ui;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
    spring_brick_assets: Res<SpringBrickAssets>,
    conveyor_brick_assets: Res<ConveyorBrickAssets>,
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

    let spring1_transform = Transform::from_xyz(-150.0, -280.0, 0.0);
    commands.spawn(SpringBrickBundle::new(
        spring1_transform,
        &spring_brick_assets,
    ));
    commands.spawn(SpringBrickBundle::new(
        Transform::from_xyz(150.0, -280.0, 0.0),
        &spring_brick_assets,
    ));
    commands.spawn(ConveyorBrickBundle::new(
        ConveyorDirection::Left,
        Transform::from_xyz(130.0, -330.0, 0.0),
        &conveyor_brick_assets,
    ));
    commands.spawn(ConveyorBrickBundle::new(
        ConveyorDirection::Right,
        Transform::from_xyz(0.0, -330.0, 0.0),
        &conveyor_brick_assets,
    ));
}

struct BrickProbability {
    all: RangeInclusive<u32>,

    normal: RangeInclusive<u32>,
    fake: RangeInclusive<u32>,
    nails: RangeInclusive<u32>,
    conveyor: RangeInclusive<u32>,
    spring: RangeInclusive<u32>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum BrickType {
    Normal,
    Fake,
    Nails,
    Conveyor,
    Spring,
}

impl BrickProbability {
    fn new() -> Self {
        Self {
            all: RangeInclusive::new(1, 100),
            normal: RangeInclusive::new(1, 50),
            fake: RangeInclusive::new(51, 60),
            nails: RangeInclusive::new(61, 80),
            conveyor: RangeInclusive::new(81, 90),
            spring: RangeInclusive::new(91, 100),
        }
    }

    fn sample(&self, rng: &mut fastrand::Rng) -> BrickType {
        let n = rng.u32(self.all.clone());
        let ranges = vec![
            (BrickType::Normal, &self.normal),
            (BrickType::Fake, &self.fake),
            (BrickType::Nails, &self.nails),
            (BrickType::Conveyor, &self.conveyor),
            (BrickType::Spring, &self.spring),
        ];
        ranges.iter().find(|x| x.1.contains(&n)).unwrap().0
    }
}

pub fn spawn_bricks_2(
    mut commands: Commands,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    normal_brick_assets: Res<NormalBrickAssets>,
    nails_brick_assets: Res<NailsBrickAssets>,
    fake_brick_assets: Res<FakeBrickAssets>,
    spring_brick_assets: Res<SpringBrickAssets>,
    conveyor_brick_assets: Res<ConveyorBrickAssets>,
) {
    let pos_rng = fastrand::Rng::with_seed(5);
    let mut brick_type_rng = fastrand::Rng::with_seed(3);
    let conveyor_dir_rng = fastrand::Rng::with_seed(4);

    let brick_prob = BrickProbability::new();
    if let Ok(window) = primary_query.get_single() {
        let height = window.height().trunc() as i32;
        let width = window.width().trunc() as i32;
        let min_x = (-1 * (width / 2)) + 50;
        let max_x = (width / 2) - 50;
        // let min_y = -1 * (height / 2);
        let max_y = height / 2;
        for i in 0..500 {
            let x = pos_rng.i32(min_x..max_x);
            let y = max_y - i * 55;
            let transform = Transform::from_xyz(x as f32, y as f32, 0.0);

            let btype = brick_prob.sample(&mut brick_type_rng);
            match btype {
                BrickType::Normal => {
                    commands.spawn(NormalBrickBundle::new(transform, &normal_brick_assets));
                }
                BrickType::Fake => {
                    commands.spawn(FakeBrickBundle::new(transform, &fake_brick_assets));
                }
                BrickType::Nails => {
                    let nail_transform = transform
                        .clone()
                        .with_translation(transform.translation + Vec3::new(0.0, 15.5, 0.0));
                    commands.spawn(NailsBrickBundle::new(nail_transform, &nails_brick_assets));
                }
                BrickType::Conveyor => {
                    let dir = if conveyor_dir_rng.bool() {
                        ConveyorDirection::Left
                    } else {
                        ConveyorDirection::Right
                    };
                    commands.spawn(ConveyorBrickBundle::new(
                        dir,
                        transform,
                        &conveyor_brick_assets,
                    ));
                }
                BrickType::Spring => {
                    commands.spawn(SpringBrickBundle::new(transform, &spring_brick_assets));
                }
            }
        }
    }
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

pub fn spawn_ceiling(
    mut commands: Commands,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    ceiling_assets: Res<CeilingAssets>,
) {
    let window = primary_query.get_single().unwrap();
    let height = window.height();
    let transform = Transform::from_xyz(
        0.0,
        (height / 2.0) - IN_GAME_UI_APP_BAR_HEIGHT - (CELLING_HEIGHT / 2.0),
        0.0,
    );
    commands.spawn(CeilingBundle::new(transform, &ceiling_assets));
}

pub fn play_background_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let dir = "sounds/background";
    audio
        .play(asset_server.load(format!("{dir}/run_amok.ogg")))
        .with_volume(0.5)
        .looped();
}
