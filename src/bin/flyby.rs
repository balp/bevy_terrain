use std::env;
use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(WireframePlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let args : Vec<String> = env::args().collect();
    let mut terrain_file_name = "resources/svanesund_heightmap_1700x1700.png";
    if args.len() > 1 {
        terrain_file_name = args[1].as_str();
    }
    wireframe_config.global = false;
    // water plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1500.0 })),
        material: materials.add(Color::rgba(0.1, 0.1, 0.7, 0.2).into()),
        transform: Transform::from_xyz(0.0, 0., 0.0),
        ..default()
    });
    if let Ok(height_map) = bevy_terrain::heightmap::load_height_map(terrain_file_name) {
        let scaled_map = height_map.transform(318., -20.);
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(bevy_terrain::terrain::create_simple_terrain(&scaled_map)),
                material: materials.add(Color::rgb(0.1, 0.7, 0.3).into()),
                transform: Transform::from_xyz(
                    -(scaled_map.width as f32 / 2.),
                    0.,
                    -(scaled_map.width as f32 / 2.),
                ),
                ..default()
            })
            .insert(Wireframe);
    } else {
        panic!("Unable to load terrain!!!")
    }

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
}