use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct InGameUi {}

#[derive(Component)]
pub struct InGameResultMenuUi {}

#[derive(Component)]
pub struct InGameResultMenuScoreText {}

#[derive(Component)]
pub struct InGameResultBackToMainMenuButton {}

#[derive(Component)]
pub struct InGameResultPlayAgainButton {}

#[derive(Component)]
pub struct ScoreText {}

#[derive(Component, Default)]
pub struct PlayerHealthText {
    pub handle: usize,
}

#[derive(Component)]
pub struct MainMenuUi {}

#[derive(Component)]
pub struct SinglePlayButton {}

#[derive(Component)]
pub struct OnlinePlayButton {}

#[derive(Component)]
pub struct QuitButton {}

#[derive(Component)]
pub struct MatchmakingUi {}

#[derive(Component)]
pub struct MatchingElapsedText {}

#[derive(Component)]
pub struct MatchingTimer {
    pub elapsed_timer: Stopwatch,
}

#[derive(Component)]
pub struct BackMainMenuButton {}

pub struct Styles {
    pub normal_button: Style,
    pub normal_button_medium: Style,

    pub normal_button_color: Color,
    pub hovered_button_color: Color,
    pub pressed_button_color: Color,
}

pub const GLOBAL_STYLES: Styles = Styles {
    normal_button: Style {
        size: Size::new(Val::Px(220.0), Val::Px(80.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    },

    normal_button_medium: Style {
        size: Size::new(Val::Px(110.0), Val::Px(40.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    },

    normal_button_color: Color::rgb(0.15, 0.15, 0.15),
    hovered_button_color: Color::rgb(0.25, 0.25, 0.25),
    pressed_button_color: Color::rgb(0.35, 0.75, 0.35),
};
