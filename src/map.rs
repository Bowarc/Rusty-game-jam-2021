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
        let map_file_path = format!("{}/{}/{}", "maps", map_name, MAP_FILE);
        let mut file = ggez::filesystem::open(ctx, map_file_path).unwrap();

        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        let map_file_data: Value = serde_json::from_str(&data).unwrap();

        self.map_file_content =
            serde_json::from_str(&map_file_data["world_data"].to_string()).unwrap();
        self.total_rows = map_file_data["config"]["rows"].as_i64().unwrap() as f32;
        self.total_cols = map_file_data["config"]["cols"].as_i64().unwrap() as f32;
        self.diag_size =
            physics::get_diagonal_size(self.total_cols, self.total_rows, self.tile_size);
        self.map_title = map_file_data["config"]["name"].to_string();
        Ok(())
    }
}

// let mut file = ggezfs::open(ctx, format!("/maps/{}/{}", map_name, MAP_FILE_NAME)).unwrap();
// let mut data = String::new();
// file.read_to_string(&mut data).unwrap();
// // Parse the string of data into serde_json::Value.
// let file_data: Value = serde_json::from_str(&data).unwrap();

// // Access parts of the data by indexing with square brackets.

// let mut translate: HashMap<i32, String> = HashMap::new();
// for a in file_data["tile_translate"].as_object() {
//     for (key, value) in a {
//         translate.insert(
//             key.parse::<i32>().unwrap(),
//             serde_json::from_str(&value.to_string()).unwrap(),
//         );
//         // println!("{} - {}", key, value);
//     }
// }

// let mut i_hashmap: HashMap<i32, graphics::spritebatch::SpriteBatch> = HashMap::new();
// for (key, value) in translate.iter() {
//     if value != "air" {
//         let mut texture_file_name: String = value.to_string();
//         texture_file_name.push_str(".png");
//         let pth = format!("/maps/{}/tiles/{}", map_name, texture_file_name);
//         println!("Loading: '{}'", pth);
//         let image = graphics::Image::new(ctx, pth);
//         i_hashmap.insert(
//             *key,
//             graphics::spritebatch::SpriteBatch::new(image.clone().unwrap()),
//         );
//     } else {
//         // println!("{} skipped", value);
//     }
// }
// let mut ghost_tiles: Vec<f32> = vec![];
// for i in file_data["transparent_tiles"].as_array().unwrap() {
//     ghost_tiles.push(i.as_f64().unwrap() as f32);
// }
// self.map_file_content = serde_json::from_str(&file_data["world_data"].to_string()).unwrap();
// self.total_rows = file_data["config"]["rows"].as_i64().unwrap() as f32;
// self.total_cols = file_data["config"]["cols"].as_i64().unwrap() as f32;
// self.diag_size =
//     ((self.total_cols.powf(2.) + self.total_rows.powf(2.)).sqrt()) * self.tile_size;
// self.map_title = file_data["config"]["name"].to_string();
// self.image_hashmap = i_hashmap;
// self.ghost_tiles = ghost_tiles.clone();
// self.crate_tilemap(ghost_tiles, translate, current_id);

// match start_time.elapsed() {
//     Ok(elapsed) => {
//         println!(
//             "Map: `{}` has been loaded in {} ms.",
//             file_data["config"]["name"]
//                 .to_string()
//                 .replace(&['"'][..], ""),
//             elapsed.as_millis()
//         );
//     }
//     Err(e) => {
//         println!("Error: {:?}", e);
//     }
// }
