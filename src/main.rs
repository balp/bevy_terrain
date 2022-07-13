
use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use image::io::Reader as ImageReader;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(WireframePlugin)
        .add_system(rotate_cam)
        .run();
}

struct HeightMap {
    width: u16,
    max_height: f64,
    min_height: f64,
    map: Vec<f64>,
}

fn load_height_map(path: &str) -> Result<HeightMap, &str> {
    if let Ok(reader) = ImageReader::open(path) {
        if let Ok(img) = reader.decode() {
            //dbg!("{}", &img);
            if img.width() == img.height() { // Only handle square height maps for now
                if let Some(fimg) = img.as_luma16() {
                    let mut result = HeightMap {
                        width: img.width() as u16,
                        max_height: f64::MIN,
                        min_height: f64::MAX,
                        map: vec![],
                    };
                    let mut buffer: Vec<f64> = Vec::new();
                    for pixel in fimg.pixels() {
                        let height = pixel[0] as f64;
                        buffer.push(height);
                        if height > result.max_height {
                            result.max_height = height;
                        }
                        if height < result.min_height {
                            result.min_height = height;
                        }
                    }
                    result.map = buffer;
                    return Ok(result)
                } else {
                    return Err("Unable to convert image to luma16")
                }
            } else {
                return Err("Not a square image")
            }
        } else {
            return Err("Can't read image")
        }
    } else {
        return Err("Can't open file")
    }
}

fn make_height_map() -> HeightMap {
    // Terrain mesh
    let _map_6x6 = HeightMap {
        width: 6,
        max_height: 0.3,
        min_height: -0.1,
        map: vec![
            -0.1, 0.1, 0.1, 0.1, 0.1, 0.1,
            -0.1, 0.2, 0.2, 0.2, 0.2, 0.1,
            -0.1, 0.2, 0.1, 0.1, 0.3, 0.1,
            -0.1, 0.2, -0.1, 0.1, 0.3, 0.1,
            -0.1, 0.2, 0.2, 0.2, 0.3, 0.1,
            -0.1, 0.1, 0.1, 0.1, 0.1, 0.1,
        ],
    };
    let _map_3x3 = HeightMap {
        width: 3,
        max_height: 8.,
        min_height: 0.,
        map: vec![
            0., 1., 0.,
            3., 4., 5.,
            6., 5., 8.,
        ]
    };
    let _map_2x2 = HeightMap {
        width: 2,
        max_height: 0.2,
        min_height: -0.1,
        map: vec![
            -0.1, 0.1,
            -0.2, 0.2,
        ]
    };
    let _map_4x4 = HeightMap {
        width: 4,
        max_height: 1.0,
        min_height: 0.,
        map: vec![
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
        ]
    };
    let _map_5x5 = HeightMap {
        width: 5,
        max_height: 0.1,
        min_height: 0.1,
        map: vec![
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
            0.1, 0.1, 0.1, 0.1, 0.1,
        ]
    };
    _map_6x6
}

fn create_simple_terrain(height_map: &HeightMap) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let heights = height_map;
    let size = height_map.width;
    let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(heights.map.len());
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(heights.map.len());
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(heights.map.len());
    let mut indices_vector: Vec<u16> = Vec::new();

    let mut x = 0;
    let mut z = 0;

    for height in &heights.map {
        vertices.push([x as f32, *height as f32, z as f32]);
        normals.push([0f32, 0f32, 1f32]);
        uvs.push([x as f32, z as f32]);
        x += 1;
        if x == size {
            x = 0;
            z += 1;
        }
    }

    let l = (heights.map.len() - size as usize - 1) as u16;
    for a in 0..l {
        if !(a % size as u16 == size as u16 - 1) {
            indices_vector.push(a);
            indices_vector.push(a + 1u16 + (size as u16));
            indices_vector.push(a + 1u16);

            indices_vector.push(a);
            indices_vector.push(a + 0u16 + (size as u16));
            indices_vector.push(a + 1u16 + (size as u16));
        }
    }
    let indices = Indices::U16(indices_vector);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(indices));

    mesh
}


