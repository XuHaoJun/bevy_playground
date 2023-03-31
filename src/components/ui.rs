use bevy::prelude::*;

#[derive(Component)]
pub struct InGameUi {}

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component, Default)]
pub struct PlayerHealthText {
    pub handle: u32,
}
