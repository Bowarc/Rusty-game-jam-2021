use ggez::graphics;
use std::collections::HashMap;
pub struct Map {
    pub map_title: String,
    pub tile_size: f32,
    pub map_file_content: Vec<Vec<i32>>,
    pub ghost_tiles: Vec<f32>,
    pub total_rows: f32,
    pub total_cols: f32,
    pub diag_size: f32,
    pub image_hashmap: HashMap<i32, graphics::spritebatch::SpriteBatch>,
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
}
