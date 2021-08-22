#![deny(unsafe_code)]
use bevy::{
	app::AppExit,
	prelude::*,
	sprite::TextureAtlasBuilder,
};
use bevy_tilemap::{
	chunk::LayerKind,
	prelude::TilemapBundle,
	Tilemap,
	TilemapLayer,
};
use bevy_render::{
	draw::Visible,
	prelude::Texture,
};

mod item;
mod map;
mod physics;
mod player;

use map::MapPlugin;
use player::PlayerPlugin;

const TILEMAP_WIDTH: i32 = 60;
const TILEMAP_HEIGHT: i32 = 60;

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

#[derive(Default, Clone)]
struct TileSpriteHandles {
	handles: Vec<HandleUntyped>,
	atlas_loaded: bool,
}

fn main() {
	App::build()
		.add_system(bevy::input::system::exit_on_esc_system.system())
		.add_system(exit_system.system())
		.add_plugins(DefaultPlugins)
		.add_plugin(MapPlugin)
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup1.system())
		.add_startup_system(setup.system())
		.run();
}

fn exit_system(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
	if (keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl))
		&& keyboard_input.pressed(KeyCode::Q)
	{
		exit.send(AppExit);
	}
}

fn setup1(mut tile_sprite_handles: ResMut<TileSpriteHandles>, asset_server: Res<AssetServer>) {
	tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut sprite_handles: ResMut<TileSpriteHandles>,
	mut textures: ResMut<Assets<Texture>>,
) {
	// spawn the camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	let mut texture_atlas_builder = TextureAtlasBuilder::default();
	for handle in sprite_handles.handles.iter() {
		let texture = textures.get(handle).unwrap();
		texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
	}
	let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
	let atlas_handle = texture_atlases.add(texture_atlas);

	// TODO: Load the map # NOT FINISHED
	let tilemap = Tilemap::builder()
		.dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
		// .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT, 1)
		.texture_dimensions(32, 32)
		// .auto_chunk()
		.auto_spawn(2, 2)
		.add_layer(
			TilemapLayer {
				kind: LayerKind::Dense,
				..Default::default()
			},
			0,
		)
		.texture_atlas(atlas_handle)
		.finish()
		.unwrap();

	let tilemap_components = TilemapBundle {
		tilemap,
		visible: Visible {
			is_visible: true,
			is_transparent: true,
		},
		transform: Default::default(),
		global_transform: Default::default(),
	};
	commands.spawn().insert_bundle(tilemap_components);
}
