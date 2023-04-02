use bevy::{prelude::*, sprite::collide_aabb::Collision};

#[derive(Component, Default)]
pub struct BoxCollider {
    pub size: Vec2,
    pub center: Vec2,
}

#[derive(Component)]
pub struct LastCollisions {
    pub entities: Vec<Entity>,
    pub collisions: Vec<Collision>,
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Velocity(pub Vec2);
