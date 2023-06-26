use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver};

pub struct DCSDataPlugin;

impl Plugin for DCSDataPlugin {
	fn build(&self, app: &mut App){
		info!("lul");
		app.add_startup_system(dcs_data_update_system)
			.add_system(read_stream);
	}
}

//fn process_dcs_csv(dcs_data: &str) {
//
//}

#[derive(Resource, Deref)]
struct DCSDataReceiver(Receiver<String>);

fn dcs_data_update_system(mut commands: Commands, time: Res<Time>) {
	let (dcs_data_tx, dcs_data_rx) = unbounded::<String>();

	wasm_bindgen_futures::spawn_local(async move{
		let client = reqwest::Client::new();
			
		let mut i = 1;
		loop {
			i += 1;
			if i >= 200 {
				break;
			}
			let txt_content = client
        		.get("http://127.0.0.1:2137/units")
        		.send()
        		.await
				.unwrap()
				.text()
				.await
				.unwrap();
			info!("request: {} units:\n{}", i, txt_content);
			//dcs_data_tx.send(txt).unwrap();
		}
	});

	commands.insert_resource(DCSDataReceiver(dcs_data_rx));
}

fn read_stream(receiver: Res<DCSDataReceiver>) {
	for from_stream in receiver.try_iter() {
        info!("test stream: {}", from_stream);
    }
}