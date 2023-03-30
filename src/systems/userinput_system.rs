use bevy::prelude::*;

use crate::components::userinput::Userinput;

pub fn userinput_system(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Userinput>) {
    let mut accelection = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        accelection.x = 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        accelection.x = -1.;
    }
    for mut userinput in player_query.iter_mut() {
        userinput.move_accelection = accelection;
    }
}
