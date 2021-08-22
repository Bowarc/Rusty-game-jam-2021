use bevy::prelude::Vec3;

// pub fn normalise_vec3(vec: Vec3) -> Vec3 {
//     Vec3::new(0., 0., 0.)
// }

// pub fn normalize_p2(pt: na::Point2<f32>) -> na::Point2<f32> {
//     let d = ((pt.x * pt.x) + (pt.y * pt.y)).sqrt();
//     if d != 0.0 {
//         na::Point2::new(pt.x / d, pt.y / d)
//     } else {
//         pt
//     }
// }

pub struct Hitbox {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Hitbox {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Hitbox {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
}

pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

pub struct WorldCollision;

impl WorldCollision {
    pub fn two_rect(hb1: Hitbox, hb2: Hitbox) -> bool {
        let is_collision = hb1.x < hb2.x + hb2.w
            && hb1.x + hb1.h > hb2.x
            && hb1.y < hb1.y + hb2.h
            && hb1.h + hb1.y > hb2.y;
        is_collision
    }
    pub fn two_circle(circle1: Circle, circle2: Circle) -> bool {
        // https://developer.mozilla.org/fr/docs/Games/Techniques/2D_collision_detection
        let dx = circle1.x - circle2.x;
        let dy = circle1.y - circle2.y;
        let distance = (dx * dx + dy * dy).sqrt();

        distance < circle1.radius + circle2.radius
    }
    pub fn world_collision(
        entity_hitbox: Hitbox,
        wanted_pos: (f32, f32),
        collide_list: Vec<Hitbox>, //any object that has an hitbow
    ) -> Hitbox {
        let mut wanted_pos = wanted_pos.clone();
        for hb in collide_list {
            let dx = Hitbox::new(
                wanted_pos.0,
                entity_hitbox.y,
                entity_hitbox.w,
                entity_hitbox.h,
            );
            let dy = Hitbox::new(
                entity_hitbox.x,
                wanted_pos.1,
                entity_hitbox.w,
                entity_hitbox.h,
            );
            if WorldCollision::two_rect(hb, dx) {
                if entity_hitbox.x - wanted_pos.0 > 0.0 {
                    wanted_pos.0 = hb.x + hb.w;
                    // println!("Collided left");
                } else {
                    wanted_pos.0 = hb.x - entity_hitbox.w;
                    // println!("Collided right");
                }
            }
            if WorldCollision::two_rect(hb, dy) {
                if entity_hitbox.y - wanted_pos.1 > 0.0 {
                    // println!("Collided up");
                    wanted_pos.1 = hb.y + hb.h;
                } else {
                    // println!("Collided down");
                    wanted_pos.1 = hb.y - entity_hitbox.h;
                }
            }
        }
        Hitbox::new(wanted_pos.0, wanted_pos.1, entity_hitbox.w, entity_hitbox.h)
    }
}
