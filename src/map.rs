#[derive(Default)]
pub struct Map {
    raw_map: Vec<Vec<i32>>,
}

impl Map {
    fn load_new_map(&mut self, map_name: String) {
        println!("Loading map: {}", map_name);
    }
}
