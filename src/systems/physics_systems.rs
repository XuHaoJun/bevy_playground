use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        fake_brick::FakeBrick,
        normal_brick::NormalBrick,
        physics::{BoxCollider, LastCollisions, Velocity},
        player::{Jumping, Player},
        spring_brick::SpringBrick,
    },
    events::physics_events::{
        CollisionEvent, FakeBrickTriggerEnterEvent, SpringBrickTriggerEnterEvent, TriggerEnterEvent,
    },
    resources::NormalBrickAssets,
    utils::physis_utils::{get_collider_size, get_collider_translation},
};

// fn clone_collision<'a>(x: &'a Collision) -> Collision { //     match x {
//         Collision::Left => Collision::Left,
//         Collision::Right => Collision::Right,
//         Collision::Top => Collision::Top,
//         Collision::Bottom => Collision::Bottom,
//         Collision::Inside => Collision::Inside,
//     }
// }

pub fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.extend(0.);
    }
}

pub fn player_collision_system(
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &BoxCollider,
            &mut Velocity,
            &mut LastCollisions,
            Option<&Jumping>,
        ),
        With<Player>,
    >,
    collider_query: Query<
        (
            Entity,
            &Transform,
            &BoxCollider,
            Option<&NormalBrick>,
            Option<&FakeBrick>,
            Option<&SpringBrick>,
            Option<&LastCollisions>,
        ),
        Without<Player>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
    mut fake_brick_trigger_enter_events: EventWriter<FakeBrickTriggerEnterEvent>,
    mut spring_brick_trigger_enter_events: EventWriter<SpringBrickTriggerEnterEvent>,
    normal_brick_assets: Res<NormalBrickAssets>,
    audio: Res<Audio>,
) {
    for (
        player_entity,
        mut player_transform,
        player_collider,
        mut player_velocity,
        mut player_last_collisions,
        maybe_player_jumping,
    ) in player_query.iter_mut()
    {
        let is_player_jumping = match maybe_player_jumping {
            Some(_) => true,
            None => false,
        };
        player_velocity.y = {
            if is_player_jumping {
                4.0
            } else {
                -1.0
            }
        };
        if is_player_jumping {
            return;
        }

        let player_translation =
            get_collider_translation(player_transform.as_ref(), player_collider);
        let player_size = get_collider_size(player_transform.as_ref(), player_collider);

        let mut collision_entities: Vec<Entity> = Vec::new();

        for (
            other_entity,
            transform,
            collider,
            maybe_normal_brick,
            maybe_fake_brick,
            maybe_spring_brick,
            other_last_collisions_opt,
        ) in collider_query.iter()
        {
            let collider_translation = get_collider_translation(transform, collider);
            let collider_size = get_collider_size(transform, collider);

            let collision = collide(
                player_translation,
                player_size,
                collider_translation,
                collider_size,
            );

            if let Some(collision) = collision {
                collision_entities.push(other_entity);

                collision_events.send_default();

                match collision {
                    Collision::Left => {
                        if player_velocity.x > 0.0 {
                            player_velocity.x = 0.0;
                            player_transform.translation.x = collider_translation.x
                                - (collider.size.x / 2.0)
                                - (player_collider.size.x / 2.0);
                        }
                    }
                    Collision::Right => {
                        if player_velocity.x < 0.0 {
                            player_velocity.x = 0.0;
                            player_transform.translation.x = collider_translation.x
                                + (collider.size.x / 2.0)
                                + (player_collider.size.x / 2.0);
                        }
                    }
                    Collision::Top => {
                        if player_velocity.y < 0.0 {
                            player_velocity.y = 0.0;
                            player_transform.translation.y = collider_translation.y
                                + (collider.size.y / 2.0)
                                + (player_collider.size.y / 2.0);
                        }
                    }
                    Collision::Bottom => {
                        if player_velocity.y > 0.0 {
                            player_velocity.y = 0.0;
                            player_transform.translation.y = collider_translation.y
                                - (collider.size.y / 2.0)
                                - (player_collider.size.y / 2.0);
                        }
                    }
                    Collision::Inside => {}
                }

                let is_trigger_enter = !player_last_collisions
                    .entities
                    .iter()
                    .any(|x| *x == other_entity);

                if let Some(_) = maybe_normal_brick {
                    match collision {
                        Collision::Top => {
                            if is_trigger_enter {
                                audio.play(normal_brick_assets.hit.clone());
                            }
                        }
                        _ => {}
                    }
                }

                // wait bevy 0.11 collision impl copy clone trait
                // https://github.com/bevyengine/bevy/pull/8121
                if let Some(_) = maybe_fake_brick {
                    if is_trigger_enter {
                        fake_brick_trigger_enter_events.send(FakeBrickTriggerEnterEvent {
                            0: TriggerEnterEvent {
                                myself: other_entity,
                                other: player_entity,
                                collision: match collision {
                                    Collision::Left => Collision::Left,
                                    Collision::Right => Collision::Right,
                                    Collision::Top => Collision::Top,
                                    Collision::Bottom => Collision::Bottom,
                                    Collision::Inside => Collision::Inside,
                                },
                            },
                        });
                    }
                }

                if let Some(_) = maybe_spring_brick {
                    if is_trigger_enter {
                        spring_brick_trigger_enter_events.send(SpringBrickTriggerEnterEvent {
                            0: TriggerEnterEvent {
                                myself: other_entity,
                                other: player_entity,
                                collision: match collision {
                                    Collision::Left => Collision::Left,
                                    Collision::Right => Collision::Right,
                                    Collision::Top => Collision::Top,
                                    Collision::Bottom => Collision::Bottom,
                                    Collision::Inside => Collision::Inside,
                                },
                            },
                        });
                    }
                }
            }
        }

        player_last_collisions.entities = collision_entities;
    }
}

// fn get_next_velocity_translation(
//     collision: Collision,
//     player_velocity: &Velocity,
//     player_translation: &Vec3,
// ) -> (Velocity, Vec3) {
//     match collision {
//         Collision::Left => {
//             if player_velocity.x > 0.0 {
//                 player_velocity.x = 0.0;
//                 player_transform.translation.x = collider_translation.x
//                     - (collider.size.x / 2.0)
//                     - (player_collider.size.x / 2.0);
//             }
//         }
//         Collision::Right => {
//             if player_velocity.x < 0.0 {
//                 player_velocity.x = 0.0;
//                 player_transform.translation.x = collider_translation.x
//                     + (collider.size.x / 2.0)
//                     + (player_collider.size.x / 2.0);
//             }
//         }
//         Collision::Top => {
//             if player_velocity.y < 0.0 {
//                 player_velocity.y = 0.0;
//                 player_transform.translation.y = collider_translation.y
//                     + (collider.size.y / 2.0)
//                     + (player_collider.size.y / 2.0);
//             }
//         }
//         Collision::Bottom => {
//             if player_velocity.y > 0.0 {
//                 player_velocity.y = 0.0;
//                 player_transform.translation.y = collider_translation.y
//                     - (collider.size.y / 2.0)
//                     - (player_collider.size.y / 2.0);
//             }
//         }
//         Collision::Inside => (player_velocity.clone(), player_translation.clone()),
//     }
// }
