use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

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
pub struct WallAssets {
    #[asset(texture_atlas(
        tile_size_x = 18.0,
        tile_size_y = 400.0,
        columns = 1,
        rows = 1,
        padding_x = 0.0,
        padding_y = 0.0
    ))]
    #[asset(path = "wall.png")]
    pub sprite_sheet: Handle<TextureAtlas>,
}
