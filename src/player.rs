use ggez;

use crate::{bloc, input, physics};

const PLAYER_SPEED: f32 = 400.;
pub struct Player {
    pub id: i32,
    pub name: String,
    pub hitbox: ggez::graphics::Rect,
    pub inputs: input::Input,
    pub speed: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, id: &mut i32) -> Self {
        Player {
            id: 0,
            name: "bob".to_string(),
            hitbox: ggez::graphics::Rect::new(x, y, w, h),
            inputs: input::Input::default(),
            speed: PLAYER_SPEED,
            angle: 0.,
        }
    }
    pub fn update_movements(&mut self, bloclist: &mut Vec<bloc::Bloc>, dt: f32) {
        let mut dir = glam::Vec2::ZERO;
        let mut delta_pos = glam::Vec2::ZERO;

        if self.inputs.key_z {
            dir.y -= 1.;
        }
        if self.inputs.key_s {
            dir.y += 1.;
        }
        if self.inputs.key_q {
            dir.x -= 1.;
        }
        if self.inputs.key_d {
            dir.x += 1.;
        }

        dir = physics::normalize_point(dir);
        delta_pos.x += dir.x * (self.speed * dt);
        delta_pos.y += dir.y * (self.speed * dt);
        self.hitbox = physics::CheckCollision::world_collision(self.hitbox, delta_pos, bloclist);
        println!("x: {}, y: {}", self.hitbox.x, self.hitbox.y);
    }
    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();
        hitbox_mesh.rectangle(
            ggez::graphics::DrawMode::stroke(1.0),
            ggez::graphics::Rect::new(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h),
            ggez::graphics::Color::RED,
        )?;
        let builded_hitbox_mesh = hitbox_mesh.build(ctx)?;

        ggez::graphics::draw(
            ctx,
            &builded_hitbox_mesh,
            (draw_offset, self.angle, ggez::graphics::Color::WHITE),
        )?;
        Ok(())
    }
}

impl physics::CollisionEntity for Player {
    fn get_hitbox(&self) -> ggez::graphics::Rect {
        self.hitbox
    }
}
