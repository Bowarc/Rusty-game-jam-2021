use ggez;
use ggez::audio::SoundSource;

use crate::{bloc, id, input, monster, physics, weapon};

const PLAYER_SPEED: f32 = 400.;
const PLAYER_BASE_HP: i32 = 100;

pub struct Player {
    pub id: i32,
    pub hp: i32,
    pub name: String,
    pub hitbox: ggez::graphics::Rect,
    pub inputs: input::Input,
    pub speed: f32,
    pub los: physics::LOS,
    pub inventory: weapon::WeaponInventory,
    pub shot_sound: ggez::audio::Source,
}

impl Player {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        id_manager: &mut id::IdManager,
        ctx: &mut ggez::Context,
    ) -> Self {
        let shot_sound = ggez::audio::Source::new(ctx, "/sounds/pistol.wav").unwrap();
        Player {
            id: id_manager.get_new_id(),
            hp: PLAYER_BASE_HP,
            name: "bob".to_string(),
            hitbox: ggez::graphics::Rect::new(x, y, w, h),
            inputs: input::Input::default(),
            speed: PLAYER_SPEED,
            los: physics::LOS::default(),
            inventory: weapon::WeaponInventory::new(id_manager),
            shot_sound,
        }
    }
    pub fn update_movements(
        &mut self,
        bloclist: &mut Vec<bloc::Bloc>,
        dt: f32,
        id_manager: &mut id::IdManager,
        monster_manager: &mut monster::MonsterManager,
        ctx: &mut ggez::Context,
    ) {
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

        if self.inputs.mouse_left || self.inputs.controler_south {
            self.shoot(id_manager, monster_manager, ctx);
        }
    }
    pub fn update_los(
        &mut self,
        camera_scroll: glam::Vec2,
        bloclist: &mut Vec<bloc::Bloc>,
        monster_list: &mut Vec<monster::Monster>,
    ) {
        // Line of sight
        let hitbox_center = (
            -camera_scroll.x + self.hitbox.center().x,
            -camera_scroll.y + self.hitbox.center().y,
        );
        self.los.angle = physics::two_points_angle(
            glam::Vec2::from(hitbox_center),
            glam::Vec2::from(self.inputs.pointing),
        );
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
            }
        }
        self.los.result = result;
    }
    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        let player_center =
            glam::Vec2::new(self.hitbox.center().x, self.hitbox.center().y) + draw_offset;

        // Used to see the orientation, code from from Heto's game
        let mut hitbox_mesh = ggez::graphics::MeshBuilder::new();
        let mut los_mesh = ggez::graphics::MeshBuilder::new();

        let gun_rect =
            ggez::graphics::Rect::new(self.hitbox.w / 2.0 - 10., self.hitbox.h / 2.0, 30.0, 10.0);

        hitbox_mesh.rectangle(
            ggez::graphics::DrawMode::stroke(1.0),
            gun_rect,
            ggez::graphics::Color::WHITE,
        )?;

        // End of debug code

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
        los_mesh.circle(
            ggez::graphics::DrawMode::stroke(10.),
            self.los.end_point,
            5.,
            0.1,
            ggez::graphics::Color::from_rgb(0, 100, 100),
        )?;

        let builded_hitbox_mesh = hitbox_mesh.build(ctx)?;
        ggez::graphics::draw(
            ctx,
            &builded_hitbox_mesh,
            (player_center, self.los.angle, ggez::graphics::Color::WHITE),
        )?;

        let builded_los_mesh = los_mesh.build(ctx)?;
        ggez::graphics::draw(
            ctx,
            &builded_los_mesh,
            (draw_offset, 0., ggez::graphics::Color::WHITE),
        )?;

        Ok(())
    }
    pub fn take_damages(&mut self, damage: i32) -> bool {
        self.hp -= damage;
        println!("Player's hp: {}", self.hp);
        if self.hp < 1 {
            println!("Player is supposed to be dead");
            true
        } else {
            false
        }
    }
    pub fn shoot(
        &mut self,
        id_manager: &mut id::IdManager,
        monster_manager: &mut monster::MonsterManager,
        ctx: &mut ggez::Context,
    ) -> weapon::ObjectDrop {
        let mut dropped_item = weapon::ObjectDrop::None;

        if self.inventory.index_is_weapon() {
            match &mut self.inventory.weapon_list[self.inventory.selected_index] {
                weapon::Weapon::Pistol(p) => {
                    if p.can_shoot() {
                        self.shot_sound.play(ctx).unwrap();
                        match self.los.result.clone() {
                            physics::RayCastResult::Ok(_line, object, _dist) => match object {
                                physics::RayCastBlocType::Monster(monster_index) => {
                                    dropped_item = monster_manager.damage_monster_isdead(
                                        monster_index,
                                        p.damage,
                                        id_manager,
                                    );
                                }
                                _ => {}
                            },
                            physics::RayCastResult::Fail => {}
                        }
                    }
                }
                _ => {}
            }
        }

        dropped_item
    }
}

impl physics::EntityTrait for Player {
    fn get_hitbox(&self) -> ggez::graphics::Rect {
        self.hitbox
    }
    fn get_angle(&self) -> f32 {
        self.los.angle
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
    fn take_damage(&mut self, damage: i32) {
        self.take_damages(damage);
    }
}
