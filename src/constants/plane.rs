use bevy_iso3d_rts_cursor_plugin::Bounds2D;

pub const BOARD_SIZE_I: f32 = 3.;
pub const BOARD_SIZE_J: f32 = 3.;
// pub const BLOCK_SIZE_PIXELS: usize = 126;
pub const BLOCK_SIZE: f32 = 15.75;
pub const GAME_X_MIN: f32 = 0.0;
pub const GAME_Z_MIN: f32 = 0.0;
pub const GAME_X_MAX: f32 = 24.0;
pub const GAME_Z_MAX: f32 = 24.0;
pub const GAME_BOUNDS: Bounds2D = Bounds2D {
    min_x: GAME_X_MIN,
    min_z: GAME_Z_MIN,
    max_x: GAME_X_MAX,
    max_z: GAME_Z_MAX,
};
// pub const GAMEPLAY_TILE_PIXELS: f32 = BLOCK_SIZE_PIXELS as f32 / BLOCK_SIZE as f32;
// pub const BOARD_SIZE_IN_GAMEPLAY_PIXELS: f32 = GAMEPLAY_TILE_PIXELS * BOARD_SIZE_I as f32;
// pub const CENTER_IN_GAMEPLAY_PIXELS: f32 = BOARD_SIZE_IN_GAMEPLAY_PIXELS / 2.;
// pub const HALF_BLOCK: f32 = BLOCK_SIZE / 2.;

// pub const GAME_NORTH: Vec3 = Vec3::new(CENTER_IN_GAMEPLAY_PIXELS, GROUND_LEVEL, 0.);
