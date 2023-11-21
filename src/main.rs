mod config;
use config::*;

mod window_setup;
use window_setup::*;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
};
use bevy::window::PrimaryWindow;

use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, KinematicCharacterControllerOutput, RigidBody};
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

#[derive(Component)]
struct Velocity(Vec3);

fn update_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    let mut movement_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        movement_direction -= Vec3::Z; // Forward
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement_direction += Vec3::Z; // Backward
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement_direction -= Vec3::X; // Left
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement_direction += Vec3::X; // Right
    }

    let speed = 5.0; // Set your desired speed

    for (mut transform, _camera) in query.iter_mut() {
        // Transform the movement direction from the camera's local space to world space
        let mut world_movement_direction = transform.rotation.mul_vec3(movement_direction);
        world_movement_direction.y = 0.0;
        if world_movement_direction.length() > 0.0 {
            let normalized_movement = world_movement_direction.normalize() * speed;
            transform.translation += normalized_movement * time.delta_seconds();
        }
    }
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                  entity, output.effective_translation, output.grounded);
    }
}

fn mouse_look_system(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Camera)>,
    mut current_pitch: Local<f32>, // Tracking the current pitch
) {
    let delta_seconds = time.delta_seconds();
    for (mut transform, _camera) in query.iter_mut() {
        for event in mouse_motion_events.read() {
            let sensitivity_x = MOUSE_SENSITIVITY * delta_seconds;
            let sensitivity_y = MOUSE_SENSITIVITY * delta_seconds;

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

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = windows.single_mut();
    let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, center) else {
        return;
    };
    // Calculate the position of the cursor along the ray.
    let cursor_position = ray.origin + ray.direction * CURSOR_DISTANCE;
    // Calculate the normal. In this case, it's the negative of the camera's direction.
    let cursor_normal = -ray.direction;
    // Draw a circle at the cursor position.
    gizmos.circle(cursor_position, cursor_normal, CURSOR_RADIUS, CURSOR_COLOR);
}
 