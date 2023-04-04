use bevy::{prelude::*, sprite::collide_aabb::Collision};
use bevy_kira_audio::prelude::*;

use crate::{
    components::player::{Health, Player},
    events::physics_events::NormalBrickTriggerEnterEvent,
    resources::NormalBrickAssets,
};

pub fn normal_brick_trigger_enter_system(
    mut trigger_enter_events: EventReader<NormalBrickTriggerEnterEvent>,
    mut player_query: Query<&mut Health, With<Player>>,
    normal_brick_assets: Res<NormalBrickAssets>,
    audio: Res<Audio>,
) {
    for event in trigger_enter_events.iter() {
        match event.collision {
            Collision::Top => {
                if let Ok(mut health) = player_query.get_mut(event.other) {
                    health.value = health.clamp(health.value + 1);
                    audio.play(normal_brick_assets.hit.clone());
                }
            }
            _ => {}
        }
    }
    trigger_enter_events.clear();
}
