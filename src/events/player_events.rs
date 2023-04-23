use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerEnterDeadEvent {
  pub handle: usize
}

#[derive(Default)]
pub struct PlayerLeaveDeadEvent {
  pub handle: usize
}
