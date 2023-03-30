use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        animation::{Animation, AnimationState},
        fake_brick::{
            new_fake_brick_box_collider, FakeBrick, FakeBrickAnimations, FakeBrickBeforeFlipDelay,
            FakeBrickFliping,
        },
        physics::BoxCollider,
    },
    constants::PHYSICS_DELTA,
    events::physics_events::FakeBrickTriggerEnterEvent,
    resources::FakeBrickAssets,
};

pub fn animate_fake_brick_system(
    mut commands: Commands,
    mut fake_brick_query: Query<
        (
            Entity,
            &mut Animation,
            &mut AnimationState,
            &FakeBrickAnimations,
            Option<&FakeBrickFliping>,
        ),
        With<FakeBrick>,
    >,
) {
    for (entity, mut animation, mut animation_state, animations, fliping_opt) in
        fake_brick_query.iter_mut()
    {
        let is_fliping = match fliping_opt {
            Some(_) => true,
            _ => false,
        };

        let next_animation = {
            if is_fliping {
                animations.flip.clone()
            } else {
                animations.idle.clone()
            }
        };
        if *animation != next_animation {
            animation_state.reset();
            animation.clone_from(&next_animation);
        }

        if is_fliping && animation_state.is_ended() {
            commands
                .entity(entity)
                .remove::<FakeBrickFliping>()
                .insert(new_fake_brick_box_collider());
        }
    }
}

pub fn fake_brick_trigger_enter_system(
    mut trigger_enter_events: EventReader<FakeBrickTriggerEnterEvent>,
    mut commands: Commands,
    fake_brick_query: Query<
        &FakeBrick,
        (Without<FakeBrickFliping>, Without<FakeBrickBeforeFlipDelay>),
    >,
) {
    for event in trigger_enter_events.iter() {
        match event.collision {
            Collision::Top => {
                let fake_brick_entity = event.myself;
                if let Ok(_) = fake_brick_query.get(fake_brick_entity) {
                    commands
                        .entity(fake_brick_entity)
                        .insert(FakeBrickBeforeFlipDelay::default());
                }
            }
            _ => {}
        }
    }
    trigger_enter_events.clear();
}

pub fn fake_brick_flip_system(
    mut commands: Commands,
    mut fake_brick_query: Query<(Entity, &mut FakeBrickBeforeFlipDelay), With<FakeBrick>>,
    // fake_brick_assets: Res<FakeBrickAssets>,
    // audio: Res<Audio>,
) {
    for (entity, mut delay) in fake_brick_query.iter_mut() {
        delay.tick(Duration::from_secs_f64(PHYSICS_DELTA));
        if delay.finished() {
            // audio.play(fake_brick_assets.hit.clone());
            commands
                .entity(entity)
                .remove::<FakeBrickBeforeFlipDelay>()
                .remove::<BoxCollider>()
                .insert(FakeBrickFliping {});
        }
    }
}
