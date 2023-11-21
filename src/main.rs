mod config;
use config::*;

mod components;
use components::*;

mod window_setup;
use window_setup::*;

mod game_systems;
use game_systems::*;

use bevy::prelude::*;

use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use bevy_rapier3d::control::{CharacterAutostep, CharacterLength};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_window)
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
    commands.spawn(TransformBundle::default())
        .with_children(|parent| {
            // Spawn the camera as a child of the character
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT, 0.0),
                ..default()
            });
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(PLAYER_WIDTH, PLAYER_WIDTH, PLAYER_HEIGHT))
        .insert(KinematicCharacterController {
            offset: CharacterLength::Relative(PLAYER_OFFSET),
            up: Vec3::Z,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(AUTOSTEP_HEIGHT),
                min_width: CharacterLength::Relative(AUTOSTEP_WIDTH),
                include_dynamic_bodies: true,
            }),
            ..default()
    });

}


