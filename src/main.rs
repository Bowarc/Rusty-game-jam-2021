#![deny(unsafe_code)]
use bevy::{
	app::AppExit,
	prelude::*
};
use bevy_retrograde::prelude::*;

mod item;
mod physics;
mod player;
mod map;

use player::{Player, PlayerPlugin, SpriteAnimFrame};
use map::MapPlugin;

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

fn main() {
	App::build()
		.add_system(bevy::input::system::exit_on_esc_system.system())
		.add_system(exit_system.system())
		.add_plugins(RetroPlugins)
		.add_plugin(MapPlugin)
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup.system())
		.run();
}

fn exit_system(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
	if (keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::RControl)) && keyboard_input.pressed(KeyCode::Q) {
		exit.send(AppExit);
	}
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_sheet_assets: ResMut<Assets<SpriteSheet>>,
) {
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

    // Load the assets
    let red_radish = asset_server.load("redRadishSheet.png"); // We should use our own sprite sheet

    // TODO: Load the map

    // Spawn the player and assign sprite
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite_bundle: SpriteBundle {
                image: red_radish.clone(),
                sprite: Sprite {
                    pixel_perfect: false,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., -50., 0.),
                ..Default::default()
            },
            sprite_sheet: sprite_sheet_assets.add(SpriteSheet {
                grid_size: UVec2::splat(16),
                tile_index: 0,
            }),
        })
        .insert(SpriteAnimFrame(0))
        .insert(TesselatedCollider {
            image: red_radish.clone(),
            tesselator_config: TesselatedColliderConfig {
                vertice_separation: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(PhysicMaterial {
            friction: 0.,
            restitution: 0.,
            ..Default::default()
        })
        .insert(Velocity::from_linear(Vec3::default()))
        .insert(Player::new("Bob".to_string()));
}
