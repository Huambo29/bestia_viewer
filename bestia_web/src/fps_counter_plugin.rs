use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FPSCounterPlugin;

impl Plugin for FPSCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(fps_counter_setup)
            .add_system(fps_counter_system);
    }
}

#[derive(Component)]
struct FPSCounterComponent;

fn fps_counter_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	let font = asset_server.load("fonts/RobotoMono-Regular.ttf");
	commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font,
                font_size: 15.0,
                color: Color::YELLOW
            }),
        ]),
        FPSCounterComponent,
    ));
}

fn fps_counter_system(
	diagnostics: Res<Diagnostics>,
	mut query: Query<&mut Text, With<FPSCounterComponent>>
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}