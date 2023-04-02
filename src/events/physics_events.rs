use bevy::{prelude::*, sprite::collide_aabb::Collision};

#[derive(Default)]
pub struct CollisionEvent;

pub struct TriggerEnterEvent {
    pub myself: Entity,
    pub other: Entity,
    pub collision: Collision,
}

#[derive(Deref, DerefMut)]
pub struct FakeBrickTriggerEnterEvent(pub TriggerEnterEvent);

#[derive(Deref, DerefMut)]
pub struct SpringBrickTriggerEnterEvent(pub TriggerEnterEvent);