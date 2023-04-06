use bevy::{app::AppExit, prelude::*};

use crate::{
    components::{
        camera::MainCamera,
        ui::{MainMenuUi, OnlinePlayButton, QuitButton, SinglePlayButton, GLOBAL_STYLES},
    },
    constants::AppState,
    resources::{InGameSetting, UiAssets},
};

pub fn spawn_main_menu_ui_all(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((Camera2dBundle::default(), MainCamera {}));
    build_main_menu_ui(&mut commands, &ui_assets)
}

pub fn despawn_main_menu_ui_all(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUi>>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &camera_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn build_main_menu_ui(commands: &mut Commands, ui_assets: &Res<UiAssets>) {
    commands
        .spawn((
            MainMenuUi {},
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    gap: Size::new(Val::Px(0.0), Val::Px(16.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::GRAY),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    // builder.spawn(ImageBundle {
                    //     image: UiImage {
                    //         texture: ui_assets.,
                    //     },
                    //     ..default()
                    // });
                    builder.spawn(
                        TextBundle::from_section(
                            "NS-SHAFT",
                            TextStyle {
                                font: ui_assets.bold_font.clone(),
                                font_size: 70.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect {
                                bottom: Val::Px(16.0),
                                ..default()
                            },
                            ..default()
                        }),
                    );
                });

            builder
                .spawn((
                    ButtonBundle {
                        style: GLOBAL_STYLES.normal_button,
                        background_color: GLOBAL_STYLES.normal_button_color.into(),
                        ..default()
                    },
                    SinglePlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Single Play",
                        TextStyle {
                            font: ui_assets.bold_font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            builder
                .spawn((
                    ButtonBundle {
                        style: GLOBAL_STYLES.normal_button,
                        background_color: GLOBAL_STYLES.normal_button_color.into(),
                        ..default()
                    },
                    OnlinePlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Online Matching",
                        TextStyle {
                            font: ui_assets.bold_font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            builder
                .spawn((
                    ButtonBundle {
                        style: GLOBAL_STYLES.normal_button,
                        background_color: GLOBAL_STYLES.normal_button_color.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: ui_assets.bold_font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn interact_with_single_play_button(
    mut in_game_setting: ResMut<InGameSetting>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SinglePlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = GLOBAL_STYLES.pressed_button_color.into();
                in_game_setting.set_offline_1p();
                app_state_next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = GLOBAL_STYLES.hovered_button_color.into();
            }
            Interaction::None => {
                *background_color = GLOBAL_STYLES.normal_button_color.into();
            }
        }
    }
}

pub fn interact_with_online_play_button(
    mut in_game_setting: ResMut<InGameSetting>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OnlinePlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = GLOBAL_STYLES.pressed_button_color.into();
                in_game_setting.set_online_2p();
                app_state_next_state.set(AppState::Matchmaking);
            }
            Interaction::Hovered => {
                *background_color = GLOBAL_STYLES.hovered_button_color.into();
            }
            Interaction::None => {
                *background_color = GLOBAL_STYLES.normal_button_color.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = GLOBAL_STYLES.pressed_button_color.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = GLOBAL_STYLES.hovered_button_color.into();
            }
            Interaction::None => {
                *background_color = GLOBAL_STYLES.normal_button_color.into();
            }
        }
    }
}
