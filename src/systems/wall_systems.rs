use crate::components::wall::*;
use bevy::prelude::*;

pub fn wall_reset_position_system(
    mut query: Query<(&WallPositionReset, &mut Transform), With<Wall>>,
) {
    for (reset, mut transform) in query.iter_mut() {
        let diff_y = transform.translation.y - reset.target_y;
        if diff_y >= 0.0 && diff_y <= 1.0 {
            transform.translation = reset.restore_position;
        }
    }
}
