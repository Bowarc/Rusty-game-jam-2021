#![allow(unused)]
use bevy::prelude::{App, Command};
use bevy_retrograde::prelude::RetroPlugins;
fn main() {
    App.build()
        .add_plugins(RetroPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Command, asset_server: Res<AssetServer>) {}
