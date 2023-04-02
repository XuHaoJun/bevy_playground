use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{conveyor_brick::*, player::Player},
    events::physics_events::CollisionEvent,
    resources::ConveyorBrickAssets,
};

pub fn player_on_conveyor_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    maybe_moved_query: Query<(Entity, Option<&ConveyorMoved>), With<Player>>,
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

    for (entity, maybe_conveyor_moved) in &maybe_moved_query {
        let maybe_on_conveyor_event = conveyor_events.iter().find(|x| x.a == entity);
        match maybe_on_conveyor_event {
            Some(event) => match maybe_conveyor_moved {
                Some(_) => {}
                None => {
                    commands.entity(entity).insert(ConveyorMoved {
                        direction: conveyor_query.get(event.b).unwrap().direction,
                    });
                    audio.play(conveyor_brick_assets.hit.clone());
                }
            },
            None => {
                match maybe_conveyor_moved {
                    Some(_) => {
                        commands.entity(entity).remove::<ConveyorMoved>();
                    }
                    None => {}
                };
            }
        }
    }

    collision_events.clear();
}
