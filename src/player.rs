use bevy::prelude::*;
use bevy_retrograde::prelude::*;

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
        app.add_system(player_movement.system());
    }
}

// Change the player velocity and animation based on the key press.
fn player_movement(
    mut frame: Local<u8>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    mut q: Query<(&Handle<SpriteSheet>, &mut SpriteAnimFrame), With<Handle<Image>>>,
    mut sprite_sheet_assets: ResMut<Assets<SpriteSheet>>,
) {
    const SPEED: f32 = 30.;
    *frame = frame.wrapping_add(1);

    for mut velocity in query.iter_mut() {
        let mut direction = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::Left) {
            // direction += Vec3::new(-SPEED, 0., 0.);
            direction += Vec3::new(-1., 0., 0.);

            let frames = [12, 13, 14, 15];
            if *frame % 10 == 0 {
                *frame = 0;
                for (sprite_sheet_handle, mut frame) in q.iter_mut() {
                    if let Some(sprite_sheet) = sprite_sheet_assets.get_mut(sprite_sheet_handle) {
                        frame.0 = frame.0.wrapping_add(1);
                        sprite_sheet.tile_index = frames[frame.0 % frames.len()];
                    }
                }
            }
        }

        if keyboard_input.pressed(KeyCode::Right) {
            // direction += Vec3::new(SPEED, 0., 0.);
            direction += Vec3::new(1., 0., 0.);
            let frames = [4, 5, 6, 7];
            if *frame % 10 == 0 {
                *frame = 0;
                for (sprite_sheet_handle, mut frame) in q.iter_mut() {
                    if let Some(sprite_sheet) = sprite_sheet_assets.get_mut(sprite_sheet_handle) {
                        frame.0 = frame.0.wrapping_add(1);
                        sprite_sheet.tile_index = frames[frame.0 % frames.len()];
                    }
                }
            }
        }

        if keyboard_input.pressed(KeyCode::Up) {
            // direction += Vec3::new(0., -SPEED, 0.);
            direction += Vec3::new(0., -1., 0.);
            let frames = [8, 9, 10, 11];
            if *frame % 10 == 0 {
                *frame = 0;
                for (sprite_sheet_handle, mut frame) in q.iter_mut() {
                    if let Some(sprite_sheet) = sprite_sheet_assets.get_mut(sprite_sheet_handle) {
                        frame.0 = frame.0.wrapping_add(1);
                        sprite_sheet.tile_index = frames[frame.0 % frames.len()];
                    }
                }
            }
        }

        if keyboard_input.pressed(KeyCode::Down) {
            // direction += Vec3::new(0., SPEED, 0.);
            direction += Vec3::new(0., 1., 0.);
            let frames = [0, 1, 2, 3];
            if *frame % 10 == 0 {
                *frame = 0;
                for (sprite_sheet_handle, mut frame) in q.iter_mut() {
                    if let Some(sprite_sheet) = sprite_sheet_assets.get_mut(sprite_sheet_handle) {
                        frame.0 = frame.0.wrapping_add(1);
                        sprite_sheet.tile_index = frames[frame.0 % frames.len()];
                    }
                }
            }
        }
        let normalized_direction = direction.normalize_or_zero();
        // *velocity = Velocity::from_linear(direction);
        *velocity = Velocity::from_linear(normalized_direction * SPEED);
    }
}
