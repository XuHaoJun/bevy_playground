use bevy::prelude::{Transform, Vec2, Vec3};

use crate::components::physics::BoxCollider;

pub fn get_collider_translation(transform: &Transform, collider: &BoxCollider) -> Vec3 {
    transform.translation + collider.center.extend(0.0)
}

pub fn get_collider_size(transform: &Transform, collider: &BoxCollider) -> Vec2 {
    collider.size * transform.scale.truncate()
}
