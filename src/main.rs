mod heightmap;
mod terrain;

use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use heightmap::HeightMap;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(WireframePlugin)
        .add_system(rotate_cam)
        .run();
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

    let height_map = heightmap::make_height_map();
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(terrain::create_simple_terrain(&height_map)),
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

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

}