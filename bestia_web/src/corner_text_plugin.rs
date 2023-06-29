use bevy::prelude::*;

pub struct CornerTextPlugin;

impl Plugin for CornerTextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CornerText("start".to_string()))
            .add_startup_system(corner_text_setup)
			.add_system(corner_text_system);
    }
}

#[derive(Resource)]
struct CornerText(String);

#[derive(Component)]
struct CornerTextComponent;

fn corner_text_setup(
	mut commands: Commands, asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/RobotoMono-Regular.ttf");

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: font,
                font_size: 20.0,
                color: Color::WHITE,
            }
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(3.0),
                left: Val::Px(3.0),
                ..default()
            },
            ..default()
        })
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.1)),
        CornerTextComponent,
    ));
}

fn corner_text_system(
	corner_text: Res<CornerText>,
	mut query: Query<(&mut Text, &CornerTextComponent)>
) {
	for (mut text, _) in query.iter_mut() {
		text.sections[0].value = corner_text.0.clone();
	}
}
