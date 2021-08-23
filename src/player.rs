pub struct Player {
    id: i32
    name: String,

}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, id: &mut i32) -> Self {
        *id += 1;
        Player {
            id: id - 1,
            name: "bob".to_string(),
        }
    }

    pub fn player_movement()
}
