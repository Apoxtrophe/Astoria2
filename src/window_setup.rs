use super::config;
use config::*;

use bevy::prelude::*;
use bevy::window::{Window, PresentMode, CursorIcon, CursorGrabMode, WindowResolution};
use bevy::window::PrimaryWindow;

pub fn setup_window(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows.single_mut();
        window.title = "Logica".to_string();
        window.resolution = WindowResolution::new(SCREEN_WIDTH,SCREEN_HEIGHT);
        window.present_mode = PresentMode::AutoVsync;
        window.cursor.icon = CursorIcon::Crosshair;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
} 
