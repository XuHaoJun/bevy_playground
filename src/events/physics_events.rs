use bevy::{prelude::*, sprite::collide_aabb::Collision};

pub struct CollisionEvent {
    pub a: Entity,
    pub b: Entity,
    pub collision: Collision,
    pub a_is_player: bool,
    pub b_is_player: bool,
}

pub struct TriggerEvent {
    pub myself: Entity,
    pub other: Entity,
    pub collision: Collision,
}

#[derive(Deref, DerefMut)]
pub struct FakeBrickTriggerEnterEvent(pub TriggerEvent);

#[derive(Deref, DerefMut)]
pub struct SpringBrickTriggerEnterEvent(pub TriggerEvent);

#[derive(Deref, DerefMut)]
pub struct ConveyorBrickTriggerEnterEvent(pub TriggerEvent);

#[derive(Deref, DerefMut)]
pub struct ConveyorBrickTriggerLeaveEvent(pub TriggerEvent);
