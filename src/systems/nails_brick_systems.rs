use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        nails_brick::NailsBrickHitbox,
        physics::BoxCollider,
        player::{Damaging, DamagingTimer, Dead, Health, Player},
    },
    resources::NailsBrickAssets,
    utils::physis_utils::{get_collider_size, get_collider_translation},
};

pub fn player_nails_hitbox_system(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &BoxCollider, &mut Health),
        (With<Player>, Without<Damaging>, Without<Dead>),
    >,
    collider_query: Query<(&Transform, &NailsBrickHitbox)>,
    // nails_brick_assets: Res<NailsBrickAssets>,
    // audio: Res<Audio>,
) {
    for (player_entity, player_transform, player_collider, mut player_health) in
        player_query.iter_mut()
    {
        let player_translation = get_collider_translation(player_transform, player_collider);
        let player_size = get_collider_size(player_transform, player_collider);

        for (transform, collider) in collider_query.iter() {
            let collider_translation = get_collider_translation(transform, collider);
            let collider_size = get_collider_size(transform, collider);

            let collision = collide(
                player_translation,
                player_size,
                collider_translation,
                collider_size,
            );

            if let Some(_) = collision {
                player_health.value = player_health.clamp(player_health.value - 1);
                commands
                    .entity(player_entity)
                    .insert(DamagingTimer::default())
                    .insert(Damaging {});
                // audio.play(nails_brick_assets.hit.clone());
            }
        }
    }
}
