use bevy::prelude::*;
use bevy_ggrs::ggrs;

const INPUT_LEFT: u8 = 1 << 0;
const INPUT_RIGHT: u8 = 1 << 1;

// pub fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
//     let mut input = 0u8;

//     if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
//         input |= INPUT_LEFT
//     }
//     if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
//         input |= INPUT_RIGHT;
//     }

//     input
// }

// pub fn direction(input: u8) -> Vec2 {
//     let mut direction = Vec2::ZERO;
//     if input & INPUT_RIGHT != 0 {
//         direction.x += 1.;
//     }
//     if input & INPUT_LEFT != 0 {
//         direction.x -= 1.;
//     }
//     direction.normalize_or_zero()
// }