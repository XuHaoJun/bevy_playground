use benimator::FrameRate;
use bevy::prelude::*;
use seldom_state::prelude::*;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref, Clone)]
struct Animation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
struct AnimationState(benimator::State);

#[derive(Component)]
struct PlayerAnimations {
    idle: Animation,
    left_run: Animation,
    right_run: Animation,

    hurt_idle: Animation,
    hurt_left_run: Animation,
    hurt_right_run: Animation,

    fly_idle: Animation,
    fly_left_run: Animation,
    fly_right_run: Animation,

    fly_hurt_idle: Animation,
    fly_hurt_left_run: Animation,
    fly_hurt_right_run: Animation,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(spawn)
        .add_system(move_player)
        .add_system(animate)
        .add_system(velocity_system)
        .add_system(ground_falling_system)
        .add_system(test_system.after(ground_falling_system))
        .run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn(Camera2dBundle::default());

    let move_fps = 12.0;
    // Create an animation
    let player_animations = PlayerAnimations {
        idle: Animation(benimator::Animation::from_indices(
            8..=8,
            FrameRate::from_fps(1.0),
        )),
        left_run: Animation(benimator::Animation::from_indices(
            0..=3,
            FrameRate::from_fps(move_fps),
        )),
        right_run: Animation(benimator::Animation::from_indices(
            9..=12,
            FrameRate::from_fps(move_fps),
        )),

        hurt_idle: Animation(benimator::Animation::from_indices(
            17..=17,
            FrameRate::from_fps(1.0),
        )),
        hurt_left_run: Animation(benimator::Animation::from_indices(
            4..=7,
            FrameRate::from_fps(move_fps),
        )),
        hurt_right_run: Animation(benimator::Animation::from_indices(
            13..=16,
            FrameRate::from_fps(move_fps),
        )),

        fly_idle: Animation(benimator::Animation::from_indices(
            36..=39,
            FrameRate::from_fps(1.0),
        )),
        fly_left_run: Animation(benimator::Animation::from_indices(
            18..=21,
            FrameRate::from_fps(move_fps),
        )),
        fly_right_run: Animation(benimator::Animation::from_indices(
            27..=30,
            FrameRate::from_fps(move_fps),
        )),

        fly_hurt_idle: Animation(benimator::Animation::from_indices(
            40..=43,
            FrameRate::from_fps(1.0),
        )),
        fly_hurt_left_run: Animation(benimator::Animation::from_indices(
            22..=25,
            FrameRate::from_fps(move_fps),
        )),
        fly_hurt_right_run: Animation(benimator::Animation::from_indices(
            28..=31,
            FrameRate::from_fps(move_fps),
        )),
    };

    let idle_animation = player_animations.idle.clone();

    commands
        // Spawn a bevy sprite-sheet
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("player.png"),
                Vec2::new(32.0, 32.0),
                9,
                4,
                None,
                None,
            )),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2 { x: 0.0, y: -1.0 }))
        // Insert the animation
        .insert(player_animations)
        .insert(idle_animation)
        // Insert the state
        .insert(AnimationState::default());
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Grounded {}

#[derive(Component)]
struct Falling {}

#[derive(Component)]
struct Damaged {}

#[derive(Component)]
struct Velocity(Vec2);

fn move_player(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    damged_query: Query<Option<&Damaged>>,
    falling_query: Query<Option<&Falling>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    player_query2: Query<(Entity, &PlayerAnimations, &Player)>,
) {
    let mut direction = Vec2::ZERO;
    // if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
    //     direction.y += 1.;
    // }
    // if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
    //     direction.y -= 1.;
    // }
    // commands.entity(player_query).remove::<Animation>();

    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x = 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x = -1.;
    }

    let is_falling = match falling_query.get_single() {
        Ok(x) => match x {
            Some(_) => true,
            _ => false,
        },
        _ => false,
    };
    let is_damged = match damged_query.get_single() {
        Ok(x) => match x {
            Some(_) => true,
            _ => false,
        },
        _ => false,
    };
    // match falling_query.get_single() {
    //     Ok(x) => match x {
    //         Some(_) => {
    //             println!("some")
    //         },
    //         _ => {
    //             println!("none")
    //         }
    //     },
    //     _ => {
    //         // println!("?")
    //     }
    // }

    // for x in falling_query.iter() {
    //     match x {
    //         Some(_) => {
    //             println!("some")
    //         },
    //         None => {
    //             println!("none")
    //         }
    //     }
    // }
    // println!("{}", is_falling);

    if direction.x > 0.0 {
        for (entity, animations, _) in player_query2.iter() {
            let next_animation = if is_damged && is_falling {
                &animations.fly_hurt_right_run
            } else if is_damged {
                &animations.hurt_right_run
            } else if is_falling {
                &animations.fly_right_run
            } else {
                &animations.right_run
            };

            commands
                .entity(entity)
                .remove::<Animation>()
                .insert(next_animation.clone());
        }
    }
    if direction.x < 0.0 {
        for (entity, animations, _) in player_query2.iter() {
            let next_animation = if is_damged && is_falling {
                &animations.fly_hurt_left_run
            } else if is_damged {
                &animations.hurt_left_run
            } else if is_falling {
                &animations.fly_left_run
            } else {
                &animations.left_run
            };
            commands
                .entity(entity)
                .remove::<Animation>()
                .insert(next_animation.clone());
        }
    }
    if direction == Vec2::ZERO {
        for (entity, animations, _) in player_query2.iter() {
            let next_animation = if is_damged && is_falling {
                &animations.fly_hurt_idle
            } else if is_damged {
                &animations.hurt_idle
            } else if is_falling {
                &animations.fly_idle
            } else {
                &animations.idle
            };
            commands
                .entity(entity)
                .remove::<Animation>()
                .insert(next_animation.clone());
        }
        return;
    }

    let move_speed = 1.0;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

fn ground_falling_system(
    mut commands: Commands,
    query: Query<(Entity, &Velocity), Without<Falling>>,
) {
    for (entity, velocity) in query.iter() {
        if velocity.0.y < 0.0 {
            commands.entity(entity).insert(Falling {});
        }
    }
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.);
    }
}

fn test_system(falling_query: Query<Option<&Falling>, With<Player>>,) {
    for x in falling_query.iter() {
        match x {
            None => {
                println!("none")
            },
            Some(x) => {
                println!("some")
            },
        }
    }
}