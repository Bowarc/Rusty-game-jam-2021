#[deny(unsafe_code)]
use bevy::prelude::*;
use bevy_retrograde::prelude::*;

mod item;
mod player;

use player::PlayerPlugin;

fn main() {
	App::build()
		.insert_resource(WindowDescriptor {
			title: "Rusty caves".into(),
			..Default::default()
		})
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup.system())
		.run();
}

fn setup(mut commands: Commands/*, asset_server: Res<AssetServer>*/) {
	// spawn the camera
	commands.spawn_bundle(CameraBundle {
		camera: Camera {
			size: CameraSize::FixedHeight(100),
			background_color: Color::new(0.2, 0.2, 0.2, 1.0),
			..Default::default()
		},
		transform: Transform::from_xyz(0., -50., 0.),
		..Default::default()
	});

	// TODO: Load the assets

	// TODO: Load the map
}
