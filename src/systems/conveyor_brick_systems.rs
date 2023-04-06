use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        conveyor_brick::*,
        player::{Health, Player},
    },
    events::physics_events::CollisionEvent,
    resources::ConveyorBrickAssets,
};

pub fn player_on_conveyor_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut maybe_moved_query: Query<(Entity, &mut Health, Option<&ConveyorMoved>), With<Player>>,
    conveyor_query: Query<&ConveyorBrick>,
    conveyor_brick_assets: Res<ConveyorBrickAssets>,
    audio: Res<Audio>,
) {
    let conveyor_events: Vec<_> = collision_events
        .iter()
        .filter(|x| x.a_is_player && !x.b_is_player)
        .filter(|x| x.collision == Collision::Top)
        .filter(|x| match conveyor_query.get(x.b) {
            Ok(_) => true,
            _ => false,
        })
        .collect();

    for (player_entity, mut player_health, maybe_conveyor_moved) in &mut maybe_moved_query {
        let maybe_on_conveyor_event = conveyor_events.iter().find(|x| x.a == player_entity);
        match maybe_on_conveyor_event {
            Some(event) => match maybe_conveyor_moved {
                // Some(x) => {
                //     let conveyor_direction = conveyor_query.get(event.b).unwrap().direction;
                //     if x.direction != conveyor_direction {
                //         commands
                //             .entity(player_entity)
                //             .remove::<ConveyorMoved>()
                //             .insert(ConveyorMoved {
                //                 direction: conveyor_direction,
                //             });
                //         audio.play(conveyor_brick_assets.hit.clone());
                //     }
                // }
                Some(_) => {}
                None => {
                    commands.entity(player_entity).insert(ConveyorMoved {
                        direction: conveyor_query.get(event.b).unwrap().direction,
                    });
                    player_health.value = player_health.clamp(player_health.value + 1);
                    audio.play(conveyor_brick_assets.hit.clone());
                }
            },
            None => {
                match maybe_conveyor_moved {
                    Some(_) => {
                        commands.entity(player_entity).remove::<ConveyorMoved>();
                    }
                    None => {}
                };
            }
        }
    }

    collision_events.clear();
}
