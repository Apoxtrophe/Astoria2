mod config;
use config::*;

use bevy::{
    ecs::query,
    input::mouse::MouseMotion,
    prelude::*,
    window::{Cursor, CursorGrabMode, PresentMode, PrimaryWindow, WindowResolution},
};

use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, KinematicCharacterControllerOutput, RigidBody};
use bevy_rapier3d::control::{CharacterAutostep, CharacterLength};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                cursor: Cursor { 
                    icon: (CursorIcon::Crosshair), 
                    visible: (false), 
                    grab_mode: (CursorGrabMode::Locked), 
                    hit_test: (true) },
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Startup, init_system)
        .add_systems(Update, update_system)
        .add_systems(Update, read_result_system)
        .add_systems(Update, mouse_look_system)
        .run();
}

#[derive(Component)]
struct Ground;

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, PLAYER_HEIGHT, 0.0),
        ..default()
    })
    .insert(KinematicCharacterController {
        // Your controller configuration here
        ..default()
    });
}

fn init_system(mut commands: Commands) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PLAYER_WIDTH, PLAYER_WIDTH,PLAYER_HEIGHT),
        KinematicCharacterController {
            offset: CharacterLength::Relative(0.01),  // Character offset
            up: Vec3::Z,  // Z as the up vector
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: true,
            }),
            ..default()
        },
    ));
}

fn update_system(mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(1.0, -0.5, 1.0));
    }
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                  entity, output.effective_translation, output.grounded);
    }
}

use bevy::prelude::*;

fn mouse_look_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Camera)>,
    mut current_pitch: Local<f32>, // Tracking the current pitch
) {
    for (mut transform, _camera) in query.iter_mut() {
        for event in mouse_motion_events.iter() {
            let sensitivity_x = MOUSE_SENSITIVITY;
            let sensitivity_y = MOUSE_SENSITIVITY;

            // Calculate new pitch
            let mut new_pitch = *current_pitch + (-event.delta.y * sensitivity_y);
            new_pitch = new_pitch.clamp(MIN_PITCH, MAX_PITCH);

            if new_pitch != *current_pitch {
                // Apply pitch rotation only if it's within the limits
                let pitch_diff = new_pitch - *current_pitch;
                let pitch = Quat::from_rotation_x(pitch_diff);
                transform.rotation = transform.rotation * pitch; // Rotate around local x axis

                *current_pitch = new_pitch; // Update the current pitch
            }

            // Yaw rotation
            let yaw = Quat::from_rotation_y(-event.delta.x * sensitivity_x);
            transform.rotation = yaw * transform.rotation; // Rotate around global y axis
        }
    }
}