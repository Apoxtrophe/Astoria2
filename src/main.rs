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
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::VertexFormat;
use bevy::render::mesh::VertexAttributeValues;

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
        //.add_systems(Update, draw_cursor)
        .add_systems(Update, center_cursor)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Load the texture
    let texture_handle = asset_server.load("landscape.png");

    // Create a material with the texture
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        ..Default::default()
    });

    // Create a plane mesh
    let plane_size = WORLD_SIZE; // Size of the plane
    let mut plane_mesh = Mesh::from(shape::Plane { size: plane_size, subdivisions:SUBDIVISIONS });

    if let Some(VertexAttributeValues::Float32x3(positions)) = plane_mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        let mut uvs = Vec::new();
        let tiling_factor = 1.0; // Adjust based on how much you want to tile
    
        for pos in positions {
            uvs.push([(pos[0] / plane_size * tiling_factor) % 1.0, (pos[1] / plane_size * tiling_factor) % 1.0]);
        }
    
        plane_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }

    // Spawn the plane entity
    commands.spawn(PbrBundle {
        mesh: meshes.add(plane_mesh),
        material,
        ..Default::default()
    });
}
