use super::{
    animation::{Animation, AnimationState},
    physics::{BoxCollider, Velocity},
};
use crate::{constants::PHYSICS_DELTA, resources::FakeBrickAssets};
use benimator::FrameRate;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Bundle)]
pub struct FakeBrickBundle {
    pub fake_brick: FakeBrick,

    pub sprites: SpriteSheetBundle,
    pub animations: FakeBrickAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    pub collider: BoxCollider,
    pub velocity: Velocity,
}

impl FakeBrickBundle {
    pub fn new(transform: Transform, assets: &FakeBrickAssets) -> Self {
        let animations = FakeBrickAnimations::default();
        let animation = animations.idle.clone();
        FakeBrickBundle {
            fake_brick: FakeBrick {},

            sprites: SpriteSheetBundle {
                texture_atlas: assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..default()
            },
            animations,
            animation,
            animation_state: AnimationState::default(),

            collider: new_fake_brick_box_collider(),
            velocity: Velocity(Vec2 { x: 0.0, y: 1.0 }),
        }
    }
}

#[derive(Component)]
pub struct FakeBrick {}

pub const FAKE_BRICK_FLIPING_SECONDS: f64 = PHYSICS_DELTA * 30.0;

#[derive(Component)]
pub struct FakeBrickFliping {}

#[derive(Component, Clone)]
pub struct FakeBrickAnimations {
    pub idle: Animation,
    pub flip: Animation,
}

impl Default for FakeBrickAnimations {
    fn default() -> Self {
        FakeBrickAnimations {
            idle: Animation(benimator::Animation::from_indices(
                0..=0,
                FrameRate::from_total_duration(Duration::from_secs_f64(PHYSICS_DELTA * 60.0)),
            )),
            flip: Animation(
                benimator::Animation::from_indices(
                    0..=5,
                    FrameRate::from_total_duration(Duration::from_secs_f64(
                        FAKE_BRICK_FLIPING_SECONDS,
                    )),
                )
                .once(),
            ),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct FakeBrickBeforeFlipDelay(Timer);

impl Default for FakeBrickBeforeFlipDelay {
    fn default() -> Self {
        FakeBrickBeforeFlipDelay {
            0: Timer::new(
                Duration::from_secs_f64(PHYSICS_DELTA * 10.0),
                TimerMode::Once,
            ),
        }
    }
}

pub fn new_fake_brick_box_collider() -> BoxCollider {
    BoxCollider {
        size: Vec2::new(97.0, 18.0),
        center: Vec2::new(0.0, 0.0),
        ..default()
    }
}
