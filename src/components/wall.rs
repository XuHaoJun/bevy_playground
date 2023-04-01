use bevy::prelude::*;

use crate::{
    constants::{WALL_HEIGHT, WALL_WIDTH},
    resources::WallAssets,
};

use super::physics::BoxCollider;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,

    pub sprites: SpriteSheetBundle,

    pub collider: BoxCollider,
}

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
