use bevy::prelude::*;

pub struct SelectEntityPlugin;

impl Plugin for SelectEntityPlugin {
	fn build(&self, app: &mut App){
		app.add_system();
	}
}

#[derive(Component)]
pub struct SelectableComponent;

#[derive(Reflect, Clone)]
pub struct SelectableRaycastSet;