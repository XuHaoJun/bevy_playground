use bevy::prelude::*;

use crate::resources::NailsBrickAssets;

use super::physics::{BoxCollider, Velocity};

#[derive(Bundle)]
pub struct NailsBrickBundle {
    pub nails_brick: NailsBrick,

    pub sprites: SpriteSheetBundle,

    pub collider: BoxCollider,
    pub velocity: Velocity,
    pub hitbox: NailsBrickHitbox,
}

impl NailsBrickBundle {
    pub fn new(transform: Transform, nails_brick_assets: &NailsBrickAssets) -> Self {
        NailsBrickBundle {
            nails_brick: NailsBrick {},

            sprites: SpriteSheetBundle {
                texture_atlas: nails_brick_assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..default()
            },

            collider: BoxCollider {
                size: Vec2::new(96.0, 16.0),
                center: Vec2::new(0.0, -15.5),
                ..default()
            },
            velocity: Velocity(Vec2 { x: 0.0, y: 1.0 }),
            hitbox: NailsBrickHitbox {
                0: BoxCollider {
                    size: Vec2::new(96.0, 16.0),
                    center: Vec2::new(0.0, 15.5),
                    ..default()
                },
            },
        }
    }
}

#[derive(Component)]
pub struct NailsBrick {}

#[derive(Component, Deref, DerefMut, Default)]
pub struct NailsBrickHitbox(pub BoxCollider);

