use bevy::prelude::*;

pub struct DCSDataPlugin;

impl Plugin for DCSDataPlugin {
	fn build(&self, app: &mut App){
		info!("lul");
		app.add_startup_system(dcs_data_update_system);
	}
}

fn get_dcs_data() {
	wasm_bindgen_futures::spawn_local(async move {
		let client = reqwest::Client::new();
		//let txt = reqwest::get("http://127.0.0.1:2137/ping").await;
		let txt = client
        	.get("http://127.0.0.1:2137/ping")
        	.send()
        	.await;
		info!("test: {:?}", txt);
	});
}

fn dcs_data_update_system() {
	get_dcs_data();
}