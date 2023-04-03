use benimator::FrameRate;
use bevy::prelude::*;
use std::time::Duration;

use super::{
    animation::AnimationState,
    physics::{BoxCollider, LastCollisions, Velocity},
    userinput::Userinput,
};
use crate::{components::animation::Animation, constants::PHYSICS_DELTA, resources::PlayerAssets};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,

    pub spirtes: SpriteSheetBundle,
    pub animations: PlayerAnimations,
    pub animation: Animation,
    pub animation_state: AnimationState,

    pub userinput: Userinput,

    pub collider: BoxCollider,
    pub last_collisions: LastCollisions,
    pub velocity: Velocity,
}

impl PlayerBundle {
    pub fn new(handle: u32, transform: Transform, player_assets: &PlayerAssets) -> Self {
        let animations = PlayerAnimations::default();
        let animation = animations.idle.clone();
        PlayerBundle {
            player: Player { handle },
            health: Health::new_player_health(),

            spirtes: SpriteSheetBundle {
                texture_atlas: player_assets.sprite_sheet.clone(),
                transform: transform.clone(),
                ..Default::default()
            },
            animations,
            animation,
            animation_state: AnimationState::default(),

            userinput: Userinput {
                move_accelection: Vec2::ZERO,
            },

            collider: BoxCollider::new_player_collider(),
            last_collisions: LastCollisions {
                entities: Vec::new(),
                collisions: Vec::new(),
            },
            velocity: Velocity(Vec2 { x: 0.0, y: -1.0 }),
        }
    }
}

impl BoxCollider {
    pub fn new_player_collider() -> Self {
        Self {
            size: Vec2::new(32.0, 32.0),
            ..default()
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct Player {
    pub handle: u32,
}

#[derive(Component, Clone)]
pub struct PlayerAnimations {
    pub idle: Animation,
    pub left_run: Animation,
    pub right_run: Animation,

    pub hurt_idle: Animation,
    pub hurt_left_run: Animation,
    pub hurt_right_run: Animation,

    pub fly_idle: Animation,
    pub fly_left_run: Animation,
    pub fly_right_run: Animation,

    pub fly_hurt_idle: Animation,
    pub fly_hurt_left_run: Animation,
    pub fly_hurt_right_run: Animation,
}

impl Default for PlayerAnimations {
    fn default() -> Self {
        let anime_time = Duration::from_secs_f64(PHYSICS_DELTA * 20.0);
        PlayerAnimations {
            idle: Animation(benimator::Animation::from_indices(
                [8],
                FrameRate::from_total_duration(anime_time),
            )),
            left_run: Animation(benimator::Animation::from_indices(
                0..=3,
                FrameRate::from_total_duration(anime_time),
            )),
            right_run: Animation(benimator::Animation::from_indices(
                9..=12,
                FrameRate::from_total_duration(anime_time.clone()),
            )),

            hurt_idle: Animation(benimator::Animation::from_indices(
                [17, 8],
                FrameRate::from_total_duration(anime_time.clone()),
            )),
            hurt_left_run: Animation(benimator::Animation::from_indices(
                4..=7,
                FrameRate::from_total_duration(anime_time.clone()),
            )),
            hurt_right_run: Animation(benimator::Animation::from_indices(
                13..=16,
                FrameRate::from_total_duration(anime_time.clone()),
            )),

            fly_idle: Animation(benimator::Animation::from_indices(
                36..=39,
                FrameRate::from_total_duration(anime_time.clone()),
            )),
            fly_left_run: Animation(benimator::Animation::from_indices(
                18..=21,
                FrameRate::from_total_duration(anime_time.clone()),
            )),
            fly_right_run: Animation(benimator::Animation::from_indices(
                27..=30,
                FrameRate::from_total_duration(anime_time.clone()),
            )),

            fly_hurt_idle: Animation(benimator::Animation::from_indices(
                40..=43,
                FrameRate::from_total_duration(anime_time.clone()),
            )),
            fly_hurt_left_run: Animation(benimator::Animation::from_indices(
                22..=25,
                FrameRate::from_total_duration(anime_time.clone()),
            )),
            fly_hurt_right_run: Animation(benimator::Animation::from_indices(
                28..=31,
                FrameRate::from_total_duration(anime_time.clone()),
            )),
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub(crate) value: i32,
    pub(crate) max: i32,
}

impl Health {
    pub fn new_player_health() -> Self {
        Health { value: 10, max: 10 }
    }

    pub fn clamp(&self, input: i32) -> i32 {
        let min = 0;
        let max = self.max;
        if input > max {
            max
        } else if input < min {
            min
        } else {
            input
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded {}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Flying {}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Damaging {}

#[derive(Component)]
pub struct Dead {}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Jumping {}

#[derive(Component)]
pub struct JumpingTimer {
    pub(crate) timer: Timer,
}

impl Default for JumpingTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f64(PHYSICS_DELTA * 12.5),
                TimerMode::Once,
            ),
        }
    }
}

#[derive(Component, Reflect)]
pub struct DamagingTimer {
    pub(crate) timer: Timer,
}

impl Default for DamagingTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f64(PHYSICS_DELTA * 75.0),
                TimerMode::Once,
            ),
        }
    }
}
