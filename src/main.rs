use benimator::FrameRate;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
    window::PresentMode,
};
use bevy_kira_audio::prelude::*;
use std::time::Duration;

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

#[derive(Component, Default)]
struct BoxCollider {
    size: Vec2,
    center: Vec2,
}

#[derive(Component)]
struct NormalBrick;

#[derive(Component)]
struct NailsBrick {}

#[derive(Component, Deref, DerefMut, Default)]
struct NailsTrigger(BoxCollider);

#[derive(Component)]
struct LastCollisions {
    entities: Vec<Entity>,
}

#[derive(Component, Reflect)]
struct DamagingTimer {
    timer: Timer,
}

impl Default for DamagingTimer {
    fn default() -> Self {
        DamagingTimer {
            timer: Timer::new(
                Duration::from_secs_f64(PHYSICS_DELTA * 40.0),
                TimerMode::Once,
            ),
        }
    }
}
#[derive(Resource, Deref, DerefMut)]
struct NormalHitSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
struct NailHitSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
struct FakeHitSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
struct DieSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
struct SpringHitSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
struct ConveyorHitSound(Handle<AudioSource>);

const PHYSICS_DELTA: f64 = 1.0 / 60.0;

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
                        resizable: false,
                        ..default()
                    },
                    ..default()
                }),
        )
        .register_type::<DamagingTimer>()
        .add_plugin(AudioPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
        .add_startup_system(setup_audio_resources)
        .add_startup_system(play_background_sound)
        .add_startup_system(spawn)
        .add_event::<CollisionEvent>()
        .add_system(animate_system)
        .add_system(animate_player_system.before(animate_system))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PHYSICS_DELTA))
                .with_system(userinput_system)
                .with_system(player_controller_system)
                .with_system(velocity_system)
                .with_system(enter_grounded_system.after(player_collision_system))
                .with_system(leave_grounded_system.after(player_collision_system))
                .with_system(enter_flying_system.after(player_collision_system))
                .with_system(leave_flying_system.after(player_collision_system))
                .with_system(
                    player_collision_system
                        .after(player_controller_system)
                        .after(velocity_system),
                )
                .with_system(damaging_timer_system)
                .with_system(player_nails_trigger_system.after(damaging_timer_system))
                .with_system(enter_dead_system.after(player_nails_trigger_system)),
        )
        .run();
}

fn setup_audio_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let dir = "sounds";
    commands.insert_resource(NormalHitSound(
        asset_server.load(format!("{dir}/normal.ogg")),
    ));
    commands.insert_resource(NailHitSound(asset_server.load(format!("{dir}/nail.ogg"))));
    commands.insert_resource(FakeHitSound(asset_server.load(format!("{dir}/fake.ogg"))));
    commands.insert_resource(DieSound(asset_server.load(format!("{dir}/die.ogg"))));
    commands.insert_resource(SpringHitSound(
        asset_server.load(format!("{dir}/spring.ogg")),
    ));
    commands.insert_resource(ConveyorHitSound(
        asset_server.load(format!("{dir}/conveyor.ogg")),
    ));
}

