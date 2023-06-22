use bevy::prelude::*;

pub struct DCSDataPlugin;

impl Plugin for DCSDataPlugin {
	fn build(&self, app: &mut App){
		info!("lul");
		app.add_system(dcs_data_update_system);
	}
}

fn dcs_data_update_system() {
	
}