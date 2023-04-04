use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::userinput::Userinput;

pub fn userinput_system(
    keys: Res<Input<KeyCode>>,
    touches: Res<Touches>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut userinput_query: Query<&mut Userinput>,
) {
    let mut accelection = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        accelection.x = 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        accelection.x = -1.;
    }

    let maybe_touch = touches.iter().last();
    if let Ok(window) = primary_query.get_single() {
        let center = window.width() / 2.0;
        if let Some(touch) = maybe_touch {
            if touches.get_pressed(touch.id()).is_some() {
                let tx = touch.position().x;
                if tx > center {
                    accelection.x = 1.;
                } else if tx < center {
                    accelection.x = -1.;
                }
            }
        }
    }

    for mut userinput in userinput_query.iter_mut() {
        userinput.move_accelection = accelection;
    }
}
