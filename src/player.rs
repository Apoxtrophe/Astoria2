use super::config;
use config::*;

use super::components;
use components::*;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
};
use bevy::window::PrimaryWindow;

use bevy_rapier3d::prelude::{Collider, KinematicCharacterController, RigidBody};
use bevy_rapier3d::control::{CharacterAutostep, CharacterLength};
use bevy_atmosphere::prelude::*;

pub fn player_initialization_system(mut commands: Commands) {
    commands.spawn(TransformBundle::default())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, PLAYER_HEIGHT, 0.0),
                ..default()
            }).insert(AtmosphereCamera::default());
        })
        .insert((
            RigidBody::KinematicPositionBased,
            Collider::cuboid(PLAYER_WIDTH, PLAYER_WIDTH, PLAYER_HEIGHT),
            KinematicCharacterController {
                offset: CharacterLength::Relative(PLAYER_OFFSET),
                up: Vec3::Z,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(AUTOSTEP_HEIGHT),
                    min_width: CharacterLength::Relative(AUTOSTEP_WIDTH),
                    include_dynamic_bodies: true,
                }),
                ..default()
            },
        ));
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
    ).normalize_or_zero() * if keyboard_input.pressed(KeyCode::ShiftLeft) { PLAYER_SPRINT_SPEED } else { PLAYER_WALK_SPEED };

    let total_mouse_motion = mouse_motion_events.iter().fold(Vec2::ZERO, |acc, event| acc + event.delta);
    let (yaw_diff, pitch_diff) = (-total_mouse_motion.x * MOUSE_SENSITIVITY, (*current_pitch - (total_mouse_motion.y * MOUSE_SENSITIVITY)).clamp(MIN_PITCH, MAX_PITCH) - *current_pitch);
    *current_pitch += pitch_diff;

    let window_center = {
        let window = windows.single();
        Vec2::new(window.width() / 2.0, window.height() / 2.0)
    };

    for (mut transform, _) in query.iter_mut() {
        transform.translation += transform.rotation.mul_vec3(movement_direction) * time.delta_seconds();
        transform.rotation *= Quat::from_rotation_y(yaw_diff) * Quat::from_rotation_x(pitch_diff);
    }

    let mut window = windows.single_mut();
    window.set_cursor_position(Some(window_center));
}