fn play_background_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let dir = "sounds/background";
    audio
        .play(asset_server.load(format!("{dir}/run_amok.ogg")))
        .with_volume(0.2)
        .looped();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Don't forget the camera ;-)
    commands.spawn(Camera2dBundle::default());

    let move_duration = Duration::from_millis(333);
    // Create an animation
    let player_animations = PlayerAnimations {
        idle: Animation(benimator::Animation::from_indices(
            [8],
            FrameRate::from_total_duration(move_duration),
        )),
        left_run: Animation(benimator::Animation::from_indices(
            0..=3,
            FrameRate::from_total_duration(move_duration),
        )),
        right_run: Animation(benimator::Animation::from_indices(
            9..=12,
            FrameRate::from_total_duration(move_duration.clone()),
        )),

        hurt_idle: Animation(benimator::Animation::from_indices(
            [17, 8],
            FrameRate::from_total_duration(move_duration.clone()),
        )),
        hurt_left_run: Animation(benimator::Animation::from_indices(
            4..=7,
            FrameRate::from_total_duration(move_duration.clone()),
        )),
        hurt_right_run: Animation(benimator::Animation::from_indices(
            13..=16,
            FrameRate::from_total_duration(move_duration.clone()),
        )),

        fly_idle: Animation(benimator::Animation::from_indices(
            36..=39,
            FrameRate::from_total_duration(move_duration.clone()),
        )),
        fly_left_run: Animation(benimator::Animation::from_indices(
            18..=21,
            FrameRate::from_total_duration(move_duration.clone()),
        )),
        fly_right_run: Animation(benimator::Animation::from_indices(
            27..=30,
            FrameRate::from_total_duration(move_duration.clone()),
        )),

        fly_hurt_idle: Animation(benimator::Animation::from_indices(
            40..=43,
            FrameRate::from_total_duration(move_duration.clone()),
        )),
        fly_hurt_left_run: Animation(benimator::Animation::from_indices(
            22..=25,
            FrameRate::from_total_duration(move_duration.clone()),
        )),
        fly_hurt_right_run: Animation(benimator::Animation::from_indices(
            28..=31,
            FrameRate::from_total_duration(move_duration.clone()),
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
            transform: Transform::from_xyz(0.0, 200.0, 2.0),
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
            ..default()
        })
        .insert(LastCollisions {
            entities: Vec::new(),
        })
        .insert(Health::new_player_health());

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
        .insert(NormalBrick {})
        .insert(BoxCollider {
            size: Vec2::new(95.0, 16.0),
            ..default()
        })
        .insert(Velocity(Vec2 { x: 0.0, y: 1.0 }));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("nails.png"),
                Vec2::new(96.0, 31.0),
                1,
                1,
                None,
                None,
            )),
            transform: Transform::from_xyz(100.0, -200.0, 0.0),
            ..Default::default()
        })
        .insert(NailsBrick {})
        .insert(BoxCollider {
            size: Vec2::new(96.0, 16.0),
            center: Vec2::new(0.0, -15.5),
            ..default()
        })
        .insert(NailsTrigger {
            0: BoxCollider {
                size: Vec2::new(96.0, 16.0),
                center: Vec2::new(0.0, 15.5),
                ..default()
            },
        })
        .insert(Velocity(Vec2 { x: 0.0, y: 1.0 }));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("wall.png"),
                Vec2::new(18.0, 400.0),
                1,
                1,
                None,
                None,
            )),
            transform: Transform::from_xyz(720.0 / 2.0 + -9.0, 1280.0 / 2.0 - 300.0, 0.0),
            ..Default::default()
        })
        .insert(NailsBrick {})
        .insert(BoxCollider {
            size: Vec2::new(18.0, 400.0),
            ..default()
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("wall.png"),
                Vec2::new(18.0, 400.0),
                1,
                1,
                None,
                None,
            )),
            transform: Transform::from_xyz(720.0 / 2.0 + -9.0, 1280.0 / 2.0 - 300.0 - 200.0, 0.0),
            ..Default::default()
        })
        .insert(NailsBrick {})
        .insert(BoxCollider {
            size: Vec2::new(18.0, 400.0),
            ..default()
        });
    // commands
    //     .spawn(SpriteSheetBundle {
    //         texture_atlas: textures.add(TextureAtlas::from_grid(
    //             asset_server.load("wall.png"),
    //             Vec2::new(18.0, 400.0),
    //             1,
    //             1,
    //             None,
    //             None,
    //         )),
    //         transform: Transform::from_xyz(720.0 / 2.0 + -9.0, (1280.0 / 2.0) - 400.0, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(NailsBrick {})
    //     .insert(BoxCollider {
    //         size: Vec2::new(18.0, 400.0),
    //         ..default()
    //     });
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
struct Health {
    value: i32,
    max: i32,
}

impl Health {
    fn new_player_health() -> Self {
        Health { value: 10, max: 10 }
    }

    fn clamp(&self, input: i32) -> i32 {
        let min = 0;
        let max = self.max;
        if input > max {
            max
        } else if input < min {
            min
        } else {
            input
        }
    }
}

#[derive(Component)]
struct Grounded {}

#[derive(Component)]
struct Flying {}

#[derive(Component)]
struct Damaging {}

