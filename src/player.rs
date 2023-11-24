use super::config;
use config::*;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
};
use bevy::window::PrimaryWindow;

use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use bevy_rapier3d::control::{CharacterAutostep, CharacterLength};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterControllerOutput;


// Player Initialization System
pub fn player_initialization_system(mut commands: Commands) {
    commands.spawn(TransformBundle::default())
        .with_children(|parent| {
            spawn_camera(parent);
        })
        .insert(Collider::cuboid(PLAYER_WIDTH, PLAYER_WIDTH, PLAYER_HEIGHT))
        .insert(RigidBody::Dynamic) // Make the player's body dynamic
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
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

// Function to spawn the camera
fn spawn_camera(parent: &mut ChildBuilder) {
    parent.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, PLAYER_HEIGHT, 0.0),
        ..default()
    })
    .insert(AtmosphereCamera::default());
}

// Player Update System
pub fn player_update_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Camera)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut current_pitch: Local<f32>,
) {
    let movement_direction = get_movement_direction(&keyboard_input);
    let speed = get_player_speed(&keyboard_input);

    let total_mouse_motion = aggregate_mouse_motion(&mut mouse_motion_events);

    for (mut transform, _camera) in query.iter_mut() {
        process_player_movement(&mut transform, movement_direction, speed, time.delta_seconds());
        process_mouse_motion(&mut transform, &mut current_pitch, total_mouse_motion);
    }

    center_cursor(&mut windows);
}

// Function to get movement direction based on keyboard input
fn get_movement_direction(keyboard_input: &Res<Input<KeyCode>>) -> Vec3 {
    Vec3::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32) as f32,
        0.0,
        (keyboard_input.pressed(KeyCode::S) as i32 - keyboard_input.pressed(KeyCode::W) as i32) as f32,
    )
}

// Function to get player speed
fn get_player_speed(keyboard_input: &Res<Input<KeyCode>>) -> f32 {
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_SPRINT_SPEED
    } else {
        PLAYER_WALK_SPEED
    }
}

// Function to aggregate total mouse motion
fn aggregate_mouse_motion(mouse_motion_events: &mut EventReader<MouseMotion>) -> Vec2 {
    let mut total_mouse_motion = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        total_mouse_motion += event.delta;
    }
    total_mouse_motion
}

// Function to process player movement
fn process_player_movement(transform: &mut Transform, movement_direction: Vec3, speed: f32, delta_time: f32) {
    let mut world_movement_direction = transform.rotation.mul_vec3(movement_direction);
    world_movement_direction.y = 0.0;
    if world_movement_direction.length_squared() > 0.0001 { // Adding a small threshold for floating-point inaccuracies
        let normalized_movement = world_movement_direction.normalize() * speed;
        transform.translation += normalized_movement * delta_time;
    }
}
// Function to process mouse motion
fn process_mouse_motion(transform: &mut Transform, current_pitch: &mut f32, total_mouse_motion: Vec2) {
    let new_pitch = (*current_pitch - (total_mouse_motion.y * MOUSE_SENSITIVITY)).clamp(MIN_PITCH, MAX_PITCH);
    let pitch_diff = new_pitch - *current_pitch;
    *current_pitch = new_pitch;

    let yaw_diff = -total_mouse_motion.x * MOUSE_SENSITIVITY;

    if pitch_diff != 0.0 || yaw_diff != 0.0 {
        let pitch = Quat::from_rotation_x(pitch_diff);
        let yaw = Quat::from_rotation_y(yaw_diff);
        transform.rotation = yaw * transform.rotation * pitch;
    }
}

// Function to center the cursor
fn center_cursor(windows: &mut Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;
    window.set_cursor_position(Some(Vec2::new(center_x, center_y)));
}


pub fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                  entity, output.effective_translation, output.grounded);
    }
}