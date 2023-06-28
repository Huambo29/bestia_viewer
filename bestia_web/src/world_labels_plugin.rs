use bevy::prelude::*;

pub struct WorldLabelsPlugin;

impl Plugin for WorldLabelsPlugin {
	fn build(&self, app: &mut App){
		app.add_system(world_label_system);
	}
}

#[derive(Component)]
pub struct UILabel;

struct LabelRenderData {
	position: Vec2,
	text: String
}

#[derive(Component)]
pub struct LabelOwnerComponent{
	pub text: String
}

fn world_label_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut label_owner_query: Query<(&mut LabelOwnerComponent, &mut Transform)>,
	camera_query: Query<(&Camera, &GlobalTransform)>,
	mut labels_query: Query<(Entity, &UILabel, &mut Text, &mut Style)>
) {
	for (camera, camera_transform) in camera_query.iter() {
		let mut labels_to_render: Vec<LabelRenderData> = Vec::new();
		
		if let Some(viewport_size) = camera.logical_viewport_size() {
			//info!("viewport size: x: {} y: {}", viewport_size.x, viewport_size.y);

			for (label_owner_component, label_owner_transform) in label_owner_query.iter_mut() {
				match camera.world_to_viewport(camera_transform, label_owner_transform.translation) {
					Some(label_screen_vec) => {
						//info!("label screen: {:?}", label_screen_vec);
						if label_screen_vec.x >= 0. && label_screen_vec.x <= viewport_size.x as f32 && label_screen_vec.y >= 0. && label_screen_vec.y <= viewport_size.y {
							labels_to_render.push(LabelRenderData{
								position: label_screen_vec,
								text: label_owner_component.text.clone()
							})
						}
					},
					None => {
						//info!("label outside camera view");
					}
				}
			}
		} else {
			error!("no viewport size");
		}
		

		let mut labels_iterator = labels_query.iter_mut();
		let mut iter_result = labels_iterator.next();
		for label_to_render in &labels_to_render {
			//info!("rendering label: {:?}", label_to_render.text);
			if iter_result.is_none() {
				//info!("iter is none");
				commands.spawn((
					TextBundle::from_section(
						label_to_render.text.clone(),
						TextStyle {
							font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
							font_size: 20.0,
							color: Color::WHITE
						}
					).with_style(Style {
						position_type: PositionType::Absolute,
						position: UiRect {
							left: Val::Px(label_to_render.position.x),
							bottom: Val::Px(label_to_render.position.y),
							..Default::default()
						},
						..Default::default()
					}),
					UILabel
				));
			} else {
				//info!("iter is something");
				let (_, _, mut text, mut style) = iter_result.unwrap();

				style.position.left = Val::Px(label_to_render.position.x);
				style.position.bottom = Val::Px(label_to_render.position.y);
				text.sections[0].value = label_to_render.text.clone();

				iter_result = labels_iterator.next();
			}
			
		}

		while !iter_result.is_none() {
			let (entity, _, _, _) = iter_result.unwrap();
			//info!("Despawning entity");
			commands.entity(entity).despawn();

			iter_result = labels_iterator.next();
		}
		//info!("cycle ende");
	}
}