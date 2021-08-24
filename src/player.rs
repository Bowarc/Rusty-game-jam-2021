use ggez;

use crate::{bloc, input, monster, physics};

const PLAYER_SPEED: f32 = 400.;
pub struct Player {
    pub id: i32,
    pub name: String,
    pub hitbox: ggez::graphics::Rect,
    pub inputs: input::Input,
    pub speed: f32,
    pub los: physics::LOS,
}

impl Player {
    pub fn new(x: f32, y: f32, w: f32, h: f32, _id: &mut i32) -> Self {
        Player {
            id: 0,
            name: "bob".to_string(),
            hitbox: ggez::graphics::Rect::new(x, y, w, h),
            inputs: input::Input::default(),
            speed: PLAYER_SPEED,
            los: physics::LOS::default(),
        }
    }
    pub fn update_movements(&mut self, bloclist: &mut Vec<bloc::Bloc>, dt: f32) {
        let mut dir = glam::Vec2::ZERO;
        let mut delta_pos = glam::Vec2::ZERO;
        if self.inputs.up {
            dir.y -= 1.;
        }
        if self.inputs.down {
            dir.y += 1.;
        }
        if self.inputs.left {
            dir.x -= 1.;
        }
        if self.inputs.right {
            dir.x += 1.;
        }
        dir = physics::normalize_point(dir);
        delta_pos.x += dir.x * (self.speed * dt);
        delta_pos.y += dir.y * (self.speed * dt);
        self.hitbox = physics::CheckCollision::world_collision(self.hitbox, delta_pos, bloclist);
        // println!("x: {}, y: {}", self.hitbox.x, self.hitbox.y);
    }
    pub fn update_los(
        &mut self,
        mouse_pos: ggez::mint::Point2<f32>,
        camera_scroll: glam::Vec2,
        bloclist: &mut Vec<bloc::Bloc>,
        monster_list: &mut Vec<monster::Monster>,
    ) {
        // Line of sight
        let hitbox_center = (
            -camera_scroll.x + self.hitbox.center().x,
            -camera_scroll.y + self.hitbox.center().y,
        );
        self.los.angle = (mouse_pos.y - hitbox_center.1).atan2(mouse_pos.x - hitbox_center.0);

        let weapon_range = 500.;

        let player_center = glam::Vec2::new(self.hitbox.center().x, self.hitbox.center().y);
        let rotated_line_end_point: glam::Vec2 = physics::rotate_line(
            player_center,
            glam::Vec2::new(
                self.hitbox.x + weapon_range + self.hitbox.w / 2. + self.hitbox.w,
                self.hitbox.y + self.hitbox.h / 2.,
            ),
            self.los.angle,
        );

        let line_of_sight: (glam::Vec2, glam::Vec2) = (player_center, rotated_line_end_point);

        let result =
            physics::RayCasting::ray_cast_tile_monster(line_of_sight, bloclist, monster_list);

        match result {
            physics::RayCastResult::Ok(line, ref _bloc, _dist) => {
                let new_pt = glam::Vec2::new(line.1.x, line.1.y);
                self.los.end_point = new_pt;
            }
            physics::RayCastResult::Fail => {
                self.los.end_point = line_of_sight.1;
                // println!("Player's ray cast failled");
            }
        }
        self.los.result = result;
    }
    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        let player_center =
            glam::Vec2::new(self.hitbox.center().x, self.hitbox.center().y) + draw_offset;

        let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();
        hitbox_mesh.rectangle(
            ggez::graphics::DrawMode::stroke(1.0),
            ggez::graphics::Rect::new(
                -self.hitbox.w / 2.0,
                -self.hitbox.h / 2.0,
                self.hitbox.w,
                self.hitbox.h,
            ),
            ggez::graphics::Color::RED,
        )?;
        let builded_hitbox_mesh = hitbox_mesh.build(ctx)?;

        ggez::graphics::draw(
            ctx,
            &builded_hitbox_mesh,
            (player_center, self.los.angle, ggez::graphics::Color::WHITE),
        )?;
        Ok(())
    }
}

impl physics::EntityTrait for Player {
    fn get_hitbox(&self) -> ggez::graphics::Rect {
        self.hitbox
    }
    fn ray_cast_bypass(&self) -> bool {
        false
    }
    fn rotated_hitbox(&self) -> Vec<glam::Vec2> {
        physics::rotate_square(self.hitbox, self.los.angle)
    }
    fn id(&self) -> i32 {
        self.id
    }
}
