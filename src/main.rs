#![deny(unsafe_code)]
use bevy::{
    app::AppExit, asset::LoadState, prelude::*, sprite::TextureAtlasBuilder, utils::HashSet,
};
use bevy_render::{draw::Visible, prelude::Texture};
use bevy_tilemap::{chunk::LayerKind, prelude::TilemapBundle, Tilemap, TilemapLayer};
use map::MapPlugin;
use player::PlayerPlugin;
use std::fs::File;
use std::io::Read;
mod item;
mod map;
mod physics;
mod player;

const TILEMAP_WIDTH: i32 = 60;
const TILEMAP_HEIGHT: i32 = 60;
const MAP_DIR: &str = "/assets/maps";

// enum GameState {
//     LoadingAssets,
//     LoadingMap,
//     Playing,
// }

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone, Default)]
struct GameStage;

#[derive(Default)]
struct Game {
    setup_done: bool,
    collisions: HashSet<(i32, i32)>,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Default)]
struct Map {
    raw_map: Vec<Vec<i32>>,
}

#[derive(Default, Clone)]
struct TileSpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

impl Game {
    fn try_move(&mut self, pos: Position, delta_pos: (i32, i32)) -> Position {
        // This shit is turbo bad but it should works
        // (if all hitboxes are exacly 1 tile (size) and movement are tile by tile)

        let wanted_pos = Position {
            x: pos.x + delta_pos.0,
            y: pos.y + delta_pos.1,
        };

        if !self.collisions.contains(&(wanted_pos.x, wanted_pos.y)) {
            wanted_pos
        } else {
            pos
        }
    }
}

fn main() {
    App::build()
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(exit_system.system())
        .init_resource::<TileSpriteHandles>()
        .init_resource::<Game>()
        .init_resource::<Map>()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(load_from_file.system())
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

fn load_from_file(
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    mut map: ResMut<Map>,
    asset_server: Res<AssetServer>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    let raw_map_json_data = asset_server.load_folder("maps").unwrap();
    println!("{:?}", raw_map_json_data);
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mut sprite_handles: ResMut<TileSpriteHandles>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if game.setup_done {
        // idk if we should let this here and use this as a system (not a startup system like rn)
        return;
    }
    // spawn the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Lets load all our textures from our folder!
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for handle in sprite_handles.handles.iter() {
            // println!("{:?}", handle);
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
        game.setup_done = true;
    }
}
