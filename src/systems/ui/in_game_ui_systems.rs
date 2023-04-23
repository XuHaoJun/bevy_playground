use std::vec;

use bevy::prelude::*;

use crate::{
    components::{
        player::{Health, Player, PlayerScore},
        ui::*,
    },
    constants::AppState,
    events::player_events::PlayerEnterDeadEvent,
    resources::{scoreboard::Scoreboard, InGameMode, InGameSetting, LocalPlayerHandle, UiAssets},
};

pub fn build_in_game_ui(
    commands: &mut Commands,
    ui_assets: &Res<UiAssets>,
    player_handles: Vec<usize>,
) {
    commands
        .spawn((
            InGameUi {},
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(32.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    gap: Size::new(Val::Px(16.0), Val::Px(0.0)),
                    padding: UiRect::new(Val::Px(8.0), Val::Px(16.0), Val::Px(0.0), Val::Px(0.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            for handle in player_handles {
                builder.spawn((
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("p{}: ", handle + 1),
                                    TextStyle {
                                        font: ui_assets.bold_font.clone(),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                TextSection::from_style(TextStyle {
                                    font: ui_assets.medium_font.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }),
                            ],
                            alignment: TextAlignment::Left,
                            ..default()
                        },
                        ..default()
                    },
                    PlayerHealthText { handle },
                ));
            }

            builder.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "score: ",
                                TextStyle {
                                    font: ui_assets.bold_font.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font: ui_assets.medium_font.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            }),
                        ],
                        alignment: TextAlignment::Left,
                        ..default()
                    },
                    ..default()
                },
                ScoreText {},
            ));
        });

    // build_in_game_result_menu(commands, ui_assets);
}

pub fn build_in_game_result_menu(commands: &mut Commands, ui_assets: &Res<UiAssets>) {
    commands
        .spawn((
            InGameResultMenuUi {},
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Px(360.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        gap: Size::new(Val::Px(16.0), Val::Px(16.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "score: ",
                                        TextStyle {
                                            font: ui_assets.bold_font.clone(),
                                            font_size: 52.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ),
                                    TextSection::from_style(TextStyle {
                                        font: ui_assets.medium_font.clone(),
                                        font_size: 52.0,
                                        color: Color::WHITE,
                                    }),
                                ],
                                alignment: TextAlignment::Left,
                                ..default()
                            },
                            ..default()
                        },
                        InGameResultMenuScoreText {},
                    ));

                    builder
                        .spawn(ButtonBundle {
                            style: GLOBAL_STYLES.normal_button,
                            background_color: GLOBAL_STYLES.normal_button_color.into(),
                            ..default()
                        })
                        .insert(InGameResultBackToMainMenuButton {})
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back To Title Menu",
                                TextStyle {
                                    font: ui_assets.bold_font.clone(),
                                    font_size: 28.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    builder
                        .spawn(ButtonBundle {
                            style: GLOBAL_STYLES.normal_button,
                            background_color: GLOBAL_STYLES.normal_button_color.into(),
                            ..default()
                        })
                        .insert(InGameResultPlayAgainButton {})
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play Again",
                                TextStyle {
                                    font: ui_assets.bold_font.clone(),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

pub fn despawn_in_game_result_menu(
    commands: &mut Commands,
    menu_query: Query<Entity, With<InGameResultMenuUi>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_in_game_result_menu_if_end(
    mut commands: Commands,
    mut player_dead_events: EventReader<PlayerEnterDeadEvent>,
    ui_assets: Res<UiAssets>,
    in_game_setting: Res<InGameSetting>,
    maybe_local_player_handle: Option<Res<LocalPlayerHandle>>,
    menu_query: Query<Entity, With<InGameResultMenuUi>>,
) {
    match in_game_setting.mode {
        InGameMode::Offline => {
            let has_dead_event = player_dead_events.len() > 0;
            let no_menu = menu_query.is_empty();
            if no_menu && has_dead_event {
                build_in_game_result_menu(&mut commands, &ui_assets)
            }
        }
        InGameMode::Online => {
            if let Some(local_player_handle) = maybe_local_player_handle {
                let has_dead_event = player_dead_events
                    .iter()
                    .find(|x| x.handle == local_player_handle.0)
                    .is_some();
                let no_menu = menu_query.is_empty();
                if no_menu && has_dead_event {
                    build_in_game_result_menu(&mut commands, &ui_assets)
                }
            }
        }
    }
}

pub fn update_score_text(
    scoreboard: Res<Scoreboard>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in &mut text_query {
        text.sections[1].value = format!("{}", scoreboard.score);
    }
}

pub fn update_health_text(
    player_query: Query<(&Player, &Health)>,
    mut text_query: Query<(&mut Text, &PlayerHealthText)>,
) {
    for (mut text, player_health_text) in &mut text_query {
        let health_opt = player_query
            .iter()
            .find(|(x, _)| x.handle == player_health_text.handle);
        if let Some((_, health)) = health_opt {
            text.sections[1].value = health.value.to_string();
        }
    }
}

pub fn update_game_result_score_text(
    in_game_setting: Res<InGameSetting>,
    maybe_local_player_handle: Option<Res<LocalPlayerHandle>>,
    player_query: Query<(&Player, &PlayerScore)>,
    mut text_query: Query<&mut Text, With<InGameResultMenuScoreText>>,
) {
    match in_game_setting.mode {
        InGameMode::Offline => {
            for (_, player_score) in player_query.iter() {
                for mut text in text_query.iter_mut() {
                    text.sections[1].value = player_score.score.to_string();
                }
            }
        }
        InGameMode::Online => {
            if let Some(local_player_handle) = maybe_local_player_handle {
                let maybe_found = player_query
                    .iter()
                    .find(|(x, _)| x.handle == local_player_handle.0);
                if let Some((_, player_score)) = maybe_found {
                    for mut text in text_query.iter_mut() {
                        text.sections[1].value = player_score.score.to_string();
                    }
                }
            }
        }
    }
}

pub fn interact_play_again_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<InGameResultPlayAgainButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = GLOBAL_STYLES.pressed_button_color.into();
                // app_state_next_state.set(AppState::MainMenu);
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

pub fn interact_back_to_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<InGameResultBackToMainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = GLOBAL_STYLES.pressed_button_color.into();
                app_state_next_state.set(AppState::MainMenu);
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
