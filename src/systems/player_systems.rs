use std::time::Duration;

use bevy::prelude::*;

use crate::{
    components::{
        animation::{Animation, AnimationState},
        physics::Velocity,
        player::{
            Damaging, DamagingTimer, Dead, Flying, Grounded, Health, Player, PlayerAnimations,
        },
        userinput::Userinput,
    },
    constants::PHYSICS_DELTA,
};

pub fn player_controller_system(mut player_query: Query<(&mut Velocity, &Userinput)>) {
    for (mut velocity, userinput) in player_query.iter_mut() {
        let move_speed = 4.0;
        velocity.x = move_speed * userinput.move_accelection.x;
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
    health_query: Query<(Entity, &Health), Without<Dead>>,
) {
    for (entity, health) in health_query.iter() {
        if health.value <= 0 {
            commands.entity(entity).insert(Dead {});
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
