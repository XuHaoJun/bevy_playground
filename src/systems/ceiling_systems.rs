use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::prelude::*;

use crate::{
    components::{
        ceiling::{CeilingHitbox, CeilingHurting, CeilingHurtingTimer},
        physics::{BoxCollider, Velocity},
        player::{Damaging, DamagingTimer, Dead, Health, Player},
    },
    resources::NailsBrickAssets,
    utils::physis_utils::{get_collider_size, get_collider_translation},
};

pub fn player_ceiling_hitbox_system(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            &Transform,
            &BoxCollider,
            &mut Velocity,
            &mut Health,
            Option<&Damaging>,
        ),
        (With<Player>, Without<Dead>, Without<CeilingHurting>),
    >,
    collider_query: Query<(&Transform, &CeilingHitbox)>,
    nails_brick_assets: Res<NailsBrickAssets>,
    audio: Res<Audio>,
) {
    for (
        player_entity,
        player_transform,
        player_collider,
        mut player_velocity,
        mut player_health,
        maybe_player_damaging,
    ) in player_query.iter_mut()
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
                if let None = maybe_player_damaging {
                    player_health.value = player_health.clamp(player_health.value - 1);
                    commands
                        .entity(player_entity)
                        .insert(DamagingTimer::default())
                        .insert(Damaging {});
                    audio.play(nails_brick_assets.hit.clone());
                }
                player_velocity.y = -8.0;
                commands
                    .entity(player_entity)
                    .insert(CeilingHurting {})
                    .insert(CeilingHurtingTimer::default())
                    .remove::<BoxCollider>();
            }
        }
    }
}

pub fn celling_hurting_player_system(
    mut commands: Commands,
    fixed_time: Res<FixedTime>,
    mut player_query: Query<
        (Entity, &mut CeilingHurtingTimer),
        (With<Player>, With<CeilingHurting>),
    >,
) {
    for (entity, mut timer) in &mut player_query {
        timer.tick(fixed_time.period);
        if timer.finished() {
            commands
                .entity(entity)
                .remove::<CeilingHurtingTimer>()
                .remove::<CeilingHurting>()
                .insert(BoxCollider::new_player_collider());
        }
    }
}
