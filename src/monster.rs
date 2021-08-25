// use crate::bloc::Bloc;
use crate::{id, physics};
use ggez;
use glam;
use rand::Rng;
const TEST_BOT_SPEED: f32 = 350.;
const VISION_CONE: f32 = 100.;

pub enum MonsterType {
    TestBot,
}
pub enum Monster {
    TestBot(TestBot),
}
pub struct Brain {
    pub iq: i32,
    pub close_vision_circle: physics::Circle,
    pub large_vision_circle: physics::Circle,
    pub vision_cone: (f32, f32),
}

pub struct MonsterManager {
    pub monster_list: Vec<Monster>,
}

pub struct TestBot {
    pub id: i32,
    pub name: String,
    pub hp: i32,
    pub hitbox: ggez::graphics::Rect,
    pub speed: f32,
    pub los: physics::LOS,
    pub brain: Brain,
}

impl Brain {
    pub fn new() -> Self {
        let iq = rand::thread_rng().gen_range(60..160);
        Brain {
            iq: iq,
            close_vision_circle: physics::Circle::new(glam::Vec2::ZERO, 0.),
            large_vision_circle: physics::Circle::new(glam::Vec2::ZERO, 0.),
            vision_cone: (0., 0.),
        }
    }
    pub fn update(&mut self, entity_pos: glam::Vec2, entity_angle: f32) {
        let usable_angle = entity_angle.to_degrees();

        self.vision_cone = (
            usable_angle - (VISION_CONE * 2.),
            usable_angle + (VISION_CONE * 2.),
        );

        self.close_vision_circle = physics::Circle::new(entity_pos, self.iq as f32);

        self.large_vision_circle = physics::Circle::new(entity_pos, (self.iq * 3) as f32);
    }
    pub fn can_see(
        &mut self,
        entity_pos: glam::Vec2,
        entity_angle: f32,
        point: glam::Vec2,
    ) -> bool {
        //  Generate the view area
        //  https://cdn.discordapp.com/attachments/406461353537175573/880003880681869342/unknown.png
        self.update(entity_pos, entity_angle);

        if physics::CheckCollision::point_in_circle(point, self.close_vision_circle) {
            // In the little circle
            println!("I SEE YOUUUU");
            return true;
        }

        if physics::CheckCollision::point_in_circle(point, self.large_vision_circle) {
            let angle_entity_point = physics::RayCasting::get_distance(entity_pos, point);
            if angle_entity_point < self.vision_cone.1 || angle_entity_point > self.vision_cone.0 {
                println!("I SEE YOUUUU");
                // In the vision cone
                return true;
            } else {
                // Not in the vision cone
                return false;
            }
        } else {
            // Not in the outside circle
            return false;
        }
    }
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
        let brain = Brain::new();
        let new_monster = match monster_type {
            MonsterType::TestBot => {
                Monster::TestBot(TestBot::new(x, y, w, h, id_manager.get_new_id(), brain))
            }
        };

        self.monster_list.push(new_monster);
    }

    pub fn update(&mut self) {
        for i in 0..self.monster_list.len() {
            match &mut self.monster_list[i] {
                Monster::TestBot(tb) => tb.update(),
            }
        }
    }
    pub fn draw_monsters(
        &self,
        ctx: &mut ggez::Context,
        draw_offset: glam::Vec2,
    ) -> ggez::GameResult {
        let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();
        let mut vision_circles_mesh = ggez::graphics::MeshBuilder::new();

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
            let (close_circle, large_circle) = match &self.monster_list[i] {
                Monster::TestBot(tb) => {
                    (tb.brain.close_vision_circle, tb.brain.large_vision_circle)
                }
            };
            vision_circles_mesh.circle(
                ggez::graphics::DrawMode::stroke(2.),
                close_circle.center,
                close_circle.radius,
                0.1,
                ggez::graphics::Color::WHITE,
            )?;
            vision_circles_mesh.circle(
                ggez::graphics::DrawMode::stroke(2.),
                large_circle.center,
                large_circle.radius,
                0.1,
                ggez::graphics::Color::WHITE,
            )?;
        }

        let builded_hitbox_mesh = hitbox_mesh.build(ctx)?;

        let builded_vision_mesh = vision_circles_mesh.build(ctx)?;

        ggez::graphics::draw(
            ctx,
            &builded_hitbox_mesh,
            (draw_offset, 0., ggez::graphics::Color::WHITE),
        )?;

        ggez::graphics::draw(
            ctx,
            &builded_vision_mesh,
            (draw_offset, 0., ggez::graphics::Color::WHITE),
        )?;
        Ok(())
    }
}

impl TestBot {
    pub fn new(x: f32, y: f32, w: f32, h: f32, id: i32, mut brain: Brain) -> Self {
        let hitbox = ggez::graphics::Rect::new(x, y, w, h);
        let los = physics::LOS::default();
        brain.update(glam::Vec2::from(hitbox.center()), los.angle);
        println!("iq: {}", brain.iq);
        TestBot {
            id: id,
            hp: 100,
            name: "monster_name".to_string(),
            hitbox: hitbox,
            speed: TEST_BOT_SPEED,
            los: los,
            brain: brain,
        }
    }
    pub fn take_damages(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 1 {
            println!("TestBot with id: {id} should be dead", id = self.id);
        }
    }

    pub fn update(&mut self) {
        self.brain
            .update(glam::Vec2::from(self.hitbox.center()), self.los.angle);
        // update(&mut self, entity_pos: glam::Vec2, entity_angle: f32)
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
    fn take_damage(&mut self, damage: i32) {
        match self {
            Monster::TestBot(tb) => tb.take_damages(damage),
        }
    }
}
