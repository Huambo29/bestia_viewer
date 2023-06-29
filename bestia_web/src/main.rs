mod camera_movement_plugin;
mod dcs_data_plugin;
mod fps_counter_plugin;
mod world_labels_plugin;
mod corner_text_plugin;
mod json_loader;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::f32::Quat;
use bevy::prelude::*;
use camera_movement_plugin::*;
use dcs_data_plugin::*;
use fps_counter_plugin::*;
use world_labels_plugin::*;
use corner_text_plugin::*;
use json_loader::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(CameraMovementPlugin)
        .add_plugin(DCSDataPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(FPSCounterPlugin)
		.add_plugin(WorldLabelsPlugin)
		.add_plugin(CornerTextPlugin)
		.add_asset::<JsonFileAsset>()
        .init_asset_loader::<JsonLoader>()
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
	let some_handle: Handle<JsonFileAsset> = asset_server.load("dcs_units_metadata/dcs_units_metadata.json");

    commands.spawn((PbrBundle {
        mesh: asset_server.load("test_imports/caucasus_high_poly_smooth.glb#Mesh0/Primitive0"),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            base_color_texture: Some(asset_server.load("test_imports/surface.png")),
			perceptual_roughness: 1.0,
			metallic: 0.0,
			reflectance: 0.0,
            ..default()
        }),
        transform: Transform::from_xyz(4.5, 0.0, 2.25).with_scale(Vec3::new(2.25, 2.25, 2.25)),
        ..default()
    	},
		TerrainComponent
	));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.5, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    	},
		LabelOwnerComponent {
			text: "2137".to_string()
		}
	));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.0, 0.0, 0.5).into()),
        transform: Transform::from_xyz(9.0, 0.0, 4.5),
        ..default()
    });

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
                near: 0.0001,
                ..default()
            }
            .into(),
            ..default()
        },
        CameraMovement { ..default() },
    ));
}
