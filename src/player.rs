// use bevy::prelude::{
//     AppBuilder, AssetServer, Commands, Input, IntoSystem, KeyCode, Plugin, Res, SystemStage,
// };
use bevy::prelude::*;
const PLAYER_INVENTORY_SIZE: i32 = 10;

use crate::item;

pub struct Player {
    // pub inventory: Vec<item::Item>,
    pub name: String,
}
impl Player {
    pub fn new() -> Self {
        Player {
            name: "Bob".to_string(),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("game_actors", SystemStage::single(player_spawn.system()))
            .add_system(player_movement.system());
    }
}

fn player_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn player_movement(keyboard_input: Res<Input<KeyCode>>) {}
