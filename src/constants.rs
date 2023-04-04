#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BrickType {
    Normal,
    Fake,
    Nails,
    Conveyor,
    Spring,
}

pub const PHYSICS_DELTA: f64 = 1.0 / 60.0;

pub const IN_GAME_UI_APP_BAR_HEIGHT: f32 = 32.0;

pub const WALL_WIDTH: f32 = 18.0;
pub const WALL_HEIGHT: f32 = 960.0 - IN_GAME_UI_APP_BAR_HEIGHT;

pub const CELLING_WIDTH: f32 = 540.0 - (WALL_WIDTH * 2.0);
pub const CELLING_HEIGHT: f32 = 16.0;