use bevy::prelude::*;
use bevy::math::f32::Quat;
use bevy::ecs::event::EventReader;
use bevy::input::mouse::*;
use bevy_mod_raycast::*;

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
	fn build(&self, app: &mut App){
		app.add_system(camera_movement_system);
	}
}

#[derive(Component)]
pub struct TerrainComponent;

#[derive(Component)]
pub struct CameraMovement {
	pub root: Vec3,
	pub camera_distance: f32
}

impl Default for CameraMovement {
    fn default() -> Self {
        CameraMovement {
            root: Vec3::ZERO,
            camera_distance: 4.0
        }
    }
}

fn camera_movement_system(
	mut query: Query<(&mut Transform, &mut CameraMovement), Without<TerrainComponent>>,
	mouse_buttons: Res<Input<MouseButton>>,
	mut mouse_motion_events: EventReader<MouseMotion>,
	mut mouse_wheel_events: EventReader<MouseWheel>,
	//meshes: Res<Assets<Mesh>>,
	//query_terrain: Query<(&Handle<Mesh>, &Transform, &TerrainComponent)>
) {
    for (mut transform, mut camera_movement) in query.iter_mut() {
		for mouse_event in mouse_motion_events.iter() {
			if mouse_buttons.pressed(MouseButton::Left) {
				let move_speed: f32 = 0.001 * camera_movement.camera_distance;
				let vector_forward: Vec3 = transform.forward();
				let vector_right: Vec3 = transform.right();

				let vector_forward: Vec3 = Vec3::new(vector_forward.x, 0.0, vector_forward.z).normalize();
				let vector_right: Vec3 = Vec3::new(vector_right.x, 0.0, vector_right.z).normalize();

				camera_movement.root += move_speed * vector_forward * mouse_event.delta.y;
				camera_movement.root += -move_speed * vector_right * mouse_event.delta.x;

				//let root_y = 0.0;
				//
				//let raycast_ray = Ray3d::new(Vec3::new(camera_movement.root.x, 1.0, camera_movement.root.z), Vec3::new(0.0, -1.0, 0.0));
				//for (terrain_mesh_handle, terrain_transform, _) in query_terrain.iter() {
				//	if let Some(terrain_mesh) = meshes.get(terrain_mesh_handle) {
				//		let mesh_to_world = terrain_transform.compute_matrix();
//
				//		if let Some(intersection) = ray_intersection_over_mesh(
				//			terrain_mesh,
				//			&mesh_to_world,
				//			&raycast_ray,
				//			Backfaces::Cull
				//		) {
				//			info!("raycast distance: {}", intersection.distance())
				//		}
				//	}
				//}	
			}
			if mouse_buttons.pressed(MouseButton::Right) {
				let rotation_speed: f32 = 0.005;
				let (heading, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
				transform.rotation = Quat::from_euler(
					EulerRot::YXZ, 
					heading - rotation_speed * mouse_event.delta.x, 
					(pitch - rotation_speed * mouse_event.delta.y).max(-1.55).min(-0.01745), 
					0.0
				);
			}
		}

		for mouse_event in mouse_wheel_events.iter() {
			let delta;
			match mouse_event.unit {
				MouseScrollUnit::Line => {
					let scroll_speed: f32 = 0.25;
					delta = -mouse_event.y * scroll_speed;
				}
				MouseScrollUnit::Pixel => {
					let scroll_speed: f32 = 0.0025;
					delta = -mouse_event.y * scroll_speed;
				}
			}
			camera_movement.camera_distance += camera_movement.camera_distance * delta;
			camera_movement.camera_distance = camera_movement.camera_distance.max(0.0001)
		}
		let rotated_vector: Vec3 = transform.forward() * camera_movement.camera_distance;
		transform.translation = camera_movement.root - rotated_vector;
    }
}