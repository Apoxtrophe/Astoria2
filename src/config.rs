//PLAYER
pub const PLAYER_HEIGHT: f32 = 2.0;
pub const PLAYER_WIDTH: f32 = 1.0;
pub const PLAYER_OFFSET: f32 = 0.01;
pub const AUTOSTEP_HEIGHT: f32 = 0.3;
pub const AUTOSTEP_WIDTH: f32 = 0.5; 
pub const PLAYER_WALK_SPEED: f32 = 6.0;
pub const PLAYER_SPRINT_SPEED: f32 = 12.0;
pub const GRAVITY: f32 = -9.81; // Gravity force
pub const JUMP_FORCE: f32 = 100.0; // Force applied when jumping

//WORLD
pub const WORLD_SIZE: f32 = 100.0;
pub const SUBDIVISIONS: u32 = 10;
pub const TEXTURE_PATH: &str = "landscape.png";

//MOUSE
pub const MOUSE_SENSITIVITY: f32 = 0.0002;
pub const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1; // Slightly less than 90 degrees
pub const MIN_PITCH: f32 = -std::f32::consts::FRAC_PI_2 + 0.1; // Slightly more than -90 degrees

//SCREEN
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;