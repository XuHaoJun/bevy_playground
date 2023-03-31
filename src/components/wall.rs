use bevy::prelude::*;

use crate::{resources::WallAssets, constants::IN_GAME_UI_APP_BAR_HEIGHT};

use super::physics::BoxCollider;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,

    pub sprites: SpriteSheetBundle,

    pub collider: BoxCollider,
}

pub const WALL_WIDTH: f32 = 18.0;
pub const WALL_HEIGHT: f32 = 960.0 - IN_GAME_UI_APP_BAR_HEIGHT;

impl WallBundle {
    pub fn new(transform: Transform, assets: &WallAssets) -> WallBundle {
        WallBundle {
            wall: Wall {},

            sprites: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(WALL_WIDTH, WALL_HEIGHT)),
                    ..default()
                },
                texture_atlas: assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..default()
            },

            collider: BoxCollider {
                size: Vec2::new(WALL_WIDTH, WALL_HEIGHT),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct Wall {}
