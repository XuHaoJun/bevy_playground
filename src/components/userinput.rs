use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Userinput {
    pub move_accelection: Vec2,
}