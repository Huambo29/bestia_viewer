use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver};
use std::collections::{HashMap, HashSet};
use bevy::math::EulerRot;
use crate::world_labels_plugin::LabelOwnerComponent;

pub struct DCSDataPlugin;

impl Plugin for DCSDataPlugin {
    fn build(&self, app: &mut App) {
        info!("lul");
        app.add_startup_system(dcs_data_setup)
            .add_system(dcs_data_stream_system)
			.add_system(dcs_entities_update_system);
    }
}

#[derive(Resource, Deref)]
struct DCSDataReceiver(Receiver<String>);

#[derive(Resource)]
struct DCSUnitsData(HashMap<i32, DCSUnit>);

fn dcs_data_setup(mut commands: Commands, time: Res<Time>) {
    let (dcs_data_tx, dcs_data_rx) = unbounded::<String>();

    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest::Client::new();

        loop {
			let response = client
				.get("http://127.0.0.1:2137/units")
				.send()
				.await;
			match response {
				Ok(response) => {
					let txt_content = response.text().await.unwrap();
					dcs_data_tx.send(txt_content).unwrap();
				},
				Err(e) => {
					warn!("No dcs connection")
				}
			}
        }
    });

    commands.insert_resource(DCSDataReceiver(dcs_data_rx));
	commands.insert_resource(DCSUnitsData(HashMap::new()));
}

#[derive(Debug, Clone)]
pub struct DCSUnit {
	id: i32,
    group_name: Option<String>,
    unit_name: Option<String>,
    unit_type: Option<String>,
    coalition: Option<String>,
    coalition_id: Option<i32>,
    country: Option<i32>,
    position_x: Option<f32>,
	position_y: Option<f32>,
	position_z: Option<f32>,
    latitude: Option<f32>,
	longitude: Option<f32>,
	altitude: Option<f32>,
    heading: Option<f32>,
    pitch: Option<f32>,
    bank: Option<f32>,
    is_human: Option<bool>,
    is_invisible: Option<bool>,
    is_radar_active: Option<bool>,
    is_jamming: Option<bool>,
    is_ir_jamming: Option<bool>,
    is_born: Option<bool>,
    is_static: Option<bool>,
    is_ai_on: Option<bool>,
}

fn csv_value_string(csv_value: &str) -> Option<String> {
	match csv_value {
		"nil" => None,
		_ => {
			let mut chars = csv_value.chars();
    		chars.next();
    		chars.next_back();
    		Some(chars.as_str().to_string())
		}
	}
}

fn csv_value_i32(csv_value: &str) -> Option<i32> {
	match csv_value {
		"nil" => None,
		_ => {
			Some(csv_value.to_string().parse::<i32>().expect("couldn't convert csv value to i32"))
		}
	}
}

fn csv_value_f32(csv_value: &str) -> Option<f32> {
	match csv_value {
		"nil" => None,
		_ => {
			Some(csv_value.to_string().parse::<f32>().expect("couldn't convert csv value to i32"))
		}
	}
}

fn csv_value_bool(csv_value: &str) -> Option<bool> {
	match csv_value {
		"nil" => None,
		"true" => Some(true),
		"false" => Some(false),
		_ => {
			error!("Incorrect csv_value: {}", csv_value);
			None
		}
	}
}

impl DCSUnit {
    pub fn new(csv_line: String) -> DCSUnit {
        let separator = ", ";
        let csv_values: Vec<&str> = csv_line.split(separator).collect();

        DCSUnit {
			id: csv_value_i32(csv_values[0]).unwrap(),
            group_name: csv_value_string(csv_values[1]),
			unit_name: csv_value_string(csv_values[2]),
			unit_type: csv_value_string(csv_values[3]),
			coalition: csv_value_string(csv_values[4]),
			coalition_id: csv_value_i32(csv_values[5]),
			country: csv_value_i32(csv_values[6]),
			position_x: csv_value_f32(csv_values[7]),
			position_y: csv_value_f32(csv_values[8]),
			position_z: csv_value_f32(csv_values[9]),
			latitude: csv_value_f32(csv_values[10]),
			longitude: csv_value_f32(csv_values[11]),
			altitude: csv_value_f32(csv_values[12]),
			heading: csv_value_f32(csv_values[13]),
			pitch: csv_value_f32(csv_values[14]),
			bank: csv_value_f32(csv_values[15]),
			is_human: csv_value_bool(csv_values[16]),
			is_invisible: csv_value_bool(csv_values[17]),
			is_radar_active: csv_value_bool(csv_values[18]),
			is_jamming: csv_value_bool(csv_values[19]),
			is_ir_jamming: csv_value_bool(csv_values[20]),
			is_born: csv_value_bool(csv_values[21]),
			is_static: csv_value_bool(csv_values[22]),
			is_ai_on: csv_value_bool(csv_values[23])
        }
    }
}

#[derive(Component)]
pub struct DCSUnitComponent {
	pub dcs_unit: DCSUnit
}

fn dcs_entities_update_system(mut dcs_units_query: Query<(&mut Transform, &mut DCSUnitComponent)>){
	for (mut unit_transform, mut dcs_unit_component) in dcs_units_query.iter_mut() {
		let dcs_unit = dcs_unit_component.dcs_unit.clone();
		unit_transform.translation = Vec3::new(dcs_unit.longitude.unwrap() - 34.265278, dcs_unit.altitude.unwrap() / 111000.0, -(dcs_unit.latitude.unwrap() - 45.129444));
		unit_transform.rotation = Quat::from_euler(EulerRot::YXZ, -dcs_unit.heading.unwrap(), dcs_unit.pitch.unwrap(), -dcs_unit.bank.unwrap())
	}
}

fn dcs_data_stream_system(
	mut commands: Commands,
	receiver: Res<DCSDataReceiver>,
	mut dcs_units_data: ResMut<DCSUnitsData>,
	mut dcs_units_query: Query<(Entity, &mut DCSUnitComponent)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
    for from_stream in receiver.try_iter() {
        //info!("test stream:\n{}", from_stream);

		let mut current_dcs_units = HashMap::new();

        let mut csv_lines = from_stream.lines();
		csv_lines.next();
		for csv_line in csv_lines {
			let unit = DCSUnit::new(csv_line.to_string());
			current_dcs_units.insert(unit.id, unit);
		}
		//info!("current_dcs_units: {:?}", current_dcs_units);

		let mut processed_units = HashSet::new();

		for (entity, mut dcs_unit_component) in dcs_units_query.iter_mut() {
			let dcs_unit_id = dcs_unit_component.dcs_unit.id;
			if !current_dcs_units.contains_key(&dcs_unit_id) {
				//info!("Removing entity unit id: {:?}", dcs_unit_id);
				commands.entity(entity).despawn();
			} else {
				//info!("updating entity unit id: {:?}", dcs_unit_id);
				dcs_unit_component.dcs_unit = current_dcs_units.get(&dcs_unit_id.clone()).unwrap().clone();
				processed_units.insert(dcs_unit_id);
			}
		}

		for (unit_id, dcs_unit) in current_dcs_units.into_iter() {
			if !processed_units.contains(&unit_id) {
				//info!("creating entity unit id: {:?}", unit_id);

				commands.spawn((
					DCSUnitComponent {
						dcs_unit: dcs_unit.clone()
					},
					PbrBundle {
						//mesh: asset_server.load("test_imports/batumi_mess.glb").into(),
						mesh: meshes.add(Mesh::from(shape::Cube { size: 0.001 })),
						material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
						..default()
					},
					LabelOwnerComponent {
						text: dcs_unit.unit_type.unwrap() 
					}
				));
			}
		}
    }
}
