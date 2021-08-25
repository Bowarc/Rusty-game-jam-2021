// use crate::bloc::Bloc;
use crate::{id, physics};
use ggez;
use glam;

const TEST_BOT_SPEED: f32 = 350.;

pub enum MonsterType {
    TestBot,
}
pub enum Monster {
    TestBot(TestBot),
}
pub struct MonsterManager {
    pub monster_list: Vec<Monster>,
}

pub struct TestBot {
    pub id: i32,
    pub name: String,
    pub hitbox: ggez::graphics::Rect,
    pub speed: f32,
    pub los: physics::LOS,
}

impl MonsterManager {
    pub fn new() -> Self {
        MonsterManager {
            monster_list: Vec::new(),
        }
    }
    pub fn new_bot(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        monster_type: MonsterType,
        mut id_manager: id::IdManager,
    ) {
        let new_monster = match monster_type {
            MonsterType::TestBot => {
                Monster::TestBot(TestBot::new(x, y, w, h, id_manager.get_new_id()))
            }
        };

        self.monster_list.push(new_monster);
    }

    pub fn draw_bots(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();

        for i in 0..self.monster_list.len() {
            let hitbox_lines = physics::rotate_square(
                physics::EntityTrait::get_hitbox(&self.monster_list[i]),
                physics::EntityTrait::get_angle(&self.monster_list[i]),
            );
            hitbox_mesh.polyline(
                ggez::graphics::DrawMode::stroke(1.),
                &hitbox_lines,
                ggez::graphics::Color::WHITE,
            )?;
        }

        let builded_hitbox_mesh = hitbox_mesh.build(ctx)?;

        ggez::graphics::draw(
            ctx,
            &builded_hitbox_mesh,
            (draw_offset, 0., ggez::graphics::Color::WHITE),
        )?;
        Ok(())
    }
}

impl TestBot {
    pub fn new(x: f32, y: f32, w: f32, h: f32, id: i32) -> Self {
        TestBot {
            id: id,
            name: "monster_name".to_string(),
            hitbox: ggez::graphics::Rect::new(x, y, w, h),
            speed: TEST_BOT_SPEED,
            los: physics::LOS::default(),
        }
    }
}

impl physics::EntityTrait for Monster {
    fn get_hitbox(&self) -> ggez::graphics::Rect {
        match self {
            Monster::TestBot(tb) => tb.hitbox,
        }
    }
    fn get_angle(&self) -> f32 {
        match self {
            Monster::TestBot(tb) => tb.los.angle,
        }
    }
    fn ray_cast_bypass(&self) -> bool {
        match self {
            Monster::TestBot(_tb) => false,
        }
    }
    fn rotated_hitbox(&self) -> Vec<glam::Vec2> {
        match self {
            Monster::TestBot(tb) => physics::rotate_square(tb.hitbox, tb.los.angle),
        }
    }
    fn id(&self) -> i32 {
        match self {
            Monster::TestBot(tb) => tb.id,
        }
    }
}
