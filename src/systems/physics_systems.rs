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
        player::Player,
    },
    events::physics_events::{CollisionEvent, FakeBrickTriggerEnterEvent, TriggerEnterEvent},
    resources::NormalBrickAssets,
    utils::physis_utils::{get_collider_size, get_collider_translation},
};

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
            Option<&LastCollisions>,
        ),
        Without<Player>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
    mut fake_brick_trigger_enter_events: EventWriter<FakeBrickTriggerEnterEvent>,
    // normal_brick_assets: Res<NormalBrickAssets>,
    // audio: Res<Audio>,
) {
    for (
        player_entity,
        mut player_transform,
        player_collider,
        mut player_velocity,
        mut player_last_collisions,
    ) in player_query.iter_mut()
    {
        player_velocity.y = -1.0;

        let player_translation =
            get_collider_translation(player_transform.as_ref(), player_collider);
        let player_size = get_collider_size(player_transform.as_ref(), player_collider);

        let mut collision_entities: Vec<Entity> = Vec::new();

        for (
            other_entity,
            transform,
            collider,
            normal_brick_opt,
            fake_brick_opt,
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

                if let Some(_) = normal_brick_opt {
                    match collision {
                        Collision::Top => {
                            if is_trigger_enter {
                                // audio.play(normal_brick_assets.hit.clone());
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(_) = fake_brick_opt {
                    if is_trigger_enter {
                        fake_brick_trigger_enter_events.send(FakeBrickTriggerEnterEvent {
                            0: TriggerEnterEvent {
                                myself: other_entity,
                                other: player_entity,
                                collision,
                            },
                        });
                    }
                }
            }
        }

        player_last_collisions.entities = collision_entities;
    }
}
