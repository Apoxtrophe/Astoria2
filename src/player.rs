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

pub fn player_initialization_system(
    mut commands: Commands,
) {
    // Build Camera
    commands.spawn(TransformBundle::default())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT, 0.0),
                ..default()
            })
            .insert(AtmosphereCamera::default());
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

pub fn player_update_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Camera)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut current_pitch: Local<f32>,
) {
    let movement_direction = Vec3::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32) as f32,
        0.0,
        (keyboard_input.pressed(KeyCode::S) as i32 - keyboard_input.pressed(KeyCode::W) as i32) as f32,
    );

    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_SPRINT_SPEED
    } else {
        PLAYER_WALK_SPEED
    };

    // Aggregate total mouse motion for the frame
    let mut total_mouse_motion = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        total_mouse_motion += event.delta;
    }

    for (mut transform, _camera) in query.iter_mut() {
        let mut world_movement_direction = transform.rotation.mul_vec3(movement_direction);
        world_movement_direction.y = 0.0;
        let target_position = if world_movement_direction.length_squared() > 0.0 {
            let normalized_movement = world_movement_direction.normalize_or_zero() * speed;
            transform.translation + normalized_movement * time.delta_seconds()
        } else {
            transform.translation
        };
        transform.translation = transform.translation.lerp(target_position, time.delta_seconds() * MOVEMENT_INTERPOLATION_FACTOR);

        // Apply aggregated mouse motion
        let new_pitch = (*current_pitch - (total_mouse_motion.y * MOUSE_SENSITIVITY)).clamp(MIN_PITCH, MAX_PITCH);
        let pitch_diff = new_pitch - *current_pitch;
        *current_pitch = new_pitch;

        let yaw_diff = -total_mouse_motion.x * MOUSE_SENSITIVITY;

        if pitch_diff != 0.0 || yaw_diff != 0.0 {
            let pitch = Quat::from_rotation_x(pitch_diff);
            let yaw = Quat::from_rotation_y(yaw_diff);
            let target_rotation = yaw * transform.rotation * pitch;
            transform.rotation = transform.rotation.lerp(target_rotation, time.delta_seconds() * ROTATION_INTERPOLATION_FACTOR);
        }
        
    }
    // Center Cursor (dynamic part)
    let mut window = windows.single_mut();
    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;
    window.set_cursor_position(Some(Vec2::new(center_x, center_y)));
}
