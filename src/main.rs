#![allow(unused)]
use bevy::prelude::{App, AssetServer, Commands, DefaultPlugins, IntoSystem, Res};
// use bevy::{prelude::*, sprite::collide_aabb::collide};
// use bevy_retrograde::prelude::RetroPlugins;

mod item;
mod player;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
