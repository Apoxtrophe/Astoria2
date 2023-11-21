mod config;
use config::*;

mod components;
use components::*;

mod window_setup;
use window_setup::*;

mod game_systems;
use game_systems::*;

use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(WORLD_SIZE).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    

}

