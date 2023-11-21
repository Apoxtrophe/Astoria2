use bevy::render::color::Color;

//PLAYER
pub const PLAYER_HEIGHT: f32 = 2.0;
pub const PLAYER_WIDTH: f32 = 1.0;
pub const PLAYER_OFFSET: f32 = 0.01;
pub const AUTOSTEP_HEIGHT: f32 = 0.3;
pub const AUTOSTEP_WIDTH: f32 = 0.5; 

//WORLD
pub const WORLD_SIZE: f32 = 20.0;

//MOUSE
pub const MOUSE_SENSITIVITY: f32 = 0.05;
pub const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1; // Slightly less than 90 degrees
pub const MIN_PITCH: f32 = -std::f32::consts::FRAC_PI_2 + 0.1; // Slightly more than -90 degrees

//SCREEN
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

//CURSOR
pub const CURSOR_DISTANCE: f32 = 0.5;
pub const CURSOR_RADIUS: f32 = 0.01;
pub const CURSOR_COLOR: Color = Color::ANTIQUE_WHITE;
