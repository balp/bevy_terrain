use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn create_terrain() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices = vec![
        [-0.8660, 0.5000, 0f32],
        [0.8660, 0.5000, 0f32],
        [-1.0000, 0.0000, 0f32],
        [1.0000, 0.0000, 0f32],
        [-0.8660, -0.5000, 0f32],
        [0.8660, -0.5000, 0f32],
    ];
    let normals = vec![
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
    ];
    let uvs = vec![
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
    ];
    let indices = Indices::U16(vec![1, 0, 2, 3, 1, 2, 3, 2, 4, 3, 4, 5]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    mesh
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Terrain mesh
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(create_terrain()),
        material: materials.add(Color::rgb(0.1, 0.7, 0.3).into()),
        ..default()
    });
    // plane
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}