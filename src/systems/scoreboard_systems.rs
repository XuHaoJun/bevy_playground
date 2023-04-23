use bevy::prelude::*;

use crate::{
    components::player::{Dead, Player, PlayerScore},
    resources::scoreboard::{ScoreTimer, Scoreboard},
};

pub fn add_score(
    time: Res<Time>,
    mut scoreboard: ResMut<Scoreboard>,
    mut score_timer: ResMut<ScoreTimer>,
    mut player_query: Query<&mut PlayerScore, (With<Player>, Without<Dead>)>,
) {
    score_timer.tick(time.delta());
    if score_timer.finished() {
        scoreboard.score += 1;
        for mut player_score in player_query.iter_mut() {
            player_score.score = scoreboard.score;
        }
    }
}

pub fn init_score(mut scoreboard: ResMut<Scoreboard>) {
    scoreboard.score = 0;
}
