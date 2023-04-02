use std::time::Duration;

use benimator::FrameRate;
use bevy::prelude::*;

use crate::{constants::PHYSICS_DELTA, resources::SpringBrickAssets};

use super::{
    animation::{Animation, AnimationState},
    physics::*,
};

#[derive(Bundle)]
pub struct SpringBrickBundle {
    pub spring_brick: SpringBrick,

    pub sprites: SpriteSheetBundle,
    pub animations: SpringBrickAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    pub collider: BoxCollider,
    pub velocity: Velocity,
}

impl SpringBrickBundle {
    pub fn new(transform: Transform, spring_brick_assets: &SpringBrickAssets) -> SpringBrickBundle {
        let animations = SpringBrickAnimations::default();
        let animation = animations.idle.clone();
        SpringBrickBundle {
            spring_brick: SpringBrick {},

            sprites: SpriteSheetBundle {
                texture_atlas: spring_brick_assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..Default::default()
            },
            animations,
            animation,
            animation_state: AnimationState::default(),

            collider: BoxCollider {
                size: Vec2::new(97.0, 22.0),
                ..default()
            },
            velocity: Velocity(Vec2 { x: 0.0, y: 1.0 }),
        }
    }
}

#[derive(Component)]
pub struct SpringBrick {}

#[derive(Component)]
pub struct SpringBrickAnimations {
    pub idle: Animation,
    pub spring: Animation,
}

#[derive(Component)]
pub struct SpringBrickSpring {}

const SPRING_DURTION: f64 = PHYSICS_DELTA * 20.0;

impl Default for SpringBrickAnimations {
    fn default() -> Self {
        Self {
            idle: Animation(benimator::Animation::from_indices(
                0..=0,
                FrameRate::from_total_duration(Duration::from_secs_f64(PHYSICS_DELTA * 60.0)),
            )),
            spring: Animation(
                benimator::Animation::from_indices(
                    0..=5,
                    FrameRate::from_total_duration(Duration::from_secs_f64(SPRING_DURTION)),
                )
                .once(),
            ),
        }
    }
}
