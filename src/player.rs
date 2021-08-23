use ggez;

use crate::input;

pub struct Player {
    id: i32,
    name: String,
    hitbox: ggez::graphics::Rect,
    inputs: input::Input,
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, id: &mut i32) -> Self {
        *id += 1;
        Player {
            id: *id - 1,
            name: "bob".to_string(),
            hitbox: ggez::graphics::Rect::new(x, y, w, h),
            inputs: input::Input::default(),
        }
    }
    pub fn player_movement() -> ggez::graphics::Rect {
        ggez::graphics::Rect::new(0., 0., 0., 0.)
    }
}
