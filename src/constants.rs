use bevy::prelude::*;
use bevy_ggrs::ggrs;
use bevy_matchbox::prelude::PeerId;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BrickType {
    Normal,
    Fake,
    Nails,
    Conveyor,
    Spring,
}

pub const PHYSICS_DELTA: f64 = 1.0 / 60.0;

pub const IN_GAME_UI_APP_BAR_HEIGHT: f32 = 32.0;

pub const WALL_WIDTH: f32 = 18.0;
pub const WALL_HEIGHT: f32 = 960.0 - IN_GAME_UI_APP_BAR_HEIGHT;

pub const CELLING_WIDTH: f32 = 540.0 - (WALL_WIDTH * 2.0);
pub const CELLING_HEIGHT: f32 = 16.0;

pub const INPUT_LEFT: u8 = 1 << 0;
pub const INPUT_RIGHT: u8 = 1 << 1;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    AssetLoading,
    MainMenu,
    Matchmaking,
    InGame,
}

pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are called `PeerId`s
    type Address = PeerId;
}