#[derive(Component)]
struct Dead {}

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
            Option<&Damaging>,
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

        let is_damaging = match damaged {
            Some(_) => true,
            _ => false,
        };

        let next_animation = get_next_animation(
            animation.clone(),
            animations.clone(),
            userinput.move_accelection,
            is_flying,
            is_damaging,
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
    is_damaging: bool,
) -> Animation {
    if direction.x > 0.0 {
        let next_animation = if is_damaging && is_flying {
            &animations.fly_hurt_right_run
        } else if is_damaging {
            &animations.hurt_right_run
        } else if is_flying {
            &animations.fly_right_run
        } else {
            &animations.right_run
        };
        return next_animation.clone();
    }
    if direction.x < 0.0 {
        let next_animation = if is_damaging && is_flying {
            &animations.fly_hurt_left_run
        } else if is_damaging {
            &animations.hurt_left_run
        } else if is_flying {
            &animations.fly_left_run
        } else {
            &animations.left_run
        };
        return next_animation.clone();
    }
    if direction == Vec2::ZERO {
        let next_animation = if is_damaging && is_flying {
            &animations.fly_hurt_idle
        } else if is_damaging {
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

fn player_collision_system(
    mut player_query: Query<
        (
            &mut Transform,
            &BoxCollider,
            &mut Velocity,
            &mut LastCollisions,
        ),
        With<Player>,
    >,
    collider_query: Query<
        (Entity, &Transform, &BoxCollider, Option<&NormalBrick>),
        Without<Player>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
    normal_hit_sound: Res<NormalHitSound>,
    audio: Res<Audio>,
) {
    for (mut player_transform, player_collider, mut player_velocity, mut player_last_collisions) in
        player_query.iter_mut()
    {
        player_velocity.y = -1.0;

        let player_translation =
            get_collider_translation(player_transform.as_ref(), player_collider);
        let player_size = get_collider_size(player_transform.as_ref(), player_collider);

        let mut collision_entities: Vec<Entity> = Vec::new();

        for (other_entity, transform, collider, normal_brick) in collider_query.iter() {
            let collider_translation = get_collider_translation(transform, collider);
            let collider_size = get_collider_size(transform, collider);

            let collision = collide(
                player_translation,
                player_size,
                collider_translation,
                collider_size,
            );

            if let Some(collision) = collision {
                collision_entities.push(other_entity);

                collision_events.send_default();

                match collision {
                    Collision::Left => {
                        if player_velocity.x > 0.0 {
                            player_velocity.x = 0.0;
                            player_transform.translation.x = collider_translation.x
                                - (collider.size.x / 2.0)
                                - (player_collider.size.x / 2.0);
                        }
                    }
                    Collision::Right => {
                        if player_velocity.x < 0.0 {
                            player_velocity.x = 0.0;
                            player_transform.translation.x = collider_translation.x
                                + (collider.size.x / 2.0)
                                + (player_collider.size.x / 2.0);
                        }
                    }
                    Collision::Top => {
                        if player_velocity.y < 0.0 {
                            player_velocity.y = 0.0;
                            player_transform.translation.y = collider_translation.y
                                + (collider.size.y / 2.0)
                                + (player_collider.size.y / 2.0);
                        }
                    }
                    Collision::Bottom => {
                        if player_velocity.y > 0.0 {
                            player_velocity.y = 0.0;
                            player_transform.translation.y = collider_translation.y
                                - (collider.size.y / 2.0)
                                - (player_collider.size.y / 2.0);
                        }
                    }
                    Collision::Inside => {}
                }

                if let Some(_) = normal_brick {
                    match collision {
                        Collision::Top => {
                            let should_play: bool = !{
                                let mut found = false;
                                for e in player_last_collisions.entities.clone().into_iter() {
                                    if e == other_entity {
                                        found = true;
                                        break;
                                    }
                                }
                                found
                            };
                            if should_play {
                                audio.play(normal_hit_sound.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        player_last_collisions.entities = collision_entities;
    }
}

fn get_collider_translation(transform: &Transform, collider: &BoxCollider) -> Vec3 {
    transform.translation + collider.center.extend(0.0)
}

fn get_collider_size(transform: &Transform, collider: &BoxCollider) -> Vec2 {
    collider.size * transform.scale.truncate()
}

fn enter_dead_system(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), Without<Dead>>,
) {
    for (entity, health) in health_query.iter() {
        if health.value <= 0 {
            commands.entity(entity).insert(Dead {});
        }
    }
}

fn player_nails_trigger_system(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &BoxCollider, &mut Health),
        (With<Player>, Without<Damaging>, Without<Dead>),
    >,
    collider_query: Query<(&Transform, &NailsTrigger)>,
    nail_hit_sound: Res<NailHitSound>,
    audio: Res<Audio>,
) {
    for (player_entity, player_transform, player_collider, mut player_health) in
        player_query.iter_mut()
    {
        let player_translation = get_collider_translation(player_transform, player_collider);
        let player_size = get_collider_size(player_transform, player_collider);

        for (transform, collider) in collider_query.iter() {
            let collider_translation = get_collider_translation(transform, collider);
            let collider_size = get_collider_size(transform, collider);

            let collision = collide(
                player_translation,
                player_size,
                collider_translation,
                collider_size,
            );

            if let Some(_) = collision {
                player_health.value = player_health.clamp(player_health.value - 1);
                commands
                    .entity(player_entity)
                    .insert(DamagingTimer::default())
                    .insert(Damaging {});
                audio.play(nail_hit_sound.clone());
            }
        }
    }
}

fn damaging_timer_system(
    mut commands: Commands,
    mut timer_query: Query<(Entity, &mut DamagingTimer), With<Damaging>>,
) {
    for (entity, mut cooldown) in timer_query.iter_mut() {
        cooldown.timer.tick(Duration::from_secs_f64(PHYSICS_DELTA));
        if cooldown.timer.finished() {
            commands
                .entity(entity)
                .remove::<(Damaging, DamagingTimer)>();
        }
    }
}
