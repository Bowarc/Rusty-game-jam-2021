use crate::{map, physics};

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

impl physics::EntityTrait for Bloc {
    fn get_hitbox(&self) -> ggez::graphics::Rect {
        match self {
            Bloc::Air(a) => a.tile.hitbox,
            Bloc::Wall(w) => w.tile.hitbox,
        }
    }
    fn get_angle(&self) -> f32 {
        match self {
            Bloc::Air(a) => a.tile.angle,
            Bloc::Wall(w) => w.tile.angle,
        }
    }
    fn ray_cast_bypass(&self) -> bool {
        match self {
            Bloc::Air(a) => a.tile.transparent,
            Bloc::Wall(w) => w.tile.transparent,
        }
    }
    fn rotated_hitbox(&self) -> Vec<glam::Vec2> {
        match self {
            Bloc::Air(a) => physics::rotate_square(a.tile.hitbox, a.tile.angle),
            Bloc::Wall(w) => physics::rotate_square(w.tile.hitbox, w.tile.angle),
        }
    }
    fn id(&self) -> i32 {
        match self {
            Bloc::Air(a) => a.id,
            Bloc::Wall(w) => w.id,
        }
    }
}
