use bevy::prelude::*;

use crate::item::Item;

const PLAYER_INVENTORY_SIZE: i32 = 10;

pub struct Player {
	pub inventory: Vec<Item>,
	pub name: String,
}

impl Player {
	pub fn new(name: String) -> Self {
		Player {
			name: name,
			inventory: Vec::new(),
		}
	}
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_startup_stage("game_actors", SystemStage::single(player_spawn.system()))
			/*.add_system(player_movement.system())*/;
	}
}

fn player_spawn(mut commands: Commands) {}

/*fn player_movement(keyboard_input: Res<Input<KeyCode>>, mut q: Query<&mut Velocity, With<Player>) {
}*/
