use bevy::prelude::*;

use crate::item::Item;
use crate::physics;

const PLAYER_INVENTORY_SIZE: i32 = 10;

pub struct SpriteAnimFrame(pub usize);

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
		//app.add_system(player_movement.system());
	}
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	// We should use our own sprite sheet
	let red_radish = asset_server.load("redRadishSheet.png");

	// Spawn the player, assign the sprite and define the colision bound box
	let texture_atlas = TextureAtlas::from_grid(red_radish, Vec2::new(16.0, 16.0), 7, 1);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			transform: Transform::from_scale(Vec3::splat(6.0)),
			..Default::default()
		})
		.insert(SpriteAnimFrame(0))
		.insert(Player::new("Bob".to_string()))
		.insert(Timer::from_seconds(0.1, true));
}

// Change the player velocity and animation based on the key press.
fn player_movement(
	time: Res<Time>,
	mut frame: Local<u8>,
	keyboard_input: Res<Input<KeyCode>>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
	const SPEED: f32 = 30.;
	*frame = frame.wrapping_add(1);

	let mut direction = Vec3::new(0., 0., 0.);

	if keyboard_input.pressed(KeyCode::Left) {
		direction += Vec3::new(-1., 0., 0.);

		let frames = [12, 13, 14, 15];
		if *frame % 10 == 0 {
			*frame = 0;
			for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
				timer.tick(time.delta());
				if timer.finished() {
					let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
					sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
				}
			}
		}
	}

	if keyboard_input.pressed(KeyCode::Right) {
		direction += Vec3::new(1., 0., 0.);
		let frames = [4, 5, 6, 7];
		if *frame % 10 == 0 {
			*frame = 0;
			for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
				timer.tick(time.delta());
				if timer.finished() {
					let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
					sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
				}
			}
		}
	}

	if keyboard_input.pressed(KeyCode::Up) {
		direction += Vec3::new(0., -1., 0.);
		let frames = [8, 9, 10, 11];
		if *frame % 10 == 0 {
			*frame = 0;
			for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
				timer.tick(time.delta());
				if timer.finished() {
					let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
					sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
				}
			}
		}
	}

	if keyboard_input.pressed(KeyCode::Down) {
		direction += Vec3::new(0., 1., 0.);
		let frames = [0, 1, 2, 3];
		if *frame % 10 == 0 {
			*frame = 0;
			for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
				timer.tick(time.delta());
				if timer.finished() {
					let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
					sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
				}
			}
		}
	}
	let normalized_direction = direction.normalize_or_zero();
	let _ = normalized_direction * SPEED;
}
