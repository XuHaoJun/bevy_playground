use std::vec;

use bevy::prelude::*;

use crate::{
    components::{
        player::{Health, Player},
        ui::*,
    },
    resources::{scoreboard::Scoreboard, UiAssets},
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
