use benimator::FrameRate;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
    window::{CursorGrabMode, PresentMode},
};

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref, Clone, Eq, PartialEq)]
struct Animation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
struct AnimationState(benimator::State);

#[derive(Component, Clone)]
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

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct BoxCollider {
    size: Vec2,
}

#[derive(Component)]
struct NormalBrick;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "ns-shaft clone".to_string(),
                        width: 720.,
                        height: 1280.,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_startup_system(spawn)
        .add_event::<CollisionEvent>()
        .add_system(animate_system)
        .add_system(animate_player_system.before(animate_system))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0 as f64))
                .with_system(userinput_system.before(player_controller_system))
                .with_system(player_controller_system)
                .with_system(velocity_system)
                .with_system(enter_grounded_system.after(check_for_collisions))
                .with_system(leave_grounded_system.after(check_for_collisions))
                .with_system(enter_flying_system.after(check_for_collisions))
                .with_system(leave_flying_system.after(check_for_collisions))
                .with_system(
                    check_for_collisions
                        .after(player_controller_system)
                        .after(velocity_system),
                ),
        )
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
            FrameRate::from_fps(move_fps),
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
                5,
                None,
                None,
            )),
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2 { x: 0.0, y: -1.0 }))
        .insert(player_animations)
        .insert(idle_animation.clone())
        .insert(Userinput {
            move_accelection: Vec2::ZERO,
        })
        .insert(AnimationState::default())
        .insert(BoxCollider {
            size: Vec2::new(32.0, 32.0),
        });

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("normal.png"),
                Vec2::new(95.0, 16.0),
                1,
                1,
                None,
                None,
            )),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..Default::default()
        })
        .insert(NormalBrick)
        .insert(BoxCollider {
            size: Vec2::new(95.0, 16.0),
        })
        .insert(Velocity(Vec2 { x: 0.0, y: 1.0 }));
}

fn animate_system(
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
struct Flying {}

#[derive(Component)]
struct Damaged {}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Userinput {
    move_accelection: Vec2,
}

fn userinput_system(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Userinput>) {
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

fn player_controller_system(mut player_query: Query<(&mut Velocity, &Userinput)>) {
    for (mut velocity, userinput) in player_query.iter_mut() {
        let move_speed = 4.0;
        velocity.x = move_speed * userinput.move_accelection.x;
    }
}

fn animate_player_system(
    mut player_query: Query<
        (
            &mut Animation,
            &mut AnimationState,
            &Userinput,
            &PlayerAnimations,
            Option<&Flying>,
            Option<&Damaged>,
        ),
        With<Player>,
    >,
) {
    for (mut animation, mut animation_state, userinput, animations, flying, damaged) in
        player_query.iter_mut()
    {
        let is_flying = match flying {
            Some(_) => true,
            _ => false,
        };

        let is_damaged = match damaged {
            Some(_) => true,
            _ => false,
        };

        let next_animation = get_next_animation(
            animation.clone(),
            animations.clone(),
            userinput.move_accelection,
            is_flying,
            is_damaged,
        );
        if next_animation != *animation {
            animation_state.reset();
            animation.clone_from(&next_animation);
        }
    }
}

fn get_next_animation(
    current_animation: Animation,
    animations: PlayerAnimations,
    direction: Vec2,
    is_flying: bool,
    is_damaged: bool,
) -> Animation {
    if direction.x > 0.0 {
        let next_animation = if is_damaged && is_flying {
            &animations.fly_hurt_right_run
        } else if is_damaged {
            &animations.hurt_right_run
        } else if is_flying {
            &animations.fly_right_run
        } else {
            &animations.right_run
        };
        return next_animation.clone();
    }
    if direction.x < 0.0 {
        let next_animation = if is_damaged && is_flying {
            &animations.fly_hurt_left_run
        } else if is_damaged {
            &animations.hurt_left_run
        } else if is_flying {
            &animations.fly_left_run
        } else {
            &animations.left_run
        };
        return next_animation.clone();
    }
    if direction == Vec2::ZERO {
        let next_animation = if is_damaged && is_flying {
            &animations.fly_hurt_idle
        } else if is_damaged {
            &animations.hurt_idle
        } else if is_flying {
            &animations.fly_idle
        } else {
            &animations.idle
        };
        return next_animation.clone();
    }
    return current_animation;
}

fn leave_grounded_system(
    mut commands: Commands,
    grounded_query: Query<(Entity, &Velocity), (With<Grounded>, With<Player>)>,
) {
    for (entity, velocity) in grounded_query.iter() {
        if velocity.y != 0.0 {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn enter_grounded_system(
    mut commands: Commands,
    no_grounded_query: Query<(Entity, &Velocity), (Without<Grounded>, With<Player>)>,
) {
    for (entity, velocity) in no_grounded_query.iter() {
        if velocity.y == 0.0 {
            commands.entity(entity).insert(Grounded {});
        }
    }
}

fn enter_flying_system(
    mut commands: Commands,
    no_flying_query: Query<(Entity, &Velocity), (Without<Flying>, With<Player>)>,
) {
    for (entity, velocity) in no_flying_query.iter() {
        if velocity.y != 0.0 {
            commands.entity(entity).insert(Flying {});
        }
    }
}

fn leave_flying_system(
    mut commands: Commands,
    flying_query: Query<(Entity, &Velocity), (With<Flying>, With<Player>)>,
) {
    for (entity, velocity) in flying_query.iter() {
        if velocity.y == 0.0 {
            commands.entity(entity).remove::<Flying>();
        }
    }
}

fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.extend(0.);
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &BoxCollider, &mut Velocity), With<Player>>,
    collider_query: Query<
        (Entity, &Transform, &BoxCollider, Option<&NormalBrick>),
        Without<Player>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut player_transform, player_collider, mut player_velocity) = player_query.single_mut();
    let player_size = player_collider.size;

    player_velocity.y = -1.0;

    for (_, transform, collider, maybe_normal_brick) in collider_query.iter() {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            collider.size,
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            if let Some(_) = maybe_normal_brick {
                match collision {
                    Collision::Left => {
                        player_velocity.x = 0.0;
                    }
                    Collision::Right => {
                        player_velocity.x = 0.0;
                    }
                    Collision::Top => {
                        player_velocity.y = 0.0;
                        player_transform.translation.y = transform.translation.y
                            + (collider.size.y / 2.0)
                            + (player_collider.size.y / 2.0);
                    }
                    Collision::Bottom => {
                        player_velocity.y = 0.0;
                    }
                    Collision::Inside => {
                        player_velocity.y = 0.0;
                    }
                }
            }
        }
    }
}
