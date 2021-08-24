pub struct ID_manager {
    current_id: i32,
}

impl ID_manager {
    pub fn new() -> Self {
        ID_manager { current_id: 0 }
    }
    pub fn get_new_id(&mut self) -> i32 {
        self.current_id += 1;
        self.current_id
    }
}
