use bevy::prelude::*;

use crate::resources::scoreboard::{ScoreTimer, Scoreboard};

pub fn add_score(
    time: Res<Time>,
    mut scoreboard: ResMut<Scoreboard>,
    mut score_timer: ResMut<ScoreTimer>,
) {
    score_timer.tick(time.delta());
    if score_timer.finished() {
        scoreboard.score += 1;
    }
}
