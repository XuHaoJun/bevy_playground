use bevy::prelude::*;

use crate::resources::NormalBrickAssets;

use super::physics::{BoxCollider, Velocity};

#[derive(Bundle)]
pub struct NormalBrickBundle {
    pub normal_brick: NormalBrick,

    pub sprites: SpriteSheetBundle,

    pub collider: BoxCollider,
    pub velocity: Velocity,
}

impl NormalBrickBundle {
    pub fn new(transform: Transform, normal_brick_assets: &NormalBrickAssets) -> NormalBrickBundle {
        NormalBrickBundle {
            normal_brick: NormalBrick {},

            sprites: SpriteSheetBundle {
                texture_atlas: normal_brick_assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..Default::default()
            },

            collider: BoxCollider {
                size: Vec2::new(95.0, 16.0),
                ..default()
            },
            velocity: Velocity(Vec2 { x: 0.0, y: 1.0 }),
        }
    }
}

#[derive(Component)]
pub struct NormalBrick;
