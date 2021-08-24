// use crate::bloc::Bloc;
use crate::physics;
use ggez;

const TEST_BOT_SPEED: f32 = 350.;

pub enum Monster {
    TestBot(TestBot),
}
pub struct TestBot {
    pub id: i32,
    pub name: String,
    pub hitbox: ggez::graphics::Rect,
    pub speed: f32,
    pub los: physics::LOS,
}

impl TestBot {
    pub fn new(x: f32, y: f32, w: f32, h: f32, _id: &mut i32) -> Self {
        TestBot {
            id: 0,
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
