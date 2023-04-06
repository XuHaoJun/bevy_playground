use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub mod floor_stage;
pub mod scoreboard;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas(
        tile_size_x = 32.0,
        tile_size_y = 32.0,
        columns = 9,
        rows = 5,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "player.png")]
    pub sprite_sheet: Handle<TextureAtlas>,

    #[asset(path = "sounds/die.ogg")]
    pub die: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct NormalBrickAssets {
    #[asset(texture_atlas(
        tile_size_x = 95.0,
        tile_size_y = 16.0,
        columns = 1,
        rows = 1,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "normal.png")]
    pub sprite_sheet: Handle<TextureAtlas>,

    #[asset(path = "sounds/normal.ogg")]
    pub hit: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct NailsBrickAssets {
    #[asset(texture_atlas(
        tile_size_x = 96.0,
        tile_size_y = 31.0,
        columns = 1,
        rows = 1,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "nails.png")]
    pub sprite_sheet: Handle<TextureAtlas>,

    #[asset(path = "sounds/nail.ogg")]
    pub hit: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FakeBrickAssets {
    #[asset(texture_atlas(
        tile_size_x = 97.0,
        tile_size_y = 36.0,
        columns = 1,
        rows = 6,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "fake.png")]
    pub sprite_sheet: Handle<TextureAtlas>,

    #[asset(path = "sounds/fake.ogg")]
    pub hit: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SpringBrickAssets {
    #[asset(texture_atlas(
        tile_size_x = 97.0,
        tile_size_y = 22.0,
        columns = 1,
        rows = 6,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "spring.png")]
    pub sprite_sheet: Handle<TextureAtlas>,

    #[asset(path = "sounds/spring.ogg")]
    pub hit: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ConveyorBrickAssets {
    #[asset(texture_atlas(
        tile_size_x = 96.0,
        tile_size_y = 16.0,
        columns = 1,
        rows = 4,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "conveyor_left.png")]
    pub left_sprite_sheet: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 96.0,
        tile_size_y = 16.0,
        columns = 1,
        rows = 4,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "conveyor_right.png")]
    pub right_sprite_sheet: Handle<TextureAtlas>,

    #[asset(path = "sounds/conveyor.ogg")]
    pub hit: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct WallAssets {
    #[asset(texture_atlas(
        tile_size_x = 18.0,
        tile_size_y = 1200.0,
        columns = 1,
        rows = 1,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "wall.png")]
    pub sprite_sheet: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub medium_font: Handle<Font>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub bold_font: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct CeilingAssets {
    #[asset(texture_atlas(
        tile_size_x = 800.0,
        tile_size_y = 16.0,
        columns = 1,
        rows = 1,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "ceiling.png")]
    pub sprite_sheet: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Copy)]
pub enum InGameMode {
    #[default]
    Offline,
    Online,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct InGameSetting {
    pub mode: InGameMode,
    pub num_players: usize,
}

impl InGameSetting {
    pub fn new_offline_1p() -> Self {
        Self {
            mode: InGameMode::Offline,
            num_players: 1,
        }
    }

    pub fn new_online_2p() -> Self {
        Self {
            mode: InGameMode::Online,
            num_players: 2,
        }
    }

    pub fn set_online_2p(&mut self) {
        self.clone_from(&Self::new_online_2p());
    }

    pub fn set_offline_1p(&mut self) {
        self.clone_from(&&Self::new_offline_1p());
    }
}
