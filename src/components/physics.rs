use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BoxCollider {
    pub size: Vec2,
    pub center: Vec2,
}

#[derive(Component)]
pub struct LastCollisions {
    pub entities: Vec<Entity>,
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Velocity(pub Vec2);
