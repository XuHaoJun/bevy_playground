use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ggrs::PlayerInputs;

use crate::{
    components::{player::Player, userinput::Userinput},
    constants::{GgrsConfig, INPUT_LEFT, INPUT_RIGHT},
    resources::{InGameMode, InGameSetting},
};

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

pub fn userinput_system_2(
    in_game_setting: Res<InGameSetting>,

    keys: Res<Input<KeyCode>>,
    touches: Res<Touches>,
    primary_query: Query<&Window, With<PrimaryWindow>>,

    maybe_network_inputs: Option<Res<PlayerInputs<GgrsConfig>>>,
    mut player_query: Query<(&Player, &mut Userinput)>,
) {
    match in_game_setting.mode {
        InGameMode::Offline => {
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

            for (_, mut userinput) in player_query.iter_mut() {
                userinput.move_accelection = accelection;
            }
        }
        InGameMode::Online => {
            if let Some(network_inputs) = maybe_network_inputs {
                for (player, mut userinput) in player_query.iter_mut() {
                    let (input, _) = network_inputs[player.handle];

                    let mut accelection = Vec2::ZERO;
                    if input & INPUT_RIGHT != 0 {
                        accelection.x += 1.;
                    }
                    if input & INPUT_LEFT != 0 {
                        accelection.x -= 1.;
                    }

                    userinput.move_accelection = accelection;
                }
            }
        }
        _ => {}
    }
}
