#[derive(Clone, Copy)]
pub struct IdManager {
    current_id: i32,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager { current_id: 0 }
    }
    pub fn get_new_id(&mut self) -> i32 {
        self.current_id += 1;
        self.current_id
    }
}
