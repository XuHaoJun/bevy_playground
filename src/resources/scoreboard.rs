use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub score: u64,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ScoreTimer(Timer);
impl Default for ScoreTimer {
    fn default() -> Self {
        ScoreTimer(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}
