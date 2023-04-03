use std::time::Duration;

use benimator::FrameRate;
use bevy::prelude::*;

use crate::{constants::PHYSICS_DELTA, resources::ConveyorBrickAssets};

use super::{
    animation::{Animation, AnimationState},
    physics::*,
};

#[derive(Bundle)]
pub struct ConveyorBrickBundle {
    pub conveyor_brick: ConveyorBrick,

    pub sprites: SpriteSheetBundle,
    pub animations: ConveyorBrickAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    pub collider: BoxCollider,
    pub velocity: Velocity,
}

impl ConveyorBrickBundle {
    pub fn new(
        direction: ConveyorDirection,
        transform: Transform,
        conveyor_brick_assets: &ConveyorBrickAssets,
    ) -> Self {
        let animations = ConveyorBrickAnimations::default();
        let animation = animations.working.clone();
        Self {
            conveyor_brick: ConveyorBrick { direction },

            sprites: SpriteSheetBundle {
                texture_atlas: match direction {
                    ConveyorDirection::Left => conveyor_brick_assets.left_sprite_sheet.clone(),
                    ConveyorDirection::Right => conveyor_brick_assets.right_sprite_sheet.clone(),
                },
                transform,
                ..Default::default()
            },
            animations,
            animation,
            animation_state: AnimationState::default(),

            collider: BoxCollider {
                size: Vec2::new(96.0, 16.0),
                ..default()
            },
            velocity: Velocity(Vec2 { x: 0.0, y: 1.0 }),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConveyorDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct ConveyorBrick {
    pub direction: ConveyorDirection,
}

#[derive(Component)]
pub struct ConveyorBrickAnimations {
    pub working: Animation,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ConveyorMoved {
    pub direction: ConveyorDirection,
}

impl Default for ConveyorBrickAnimations {
    fn default() -> Self {
        Self {
            working: Animation(benimator::Animation::from_indices(
                0..=3,
                FrameRate::from_total_duration(Duration::from_secs_f64(PHYSICS_DELTA * 30.0)),
            )),
        }
    }
}
