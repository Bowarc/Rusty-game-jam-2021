use crate::bloc;
use crate::physics;
use ggez;
use serde_json::Value; //Result
use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;
const MAP_FILE: &str = "map_settings.json";
pub struct Map {
    pub map_title: String,
    pub tile_size: f32,
    pub map_file_content: Vec<Vec<i32>>,
    pub bloc_list: Vec<bloc::Bloc>,
    pub ghost_tiles: Vec<f32>,
    pub total_rows: f32,
    pub total_cols: f32,
    pub diag_size: f32,
    pub image_hashmap: HashMap<i32, ggez::graphics::spritebatch::SpriteBatch>,
}

pub struct Tile {
    pub hitbox: ggez::graphics::Rect,
    pub material: i32,
    pub transparent: bool,
    pub angle: f32,
}
impl Map {
    pub fn new(tile_size: f32, _current_id: &mut i32) -> Self {
        Map {
            map_title: "map title".to_string(),
            tile_size: tile_size,
            map_file_content: Vec::new(),
            bloc_list: Vec::new(),
            ghost_tiles: Vec::new(),
            total_rows: 0.,
            total_cols: 0.,
            diag_size: 0.,
            image_hashmap: HashMap::new(),
        }
    }

    pub fn load_new_map(&mut self, map_name: String, ctx: &mut ggez::Context) -> ggez::GameResult {
        let start_time = SystemTime::now();

        println!("Loading map: {}", map_name);

        let map_file_path = format!("/maps/{}/{}", map_name, MAP_FILE);
        // let mut file = ggezfs::open(ctx, format!("/maps/{}/{}", map_name, MAP_FILE_NAME)).unwrap();
        let mut file = ggez::filesystem::open(ctx, map_file_path).unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let map_file_data: Value = serde_json::from_str(&data).unwrap();

        let mut ghost_tiles: Vec<f32> = vec![];
        for i in map_file_data["ghost_tiles"].as_array().unwrap() {
            ghost_tiles.push(i.as_f64().unwrap() as f32);
        }

        let mut tile_translate: HashMap<i32, String> = HashMap::new();
        for translate in map_file_data["tile_translate"].as_object() {
            for (key, value) in translate {
                tile_translate.insert(
                    key.parse::<i32>().unwrap(),
                    serde_json::from_str(&value.to_string()).unwrap(),
                );
            }
        }

        let mut image_hashmap: HashMap<i32, ggez::graphics::spritebatch::SpriteBatch> =
            HashMap::new();
        for (key, value) in tile_translate.iter() {
            if value != "air" {
                let mut texture_file_name: String = value.to_string();
                texture_file_name.push_str(".png");
                let pth = format!("/maps/{}/tiles/{}", map_name, texture_file_name);
                println!("Loading: '{}'", pth);
                let image = ggez::graphics::Image::new(ctx, pth);
                image_hashmap.insert(
                    *key,
                    ggez::graphics::spritebatch::SpriteBatch::new(image.clone().unwrap()),
                );
            } else {
            }
        }

        self.map_file_content =
            serde_json::from_str(&map_file_data["map_data"].to_string()).unwrap();
        self.total_rows = map_file_data["config"]["rows"].as_i64().unwrap() as f32;
        self.total_cols = map_file_data["config"]["cols"].as_i64().unwrap() as f32;
        self.diag_size =
            physics::get_diagonal_size(self.total_cols, self.total_rows, self.tile_size);
        self.map_title = map_file_data["config"]["name"].to_string();
        self.image_hashmap = image_hashmap;

        self.crate_tilemap(ghost_tiles);

        match start_time.elapsed() {
            Ok(elapsed) => {
                println!(
                    "Map: `{}` has been loaded in {} ms.",
                    map_file_data["config"]["name"]
                        .to_string()
                        .replace(&['"'][..], ""),
                    elapsed.as_millis()
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        Ok(())
    }
    pub fn crate_tilemap(&mut self, transparent_tiles: Vec<f32>) {
        let mut bloclist: Vec<bloc::Bloc> = Vec::new();

        for (y, row) in self.map_file_content.iter().enumerate() {
            for (x, material) in row.iter().enumerate() {
                let tile = Tile::new(
                    (x as f32 * self.tile_size) as f32,
                    (y as f32 * self.tile_size) as f32,
                    self.tile_size,
                    *material,
                    transparent_tiles.contains(&(*material as f32)),
                    0.,
                );

                let (ok, new_bloc) = match material {
                    -1 => (true, Some(bloc::Bloc::Air(bloc::Air::new(0, tile)))),
                    4 => (true, Some(bloc::Bloc::Wall(bloc::Wall::new(0, tile)))),
                    _ => (false, None),
                };

                if ok {
                    bloclist.push(new_bloc.unwrap())
                } else {
                    println!(
                        "[WARNING] Failed to create bloc(x: {}, y:{}, material: {})",
                        x, y, material
                    )
                }
            }
        }
        self.bloc_list = bloclist;
        println!("Bloc list size: {}", self.bloc_list.len());
    }
    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        let draw_hitboxes = true;
        let draw_images = false;

        if draw_hitboxes {
            self.draw_hitboxes(ctx, draw_offset)?;
        }
        if draw_images {
            self.draw_images(ctx, draw_offset)?;
        }
        Ok(())
    }
    pub fn draw_images(
        &self,
        _ctx: &mut ggez::Context,
        _draw_offset: glam::Vec2,
    ) -> ggez::GameResult {
        Ok(())
    }
    pub fn draw_hitboxes(
        &self,
        ctx: &mut ggez::Context,
        draw_offset: glam::Vec2,
    ) -> ggez::GameResult {
        let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();

        for bloc in &self.bloc_list {
            let tile = match bloc {
                bloc::Bloc::Air(a) => &a.tile,
                bloc::Bloc::Wall(w) => &w.tile,
            };

            //  THIS IS TEMPORARY

            let color = match tile.material {
                -1 => ggez::graphics::Color::from_rgba(100, 100, 100, 255),
                4 => ggez::graphics::Color::from_rgba(0, 100, 200, 255),
                _ => ggez::graphics::Color::BLACK,
            };

            //  THIS IS TEMPORARY
            let rotated_tile_hitbox = tile.get_rotated_hitbox();
            hitbox_mesh.polyline(
                ggez::graphics::DrawMode::stroke(1.),
                &rotated_tile_hitbox, //&rotated_tile_hitbox.to_vec(),
                color,
            )?;
        }

        let builded = hitbox_mesh.build(ctx)?;

        ggez::graphics::draw(
            ctx,
            &builded,
            (draw_offset, 0., ggez::graphics::Color::WHITE),
        )?;
        Ok(())
    }
}

impl Tile {
    pub fn new(x: f32, y: f32, size: f32, material: i32, transparent: bool, a: f32) -> Self {
        Tile {
            hitbox: ggez::graphics::Rect::new(x, y, size, size),
            material: material,
            transparent: transparent,
            angle: a,
        }
    }
    pub fn get_rotated_hitbox(&self) -> Vec<glam::Vec2> {
        //physics::RotatedHitbox
        physics::rotate_square(self.hitbox, self.angle)
    }
}
