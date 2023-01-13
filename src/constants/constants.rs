use bevy::prelude::*;

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const GAME_TITLE: &str = "Rusty Tank Game";
pub const START_X_POX: f32 = 1080.0;
pub const START_Y_POX: f32 = 0.0;

pub const CAM_ORIGIN_X: f32 = 15.0;
pub const CAM_ORIGIN_Y: f32 = 15.0;
pub const CAM_ORIGIN_Z: f32 = 15.0;

pub const GROUND_LEVEL: f32 = 7.75;

pub const SELECTION_Y1: f32 = GROUND_LEVEL;
pub const SELECTION_Y2: f32 = GROUND_LEVEL + 0.2;

pub const BOUNDING_BOX_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.33);
pub const SELECTED_AREA_BOX_COLOR: Color = Color::rgba(1.0, 1.0, 0.0, 0.33);

pub const THICKNESS_OF_SELECTION_LINES: f32 = 0.05;
pub const AFTER_SELECTION_BLINK_DURATION: f32 = 0.08;
pub const SELECTION_CONFIRMED_BOX_RADIUS: f32 = 0.1;
