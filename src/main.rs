mod config;
use config::*;

mod components;
use components::*;

mod window_setup;
use window_setup::*;

mod game_systems;
use game_systems::*;

use bevy::prelude::*;
use bevy_atmosphere::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AtmospherePlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_window)
        .add_systems(Startup, build_camera)
        .add_systems(Update, update_system)
        .add_systems(Update, mouse_look_system)
        .add_systems(Update, read_result_system)
        .add_systems(Update, draw_cursor)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load(TEXTURE_PATH);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..default()
    });
    
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: WORLD_SIZE, subdivisions: SUBDIVISIONS})),
        material,
        ..default()
    });
    
    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
