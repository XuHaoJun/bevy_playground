use bevy::prelude::*;

#[derive(Component, Deref, Clone, Eq, PartialEq)]
pub struct Animation(pub benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);