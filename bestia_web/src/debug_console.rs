pub struct DebugConsole;

#[derive(Component)]
struct ConsoleUpdate;

impl Plugin for DebugConsole {
	fn build(&self, app: &mut App){
		app.add_startup_system(console_setup);
		//.add_system(console_update_system);
	}
}

fn console_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	let font = asset_server.load("fonts/RobotoMono-Regular.ttf");
	commands.spawn(Camera2dBundle::default());
	commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\n10:",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            )
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        //ConsoleUpdate,
    ));
}

fn console_update_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<ConsoleUpdate>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!(
            "This text changes in the bottom right - {fps:.1} fps, {frame_time:.3} ms/frame",
        );
    }
}