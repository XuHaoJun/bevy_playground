use bevy::prelude::*;

use crate::components::animation::{Animation, AnimationState};

pub fn animate_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut state, mut texture, animation) in query.iter_mut() {
        // Update the state
        state.update(animation, time.delta());

        // Update the texture atlas
        texture.index = state.frame_index();
    }
}
