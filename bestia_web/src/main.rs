mod camera_movement_plugin;
mod dcs_data_plugin;

use bevy::math::f32::Quat;
use bevy::prelude::*;
use camera_movement_plugin::*;
use dcs_data_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraMovementPlugin)
        .add_plugin(DCSDataPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("test_imports/caucasus_low_poly.gltf#Scene0"),
        //material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_xyz(4.5, 0.0, 2.25).with_scale(Vec3::new(2.25, 2.25, 2.25)),
        ..default()
    });

    commands.spawn(PbrBundle {
        //mesh: asset_server.load("test_imports/batumi_mess.glb").into(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.05, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        //mesh: asset_server.load("test_imports/batumi_mess.glb").into(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.0, 0.0, 0.5).into()),
        transform: Transform::from_xyz(9.0, 0.05, 4.5),
        ..default()
    });

    //commands.spawn(PbrBundle {
    //	mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
    //    material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
    //	transform: Transform::from_xyz(0.0, -0.5, 0.0),
    //    ..default()
    //});

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            //intensity: 1500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-0.38539) * Quat::from_rotation_y(0.78539)),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: PerspectiveProjection {
                far: 10000.0,
                near: 0.01,
                ..default()
            }.into(),
            ..default()
        },
        CameraMovement { ..default() },
    ));
}
