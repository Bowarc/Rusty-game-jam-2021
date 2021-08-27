use crate::{bloc, id, physics, weapon};
use ggez;
use glam;
use rand::Rng;
const TEST_BOT_SPEED: f32 = 250.;
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
    pub see_something: bool,
    pub wandering_path: Vec<glam::Vec2>,
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
        id_manager: &mut id::IdManager,
    ) {
        let brain = Brain::new();
        let new_monster = match monster_type {
            MonsterType::TestBot => {
                Monster::TestBot(TestBot::new(x, y, w, h, id_manager.get_new_id(), brain))
            }
        };

        self.monster_list.push(new_monster);
    }

    pub fn update(&mut self, player_pos: glam::Vec2) {
        for i in 0..self.monster_list.len() {
            match &mut self.monster_list[i] {
                Monster::TestBot(tb) => tb.update(player_pos),
            }
        }
    }
    pub fn update_movements(
        &mut self,
        dt: f32,
        bloc_list: &Vec<bloc::Bloc>,
        map_infos: (Vec<Vec<i32>>, Vec<f32>, f32),
    ) {
        let mut pathfinding_count = 0;
        let pathfinding_threshold = 3;

        for i in 0..self.monster_list.len() {
            match &mut self.monster_list[i] {
                Monster::TestBot(tb) => {
                    if tb.brain.wandering_path.is_empty()
                        && pathfinding_count < pathfinding_threshold
                    {
                        println!("Creating path for bot id: {}", tb.id);
                        pathfinding_count += 1;
                        // let raw_map = map

                        let random_desired_pos = glam::Vec2::new(
                            rand::thread_rng().gen_range(
                                1. * map_infos.2..(map_infos.0[0].len() as f32 - 1.) * map_infos.2,
                            ),
                            rand::thread_rng().gen_range(
                                1. * map_infos.2..(map_infos.0.len() as f32 - 1.) * map_infos.2,
                            ),
                        );
                        let path_result = physics::PathFinding::astar(
                            glam::Vec2::from(tb.hitbox.center()),
                            random_desired_pos,
                            map_infos.clone(),
                        );
                        match path_result {
                            physics::PathFindingResult::Ok(path) => {
                                tb.brain.wandering_path = path;
                            }
                            physics::PathFindingResult::Fail => {
                                println!("Failed pathfinding for bot id: {}", tb.id);
                            }
                        }
                    }
                    tb.update_movements(dt, bloc_list);
                }
            }
        }
    }

    pub fn damage_monster_isdead(
        &mut self,
        monster_index: usize,
        damage: i32,
        id_manager: &mut id::IdManager,
    ) -> weapon::ObjectDrop {
        let mut monster_is_dead = false;

        let drop: weapon::ObjectDrop;

        match &mut self.monster_list[monster_index] {
            Monster::TestBot(tb) => {
                tb.take_damages(damage);
                // println!("Bot with id: {} has been damaged", tb.id);
                if tb.is_dead() {
                    monster_is_dead = true;
                };
                drop = weapon::generate_drop(id_manager);
            }
        }

        if monster_is_dead {
            self.monster_list.swap_remove(monster_index);
        }

        drop
    }

    pub fn draw_monsters(
        &self,
        ctx: &mut ggez::Context,
        draw_offset: glam::Vec2,
    ) -> ggez::GameResult {
        if !self.monster_list.is_empty() {
            let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();
            let mut vision_circles_mesh = ggez::graphics::MeshBuilder::new();

            for i in 0..self.monster_list.len() {
                let monster_hitbox = physics::EntityTrait::get_hitbox(&self.monster_list[i]);
                let hitbox_lines = physics::rotate_square(
                    physics::EntityTrait::get_hitbox(&self.monster_list[i]),
                    physics::EntityTrait::get_angle(&self.monster_list[i]),
                );

                let (close_circle, large_circle, vision_cone, iq, see_something) =
                    match &self.monster_list[i] {
                        Monster::TestBot(tb) => (
                            tb.brain.close_vision_circle,
                            tb.brain.large_vision_circle,
                            tb.brain.vision_cone,
                            tb.brain.iq,
                            tb.brain.see_something,
                        ),
                    };

                let accent_color: ggez::graphics::Color;
                if see_something {
                    accent_color = ggez::graphics::Color::RED;
                } else {
                    accent_color = ggez::graphics::Color::WHITE;
                }
                let cone_0_endpoint_r: glam::Vec2 = physics::rotate_line(
                    glam::Vec2::from(monster_hitbox.center()),
                    glam::Vec2::new(
                        monster_hitbox.x + iq as f32 * 3. + monster_hitbox.w / 2.,
                        monster_hitbox.y + monster_hitbox.h / 2.,
                    ),
                    vision_cone.0,
                );

                let cone_1_endpoint_r: glam::Vec2 = physics::rotate_line(
                    glam::Vec2::from(monster_hitbox.center()),
                    glam::Vec2::new(
                        monster_hitbox.x + iq as f32 * 3. + monster_hitbox.w / 2.,
                        monster_hitbox.y + monster_hitbox.h / 2.,
                    ),
                    vision_cone.1,
                );

                hitbox_mesh.polyline(
                    ggez::graphics::DrawMode::stroke(1.),
                    &hitbox_lines,
                    accent_color,
                )?;

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
                vision_circles_mesh.line(
                    &[glam::Vec2::from(monster_hitbox.center()), cone_0_endpoint_r],
                    1.,
                    ggez::graphics::Color::WHITE,
                )?;
                vision_circles_mesh.line(
                    &[glam::Vec2::from(monster_hitbox.center()), cone_1_endpoint_r],
                    1.,
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
        }

        Ok(())
    }
}

impl Brain {
    pub fn new() -> Self {
        let iq = rand::thread_rng().gen_range(60..160);
        Brain {
            iq: iq,
            close_vision_circle: physics::Circle::new(glam::Vec2::ZERO, 0.),
            large_vision_circle: physics::Circle::new(glam::Vec2::ZERO, 0.),
            vision_cone: (0., 0.),
            see_something: false,
            wandering_path: Vec::new(),
        }
    }
    pub fn update(&mut self, entity_pos: glam::Vec2, entity_angle: f32) {
        let usable_angle = entity_angle;

        self.vision_cone = (
            usable_angle - (VISION_CONE.to_radians() / 2.),
            usable_angle + (VISION_CONE.to_radians() / 2.),
        );

        self.close_vision_circle = physics::Circle::new(entity_pos, self.iq as f32);

        self.large_vision_circle = physics::Circle::new(entity_pos, (self.iq * 3) as f32);
    }
    pub fn can_see(&mut self, entity_pos: glam::Vec2, point: glam::Vec2) -> bool {
        //  Generate the view area
        //  https://cdn.discordapp.com/attachments/406461353537175573/880003880681869342/unknown.png
        //

        let mut result = false;
        if physics::CheckCollision::point_in_circle(point, self.close_vision_circle) {
            // In the little circle
            result = true;
        }
        if !result {
            if physics::CheckCollision::point_in_circle(point, self.large_vision_circle) {
                let mut angle_entity_point =
                    physics::two_points_angle(entity_pos, point).to_degrees();
                let mut cone: (f32, f32) = (
                    self.vision_cone.0.to_degrees(),
                    self.vision_cone.1.to_degrees(),
                );
                if cone.0 < 0. {
                    cone.0 = cone.0 + 360.;
                }
                if cone.1 < 0. {
                    cone.1 = cone.1 + 360.;
                }
                if angle_entity_point < 0. {
                    angle_entity_point = angle_entity_point + 360.;
                }
                // println!(
                //     "Vision cone: {}, {}.\n angle player: {}",
                //     cone.0, cone.1, angle_entity_point
                // );
                let condition2 = (cone.0 < angle_entity_point && angle_entity_point < cone.1)
                    || (cone.1 < cone.0
                        && ((cone.0 < angle_entity_point && angle_entity_point > 0.)
                            || (angle_entity_point < cone.1 && angle_entity_point >= 0.)));
                if condition2 {
                    // In the vision cone
                    result = true;
                } else {
                    // Not in the vision cone
                    result = false;
                }
            } else {
                // Not in the outside circle
                result = false
            }
        }
        if result {
            // println!("I SEE YOUUUU");
            self.see_something = true
        } else {
            // println!("WHEEERREE AAARRREE YOUUUUU");
            self.see_something = false
        }
        result
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
    }
    pub fn is_dead(&self) -> bool {
        if self.hp < 1 {
            // println!("TestBot with id: {id} should be dead", id = self.id);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, player_pos: glam::Vec2) {
        self.los.angle += 0.006; // make it rotate
        if self.los.angle as f64 > std::f64::consts::PI {
            self.los.angle = -std::f64::consts::PI as f32;
        }

        if self.brain.see_something {
            self.los.angle =
                physics::two_points_angle(glam::Vec2::from(self.hitbox.center()), player_pos);
        }
        self.brain
            .update(glam::Vec2::from(self.hitbox.center()), self.los.angle);
        self.brain
            .can_see(glam::Vec2::from(self.hitbox.center()), player_pos);
    }

    pub fn update_movements(&mut self, dt: f32, bloc_list: &Vec<bloc::Bloc>) {
        if !self.brain.wandering_path.is_empty() {
            let desired_position = self.brain.wandering_path[0];

            let my_pos = self.hitbox.center();

            let mut direction =
                glam::Vec2::new(desired_position.x - my_pos.x, desired_position.y - my_pos.y);

            let mut delta_pos = glam::Vec2::ZERO;

            direction = physics::normalize_point(direction);

            let mut speed = TEST_BOT_SPEED * dt;

            let distance_to_desired_position =
                physics::RayCasting::get_distance(glam::Vec2::from(my_pos), desired_position);

            if distance_to_desired_position < speed {
                speed = distance_to_desired_position;
            }

            delta_pos.x += direction.x * speed;
            delta_pos.y += direction.y * speed;

            let new_hitbox = physics::CheckCollision::world_collision(
                self.hitbox,
                glam::Vec2::new(delta_pos.x, delta_pos.y),
                &bloc_list,
            );

            if self.hitbox == new_hitbox {
                println!("DOOOOR STUCK, id: {}", self.id);
                self.brain.wandering_path = Vec::new();
            } else {
                self.hitbox = new_hitbox;
            }
        }
        if !self.brain.wandering_path.is_empty() {
            let d = physics::RayCasting::get_distance(
                glam::Vec2::from(self.hitbox.center()),
                self.brain.wandering_path[0],
            );
            if d < 1. {
                self.brain.wandering_path.remove(0);
            }
        };
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
