use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_cam)
        .run();
}

fn create_terrain() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices = vec![
        [0f32, -0.1, 0f32],
        [0f32, -0.1, 1f32],
        [0f32,  0.1, 2f32],
        [0f32,  0.1, 3f32],
        [0f32, -0.1, 4f32],
        [0f32, -0.1, 5f32],

        [1f32,  0.0, 0f32],
        [1f32,  0.1, 1f32],
        [1f32,  0.5, 2f32],
        [1f32,  0.5, 3f32],
        [1f32,  0.1, 4f32],
        [1f32,  0.0, 5f32],

        [2f32,  0.0, 0f32],
        [2f32,  0.1, 1f32],
        [2f32,  0.6, 2f32],
        [2f32,  0.6, 3f32],
        [2f32,  0.1, 4f32],
        [2f32,  0.0, 5f32],

        [3f32,  0.2, 0f32],
        [3f32,  0.1, 1f32],
        [3f32,  0.4, 2f32],
        [3f32,  0.4, 3f32],
        [3f32,  0.1, 4f32],
        [3f32,  0.2, 5f32],

        [4f32, -0.1, 0f32],
        [4f32,  0.2, 1f32],
        [4f32,  0.2, 2f32],
        [4f32,  0.3, 3f32],
        [4f32,  0.2, 4f32],
        [4f32, -0.1, 5f32],

        [5f32, -0.1, 0f32],
        [5f32, -0.1, 1f32],
        [5f32,  0.3, 2f32],
        [5f32,  0.3, 3f32],
        [5f32, -0.1, 4f32],
        [5f32, -0.1, 5f32],

    ];
    let normals = vec![
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
        [0f32, 0f32, 1f32],
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
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
        [0.0000, 0.0000],
    ];

    /*
          0  1  2  3  4  5
          6  7  8  9 10 11
         12 13 14 15 16 17
         18 19 20 21 22 23
         24 25 26 27 28 29
         30 31 32 33 34 35
     */
    let indices = Indices::U16(vec![
         0, 1, 7,  1, 2, 8,  2, 3, 9,  3, 4,10,  4, 5,11,
         0, 7, 6,  1, 8, 7,  2, 9, 6,  3,10, 9,  4,11,10,

         6, 7,13,  7, 8,14,  8, 9,15,  9,10,16, 10,11,17,
         6,13,12,  7,14,13,  8,15,14,  9,16,15, 10,17,16,

        12,13,19, 13,14,20, 14,15,21, 15,16,22, 16,17,23,
        12,19,18, 13,20,19, 14,21,20, 15,22,21, 16,23,22,

        18,19,25, 19,20,26, 20,21,27, 21,22,28, 22,23,29,
        18,25,24, 19,26,25, 20,27,26, 21,28,27, 22,29,28,

        24,25,31, 25,26,32, 26,27,33, 27,28,34, 28,29,35,
        24,31,30, 25,32,31, 26,33,32, 27,34,33, 28,35,34,



    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));
    mesh
}


fn rotate_cam(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let hsize = (5 / 2) as f32;
    for mut transform in query.iter_mut() {
        let alpha = (time.seconds_since_startup() as f32 / 2.);
        *transform = Transform::from_xyz(
            hsize + alpha.cos() * hsize,
            3.,
            hsize + alpha.sin() * hsize,
        )
            .looking_at(Vec3::new(0., 0., 0.), Vec3::Y);
    }
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
        transform: Transform::from_xyz(-2.5, 0.0, -2.5),
        ..default()
    });
    // water plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1500.0 })),
        material: materials.add(Color::rgba(0.1, 0.1, 0.7, 0.3).into()),
        transform: Transform::from_xyz(0.0, -0.0, 0.0),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
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