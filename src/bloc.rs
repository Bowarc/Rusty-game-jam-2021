use crate::map;

// #[derive(Debug)]
pub enum Bloc {
    Air(Air),
    Wall(Wall),
}

pub struct Air {
    pub id: i32,
    pub tile: map::Tile,
}

pub struct Wall {
    pub id: i32,
    pub tile: map::Tile,
}

impl Air {
    pub fn new(id: i32, tile: map::Tile) -> Self {
        Air { id: id, tile: tile }
    }
}

impl Wall {
    pub fn new(id: i32, tile: map::Tile) -> Self {
        Wall { id: id, tile: tile }
    }
}
