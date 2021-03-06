use crate::{map, physics};
use std;

pub enum Bloc {
    Air(Air),
    Wall(Wall),
    Water(Water),
    Lava(Lava),
    Spawn(Spawn),
    End(End),
}

pub struct Air {
    pub id: i32,
    pub tile: map::Tile,
}

pub struct Wall {
    pub id: i32,
    pub tile: map::Tile,
}

pub struct Water {
    pub id: i32,
    pub tile: map::Tile,
}

pub struct Lava {
    pub id: i32,
    pub tile: map::Tile,
    pub damage: i32,
    pub damage_speed: i32,
    pub id_time_list: std::collections::HashMap<i32, std::time::SystemTime>,
}

pub struct Spawn {
    pub id: i32,
    pub tile: map::Tile,
}

pub struct End {
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

impl Water {
    pub fn new(id: i32, tile: map::Tile) -> Self {
        Self { id: id, tile: tile }
    }
}

impl Spawn {
    pub fn new(id: i32, tile: map::Tile) -> Self {
        Self { id: id, tile: tile }
    }
}

impl End {
    pub fn new(id: i32, tile: map::Tile) -> Self {
        Self { id: id, tile: tile }
    }
}

impl Lava {
    pub fn new(id: i32, tile: map::Tile) -> Self {
        Lava {
            id: id,
            tile: tile,
            damage: 5,
            damage_speed: 500, // hit cooldown
            id_time_list: std::collections::HashMap::new(),
        }
    }
    pub fn damage<E: physics::EntityTrait>(&mut self, entity: &mut E) {
        self.update();
        if !self.id_time_list.contains_key(&entity.id()) {
            self.id_time_list
                .insert(entity.id(), std::time::SystemTime::now());
            entity.take_damage(self.damage);
        }
    }

    pub fn update(&mut self) {
        for (id, time) in self.id_time_list.clone().iter() {
            match time.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_millis() > self.damage_speed as u128 {
                        self.id_time_list.remove(&id);
                    }
                }
                Err(e) => {
                    println!("There has been an error in the lava update function: {}", e);
                }
            }
        }
    }
}

impl physics::EntityTrait for Bloc {
    fn get_hitbox(&self) -> ggez::graphics::Rect {
        match self {
            Bloc::Air(a) => a.tile.hitbox,
            Bloc::Wall(w) => w.tile.hitbox,
            Bloc::Water(w) => w.tile.hitbox,
            Bloc::Lava(l) => l.tile.hitbox,
            Bloc::Spawn(s) => s.tile.hitbox,
            Bloc::End(e) => e.tile.hitbox,
        }
    }
    fn get_angle(&self) -> f32 {
        match self {
            Bloc::Air(a) => a.tile.angle,
            Bloc::Wall(w) => w.tile.angle,
            Bloc::Water(w) => w.tile.angle,
            Bloc::Lava(l) => l.tile.angle,
            Bloc::Spawn(s) => s.tile.angle,
            Bloc::End(e) => e.tile.angle,
        }
    }
    fn ray_cast_bypass(&self) -> bool {
        match self {
            Bloc::Air(a) => a.tile.transparent,
            Bloc::Wall(w) => w.tile.transparent,
            Bloc::Water(w) => w.tile.transparent,
            Bloc::Lava(l) => l.tile.transparent,
            Bloc::Spawn(s) => s.tile.transparent,
            Bloc::End(e) => e.tile.transparent,
        }
    }
    fn rotated_hitbox(&self) -> Vec<glam::Vec2> {
        match self {
            Bloc::Air(a) => physics::rotate_square(a.tile.hitbox, a.tile.angle),
            Bloc::Wall(w) => physics::rotate_square(w.tile.hitbox, w.tile.angle),
            Bloc::Water(w) => physics::rotate_square(w.tile.hitbox, w.tile.angle),
            Bloc::Lava(l) => physics::rotate_square(l.tile.hitbox, l.tile.angle),
            Bloc::Spawn(s) => physics::rotate_square(s.tile.hitbox, s.tile.angle),
            Bloc::End(e) => physics::rotate_square(e.tile.hitbox, e.tile.angle),
        }
    }
    fn id(&self) -> i32 {
        match self {
            Bloc::Air(a) => a.id,
            Bloc::Wall(w) => w.id,
            Bloc::Water(w) => w.id,
            Bloc::Lava(l) => l.id,
            Bloc::Spawn(s) => s.id,
            Bloc::End(e) => e.id,
        }
    }
    fn take_damage(&mut self, _damage: i32) {}
}
