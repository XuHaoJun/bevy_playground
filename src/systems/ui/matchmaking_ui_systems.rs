use bevy::{prelude::*, time::Stopwatch};

use crate::{
    components::{camera::MainCamera, ui::*},
    constants::AppState,
    resources::UiAssets,
};

pub fn spawn_matchmaking_ui_all(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((Camera2dBundle::default(), MainCamera {}));
    commands.spawn(MatchingTimer {
        elapsed_timer: Stopwatch::new(),
    });
    commands
        .spawn((
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
            MatchmakingUi {},
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Matching...",
                TextStyle {
                    font: ui_assets.bold_font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ));
            builder.spawn((
                TextBundle::from_section(
                    " seconds elapsed",
                    TextStyle {
                        font: ui_assets.medium_font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
                MatchingElapsedText {},
            ));

            builder
                .spawn((
                    ButtonBundle {
                        style: GLOBAL_STYLES.normal_button_medium,
                        background_color: GLOBAL_STYLES.normal_button_color.into(),
                        ..default()
                    },
                    BackMainMenuButton {},
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Cancel",
                        TextStyle {
                            font: ui_assets.bold_font.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn despawn_matchmaking_ui_all(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            With<MatchmakingUi>,
            With<MainCamera>,
            With<MatchingTimer>,
            With<MatchingTimer>,
        )>,
    >,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_matching_elapsed_time(
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<MatchingElapsedText>>,
    mut timer_query: Query<&mut MatchingTimer>,
) {
    let mut timer = timer_query.single_mut();
    timer.elapsed_timer.tick(time.delta());
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "{} seconds elapsed",
            timer.elapsed_timer.elapsed_secs().trunc()
        )
    }
}

pub fn interact_with_back_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackMainMenuButton>),
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
