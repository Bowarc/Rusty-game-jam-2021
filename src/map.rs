use crate::physics;
use ggez;
use serde_json::Value; //Result
use std::collections::HashMap;
use std::io::Read;
// use std::time::SystemTime;
const MAP_FILE: &str = "map_settings.json";
pub struct Map {
    pub map_title: String,
    pub tile_size: f32,
    pub map_file_content: Vec<Vec<i32>>,
    pub ghost_tiles: Vec<f32>,
    pub total_rows: f32,
    pub total_cols: f32,
    pub diag_size: f32,
    pub image_hashmap: HashMap<i32, ggez::graphics::spritebatch::SpriteBatch>,
}

impl Map {
    pub fn new(tile_size: f32, current_id: &mut i32) -> Self {
        Map {
            map_title: "map title".to_string(),
            tile_size: tile_size,
            map_file_content: Vec::new(),
            ghost_tiles: Vec::new(),
            total_rows: 0.,
            total_cols: 0.,
            diag_size: 0.,
            image_hashmap: HashMap::new(),
        }
    }

    pub fn load_new_map(&mut self, map_name: String, ctx: &mut ggez::Context) -> ggez::GameResult {
        let map_file_path = format!("/maps/{}/{}", map_name, MAP_FILE);
        // let mut file = ggezfs::open(ctx, format!("/maps/{}/{}", map_name, MAP_FILE_NAME)).unwrap();
        let mut file = ggez::filesystem::open(ctx, map_file_path).unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let map_file_data: Value = serde_json::from_str(&data).unwrap();

        self.map_file_content =
            serde_json::from_str(&map_file_data["map_data"].to_string()).unwrap();
        self.total_rows = map_file_data["config"]["rows"].as_i64().unwrap() as f32;
        self.total_cols = map_file_data["config"]["cols"].as_i64().unwrap() as f32;
        self.diag_size =
            physics::get_diagonal_size(self.total_cols, self.total_rows, self.tile_size);
        self.map_title = map_file_data["config"]["name"].to_string();
        Ok(())
    }
}
