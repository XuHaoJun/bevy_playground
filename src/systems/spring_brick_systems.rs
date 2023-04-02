use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        animation::{Animation, AnimationState},
        player::{Health, Jumping, JumpingTimer, Player},
        spring_brick::*,
    },
    events::physics_events::SpringBrickTriggerEnterEvent,
    resources::SpringBrickAssets,
};

pub fn spring_brick_trigger_enter_system(
    mut commands: Commands,
    mut trigger_enter_events: EventReader<SpringBrickTriggerEnterEvent>,
    mut spring_brick_query: Query<
        (&mut AnimationState, Option<&SpringBrickSpring>),
        With<SpringBrick>,
    >,
    mut player_query: Query<(Entity, &mut Health), (With<Player>, Without<Jumping>)>,
    spring_brick_assets: Res<SpringBrickAssets>,
    audio: Res<Audio>,
) {
    for event in trigger_enter_events.iter() {
        match event.collision {
            Collision::Top => {
                let spring_brick_entity = event.myself;
                let other_entity = event.other;
                if let Ok((mut animation_state, maybe_spring)) =
                    spring_brick_query.get_mut(spring_brick_entity)
                {
                    match maybe_spring {
                        Some(_) => {
                            animation_state.reset();
                        }
                        None => {
                            commands
                                .entity(spring_brick_entity)
                                .insert(SpringBrickSpring {});
                        }
                    }
                    audio.play(spring_brick_assets.hit.clone());

                    if let Ok((player_entity, mut player_health)) =
                        player_query.get_mut(other_entity)
                    {
                        player_health.value = player_health.clamp(player_health.value + 1);
                        commands
                            .entity(player_entity)
                            .insert(Jumping {})
                            .insert(JumpingTimer::default());
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn animate_spring_brick_system(
    mut commands: Commands,
    mut spring_brick_query: Query<
        (
            Entity,
            &mut Animation,
            &mut AnimationState,
            &SpringBrickAnimations,
            Option<&SpringBrickSpring>,
        ),
        With<SpringBrick>,
    >,
) {
    for (entity, mut animation, mut animation_state, animations, maybe_spring) in
        spring_brick_query.iter_mut()
    {
        let is_spring = match maybe_spring {
            Some(_) => true,
            _ => false,
        };

        let next_animation = {
            if is_spring {
                animations.spring.clone()
            } else {
                animations.idle.clone()
            }
        };
        if *animation != next_animation {
            animation_state.reset();
            animation.clone_from(&next_animation);
        }

        if is_spring && animation_state.is_ended() {
            commands.entity(entity).remove::<SpringBrickSpring>();
        }
    }
}