fn rotate_cam(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let hsize = (5 / 2) as f32;
    for mut transform in query.iter_mut() {
        let alpha = time.seconds_since_startup() as f32 / 2.;
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
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;
    // water plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1500.0 })),
        material: materials.add(Color::rgba(0.1, 0.1, 0.7, 0.2).into()),
        transform: Transform::from_xyz(0.0, 0., 0.0),
        ..default()
    });

    let height_map = make_height_map();
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(create_simple_terrain(&height_map)),
        material: materials.add(Color::rgb(0.1, 0.7, 0.3).into()),
        transform: Transform::from_xyz(-(height_map.width as f32 / 2.),
                                       0.,
                                       -(height_map.width as f32 / 2.)),
        ..default()
    })
        .insert(Wireframe)
    ;
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
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

#[cfg(test)]
mod tests {
    use crate::{create_simple_terrain, HeightMap, load_height_map};
    use bevy::prelude::*;
    use bevy::render::mesh::Indices::U16;
    use bevy::render::mesh::VertexAttributeValues;


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn terrain_from_file() {
        let map = load_height_map("resources/test/402.png");
        assert!(map.is_ok());
        assert_eq!(map.unwrap().width, 300);
    }

    #[test]
    /// Make sure a simple 2x2 height map generates correct
    ///
    /// The test sets up a height map like this:
    ///      0  1
    ///      -  -
    ///  0|  0  1
    ///  1|  2  3
    ///
    /// the verifies the resulting positions and indices for the mesh
    fn make_simple_terrain_2x2() {
        let _map_2x2 = HeightMap {
            width: 2,
            max_height: 3.,
            min_height: 0.,
            map: vec![
                0.,
                1.,
                2.,
                3.,
            ],
        };
        let mesh = create_simple_terrain(&_map_2x2);
        assert_eq!(mesh.count_vertices(), 4);
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        dbg!("{}", positions);
        let expected_pos = VertexAttributeValues::from(vec![
            [0., 0., 0.],
            [1., 1., 0.],
            [0., 2., 1.],
            [1., 3., 1.],
        ]);
        assert_eq!(positions.get_bytes(), expected_pos.get_bytes());

        if let U16(bytes) = mesh.indices().unwrap() {
            dbg!("{}", bytes);
            assert_eq!(*bytes, vec![
                0u16, 3u16, 1u16,
                0u16, 2u16, 3u16,
            ]);
        }
    }

    #[test]
    /// Make sure a simple 3x3 height map generates correct
    ///
    /// The test sets up a height map like this:
    ///      0  1  2
    ///      -  -  -
    ///  0|  0  1  2
    ///  1|  3  4  5
    ///  2|  6  7  8
    ///
    /// the verifies the resulting positions and indices for the mesh
    fn make_simple_terrain_3x3() {
        let _map_3x3 = HeightMap {
            width: 3,
            max_height: 8.,
            min_height: 0.,
            map: vec![
                0., 1., 2.,
                3., 4., 5.,
                6., 7., 8.,
            ],
        };
        let mesh = create_simple_terrain(&_map_3x3);
        assert_eq!(mesh.count_vertices(), 9);
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        dbg!("{}", positions);
        let expected_pos = VertexAttributeValues::from(vec![
            [0., 0., 0.],
            [1., 1., 0.],
            [2., 2., 0.],
            [0., 3., 1.],
            [1., 4., 1.],
            [2., 5., 1.],
            [0., 6., 2.],
            [1., 7., 2.],
            [2., 8., 2.],
        ]);
        assert_eq!(positions.get_bytes(), expected_pos.get_bytes());

        if let U16(bytes) = mesh.indices().unwrap() {
            dbg!("{}", bytes);
            assert_eq!(*bytes, vec![0u16, 4u16, 1u16,
                                    0u16, 3u16, 4u16,
                                    1u16, 5u16, 2u16,
                                    1u16, 4u16, 5u16,
                                    3u16, 7u16, 4u16,
                                    3u16, 6u16, 7u16,
                                    4u16, 8u16, 5u16,
                                    4u16, 7u16, 8u16,
            ]);
        }
    }
}