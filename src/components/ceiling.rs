use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::{CELLING_HEIGHT, CELLING_WIDTH, PHYSICS_DELTA},
    resources::CeilingAssets,
};

use super::physics::{BoxCollider, Velocity};

#[derive(Bundle)]
pub struct CeilingBundle {
    pub ceiling: Ceiling,

    pub sprites: SpriteSheetBundle,

    pub hitbox: CeilingHitbox,
}

impl CeilingBundle {
    pub fn new(transform: Transform, assets: &CeilingAssets) -> Self {
        Self {
            ceiling: Ceiling {},

            sprites: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(CELLING_WIDTH, CELLING_HEIGHT)),
                    ..default()
                },
                texture_atlas: assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..default()
            },

            hitbox: CeilingHitbox(BoxCollider {
                size: Vec2::new(CELLING_WIDTH, CELLING_HEIGHT),
                ..default()
            }),
        }
    }
}

#[derive(Component)]
pub struct Ceiling {}

#[derive(Component)]
pub struct CeilingHurting;

#[derive(Component, Deref, DerefMut)]
pub struct CeilingHurtingTimer(pub Timer);

impl Default for CeilingHurtingTimer {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f64(PHYSICS_DELTA * 8.0),
            TimerMode::Once,
        ))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct CeilingHitbox(pub BoxCollider);
