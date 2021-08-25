use crate::{bloc, id, physics};
use ggez;
use serde_json::Value; //Result
use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;
use noise::{
    utils::{
        NoiseMapBuilder,
        PlaneMapBuilder
    },
    SuperSimplex,
    Seedable,
};

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
    pub difficulty: u32,
}

pub struct Tile {
    pub hitbox: ggez::graphics::Rect,
    pub material: i32,
    pub transparent: bool,
    pub angle: f32,
}
impl Map {
    pub fn new(tile_size: f32) -> Self {
        Map {
            map_title: String::new(),
            tile_size: tile_size,
            map_file_content: Vec::new(),
            bloc_list: Vec::new(),
            ghost_tiles: Vec::new(),
            total_rows: 0.,
            total_cols: 0.,
            diag_size: 0.,
            image_hashmap: HashMap::new(),
            difficulty: 0,
        }
    }

    pub fn gen_new_map(
        &mut self,
        ctx: &mut ggez::Context,
        id_manager: id::IdManager,
    ) -> ggez::GameResult {
        const MAP_WIDTH: usize = 100;
        const MAP_HEIGHT: usize = 100;

        let start_time = SystemTime::now();
        println!("Loading map: {}", self.difficulty);

        let ghost_tiles: Vec<f32> = vec![-1., 10., 18., 19., 20., 21.];
        let tile_translate: HashMap<i32, String> = vec![
            (-1, "air".to_string()),
            (4, "wall".to_string()),
            (9, "water".to_string()),
            (12, "crate".to_string()),
            (18, "lava".to_string()),/*
            (19, "pack".to_string()),
            (20, "spawn".to_string()),
            (21, "end".to_string())*/
        ].into_iter().collect();

        let mut image_hashmap: HashMap<i32, ggez::graphics::spritebatch::SpriteBatch> =
            HashMap::new();
        for (key, value) in tile_translate.iter() {
            if value != "air" {
                let mut texture_file_name: String = value.to_string();
                texture_file_name.push_str(".png");
                let pth = format!("/tiles/{}", texture_file_name);
                println!("Loading: '{}'", pth);
                let image = ggez::graphics::Image::new(ctx, pth);
                image_hashmap.insert(
                    *key,
                    ggez::graphics::spritebatch::SpriteBatch::new(image.clone().unwrap()),
                );
            }
        }

        let simplex = SuperSimplex::default().set_seed(rand::random::<u32>());

        let noise_map = PlaneMapBuilder::new(&simplex)
            .set_size(MAP_WIDTH, MAP_HEIGHT)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        let mut map = Box::new([[0; MAP_WIDTH]; MAP_HEIGHT]);
        let mut line = [0; MAP_WIDTH];
        map[0] = [4; MAP_HEIGHT];
        map[MAP_HEIGHT - 1] = [4; MAP_HEIGHT];

        for i in 1..MAP_HEIGHT - 1 {
            for j in 1..MAP_WIDTH - 1 {
                line[0] = 4;
                line[MAP_WIDTH - 1] = 4;
                let level = noise_map.get_value(i, j);
                if level <= -0.6 {
                    if self.difficulty >= 5 && self.difficulty < 20 {
                        line[j] = 9;
                    }
                    else if self.difficulty >= 20 {
                        line[j] = 18;
                    }
                    else {
                        line[j] = -1;
                    }
                }
                else if level > -0.6 && level <= 0.5 {
                    line[j] = -1;
                }
                else {
                    line[j] = 4;
                }
            map[i] = line;
            }
        }
        let mut map_vec: Vec<Vec<i32>> = Vec::new();
        for i in 0..MAP_HEIGHT {
            map_vec.push(map[i].to_vec());
        }

        self.map_file_content = map_vec;
        self.total_rows = MAP_HEIGHT as f32;
        self.total_cols = MAP_WIDTH as f32;
        self.diag_size = physics::get_diagonal_size(self.total_cols, self.total_rows, self.tile_size);
        self.map_title = self.difficulty.to_string();
        self.image_hashmap = image_hashmap;
        self.crate_tilemap(ghost_tiles, id_manager);

        match start_time.elapsed() {
            Ok(elapsed) => {
                println!(
                    "Map: `{}` has been loaded in {} ms.",
                    self.map_title,
                    elapsed.as_millis()
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        Ok(())
    }

    pub fn crate_tilemap(&mut self, transparent_tiles: Vec<f32>, mut id_manager: id::IdManager) {
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
                    -1 => (
                        true,
                        Some(bloc::Bloc::Air(bloc::Air::new(
                            id_manager.get_new_id(),
                            tile,
                        ))),
                    ),
                    4 => (
                        true,
                        Some(bloc::Bloc::Wall(bloc::Wall::new(
                            id_manager.get_new_id(),
                            tile,
                        ))),
                    ),
                    9 => (
                        true,
                        Some(bloc::Bloc::Water(bloc::Water::new(
                            id_manager.get_new_id(),
                            tile,
                        ))),
                    ),
                    18 => (
                        true,
                        Some(bloc::Bloc::Lava(bloc::Lava::new(
                            id_manager.get_new_id(),
                            tile,
                        ))),
                    ),
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
    pub fn draw(&mut self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        let draw_hitboxes = false;
        let draw_images = true;

        if draw_hitboxes {
            self.draw_hitboxes(ctx, draw_offset)?;
        }
        if draw_images {
            self.draw_images(ctx, draw_offset)?;
        }
        Ok(())
    }
    pub fn draw_images(
        &mut self,
        ctx: &mut ggez::Context,
        draw_offset: glam::Vec2,
    ) -> ggez::GameResult {
        let tile_size_mult = self.tile_size / 32.;
        let rotation_offset = 0.;

        for bloc in self.bloc_list.iter() {
            let tile = match bloc {
                bloc::Bloc::Air(a) => &a.tile,
                bloc::Bloc::Wall(w) => &w.tile,
                bloc::Bloc::Water(w) => &w.tile,
                bloc::Bloc::Lava(l) => &l.tile,
            };

            if tile.material == -1 {
                continue;
            }

            let point = glam::Vec2::new(0.5, 0.5);
            let tile_drawparams = ggez::graphics::DrawParam::new()
                .dest(ggez::mint::Point2::from_slice(&[
                    tile.hitbox.center().x,
                    tile.hitbox.center().y,
                ]))
                .scale(ggez::mint::Vector2::from_slice(&[
                    tile_size_mult,
                    tile_size_mult,
                ]))
                .offset(point)
                .rotation(tile.angle + rotation_offset);
            let h = self.image_hashmap.get_mut(&(tile.material as i32)).unwrap();
            h.add(tile_drawparams);
        }

        for image in self.image_hashmap.clone().keys() {
            let h = self.image_hashmap.get_mut(image).unwrap();
            ggez::graphics::draw(ctx, h, (draw_offset, 0., ggez::graphics::Color::WHITE))?;
            h.clear();
        }
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
                bloc::Bloc::Water(w) => &w.tile,
                bloc::Bloc::Lava(l) => &l.tile,
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
