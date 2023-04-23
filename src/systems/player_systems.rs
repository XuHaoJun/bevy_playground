use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        animation::{Animation, AnimationState},
        conveyor_brick::{ConveyorDirection, ConveyorMoved},
        physics::{BoxCollider, Velocity},
        player::*,
        userinput::Userinput,
    },
    constants::PHYSICS_DELTA,
    events::player_events::PlayerEnterDeadEvent,
    resources::PlayerAssets,
};

pub fn player_controller_system(
    mut player_query: Query<
        (
            &mut Velocity,
            &Userinput,
            Option<&Flying>,
            Option<&ConveyorMoved>,
        ),
        With<Player>,
    >,
) {
    for (mut velocity, userinput, maybe_flying, maybe_conveyor_moved) in player_query.iter_mut() {
        let move_speed = {
            match maybe_flying {
                Some(_) => 2.0,
                None => 4.0,
            }
        };
        let conveyor_x_velocity = match maybe_conveyor_moved {
            Some(conveyor_moved) => match conveyor_moved.direction {
                ConveyorDirection::Left => -2.0,
                ConveyorDirection::Right => 2.0,
            },
            None => 0.0,
        };
        velocity.x = (move_speed * userinput.move_accelection.x) + conveyor_x_velocity;
    }
}

pub fn animate_player_system(
    mut player_query: Query<
        (
            &mut Animation,
            &mut AnimationState,
            &Userinput,
            &PlayerAnimations,
            Option<&Flying>,
            Option<&Damaging>,
        ),
        With<Player>,
    >,
) {
    for (mut animation, mut animation_state, userinput, animations, flying, damaged) in
        player_query.iter_mut()
    {
        let is_flying = match flying {
            Some(_) => true,
            _ => false,
        };

        let is_damaging = match damaged {
            Some(_) => true,
            _ => false,
        };

        let next_animation = get_next_animation(
            animation.clone(),
            animations.clone(),
            userinput.move_accelection,
            is_flying,
            is_damaging,
        );
        if next_animation != *animation {
            animation_state.reset();
            animation.clone_from(&next_animation);
        }
    }
}

fn get_next_animation(
    current_animation: Animation,
    animations: PlayerAnimations,
    direction: Vec2,
    is_flying: bool,
    is_damaging: bool,
) -> Animation {
    if direction.x > 0.0 {
        let next_animation = if is_damaging && is_flying {
            &animations.fly_hurt_right_run
        } else if is_damaging {
            &animations.hurt_right_run
        } else if is_flying {
            &animations.fly_right_run
        } else {
            &animations.right_run
        };
        return next_animation.clone();
    }
    if direction.x < 0.0 {
        let next_animation = if is_damaging && is_flying {
            &animations.fly_hurt_left_run
        } else if is_damaging {
            &animations.hurt_left_run
        } else if is_flying {
            &animations.fly_left_run
        } else {
            &animations.left_run
        };
        return next_animation.clone();
    }
    if direction == Vec2::ZERO {
        let next_animation = if is_damaging && is_flying {
            &animations.fly_hurt_idle
        } else if is_damaging {
            &animations.hurt_idle
        } else if is_flying {
            &animations.fly_idle
        } else {
            &animations.idle
        };
        return next_animation.clone();
    }
    return current_animation;
}

pub fn enter_grounded_system(
    mut commands: Commands,
    no_grounded_query: Query<(Entity, &Velocity), (Without<Grounded>, With<Player>)>,
) {
    for (entity, velocity) in no_grounded_query.iter() {
        if velocity.y == 0.0 {
            commands.entity(entity).insert(Grounded {});
        }
    }
}

pub fn leave_grounded_system(
    mut commands: Commands,
    grounded_query: Query<(Entity, &Velocity), (With<Grounded>, With<Player>)>,
) {
    for (entity, velocity) in grounded_query.iter() {
        if velocity.y != 0.0 {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

pub fn enter_flying_system(
    mut commands: Commands,
    no_flying_query: Query<(Entity, &Velocity), (Without<Flying>, With<Player>)>,
) {
    for (entity, velocity) in no_flying_query.iter() {
        if velocity.y != 0.0 {
            commands.entity(entity).insert(Flying {});
        }
    }
}

pub fn leave_flying_system(
    mut commands: Commands,
    flying_query: Query<(Entity, &Velocity), (With<Flying>, With<Player>)>,
) {
    for (entity, velocity) in flying_query.iter() {
        if velocity.y == 0.0 {
            commands.entity(entity).remove::<Flying>();
        }
    }
}

pub fn enter_dead_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Player, &Health), Without<Dead>>,
    player_assets: Res<PlayerAssets>,
    audio: Res<Audio>,
    mut dead_events: EventWriter<PlayerEnterDeadEvent>,
) {
    for (entity, player, health) in player_query.iter() {
        if health.value <= 0 {
            commands.entity(entity).insert(Dead {});
            audio.play(player_assets.die.clone());
            dead_events.send(PlayerEnterDeadEvent {
                handle: player.handle,
            })
        }
    }
}

pub fn damaging_timer_system(
    mut commands: Commands,
    mut timer_query: Query<(Entity, &mut DamagingTimer), With<Damaging>>,
) {
    for (entity, mut cooldown) in timer_query.iter_mut() {
        cooldown.timer.tick(Duration::from_secs_f64(PHYSICS_DELTA));
        if cooldown.timer.finished() {
            commands
                .entity(entity)
                .remove::<(Damaging, DamagingTimer)>();
        }
    }
}

pub fn jumping_timer_system(
    mut commands: Commands,
    mut timer_query: Query<(Entity, &mut JumpingTimer), With<Jumping>>,
) {
    for (entity, mut cooldown) in timer_query.iter_mut() {
        cooldown.timer.tick(Duration::from_secs_f64(PHYSICS_DELTA));
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<(Jumping, JumpingTimer)>();
        }
    }
}

pub fn player_out_window_die_system(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Health, &Transform, &BoxCollider), (Without<Dead>, With<Player>)>,
) {
    if let Ok(window) = primary_query.get_single() {
        let height = window.height().trunc();
        let width = window.width();
        for (mut health, transform, collider) in player_query.iter_mut() {
            let x = transform.translation.x + collider.size.x / 2.0;
            let y = transform.translation.y + collider.size.y / 2.0;
            if x < -width / 2.0 || x > width / 2.0 || y < -height / 2.0 || y > height / 2.0 {
                health.value = health.clamp(0);
            }
        }
    }
}
