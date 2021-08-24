use ggez;
use glam;

use crate::{bloc, map};

pub trait CollisionEntity {
    fn get_hitbox(&self) -> ggez::graphics::Rect;
}
pub struct CheckCollision;

pub struct Point2 {
    x: f32,
    y: f32,
}

pub struct RotatedHitbox {
    p1: glam::Vec2,
    p2: glam::Vec2,
    p3: glam::Vec2,
    p4: glam::Vec2,
}

pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

pub fn get_diagonal_size(w: f32, h: f32, ts: f32) -> f32 {
    ((w.powf(2.) + h.powf(2.)).sqrt()) * ts
}
pub fn rotate_line(origin: glam::Vec2, point: glam::Vec2, angle: f32) -> glam::Vec2 {
    let (ox, oy) = (origin.x, origin.y);
    let (px, py) = (point.x, point.y);

    glam::Vec2::new(
        ox + angle.cos() * (px - ox) - angle.sin() * (py - oy),
        oy + angle.sin() * (px - ox) + angle.cos() * (py - oy),
    )
}

pub fn normalize_point(pt: glam::Vec2) -> glam::Vec2 {
    let d = ((pt.x * pt.x) + (pt.y * pt.y)).sqrt();
    if d != 0.0 {
        glam::Vec2::new(pt.x / d, pt.y / d)
    } else {
        pt
    }
}

pub fn rotate_square(r: ggez::graphics::Rect, angle: f32) -> Vec<glam::Vec2> {
    //-> RotatedHitbox
    let angle = angle;
    // generate points;
    let cx = r.x + r.w / 2.;
    let cy = r.y + r.h / 2.;

    let p1 = glam::Vec2::new(r.x, r.y);
    let p2 = glam::Vec2::new(r.x + r.w, r.y);
    let p3 = glam::Vec2::new(r.x + r.w, r.y + r.h);
    let p4 = glam::Vec2::new(r.x, r.y + r.h);

    let points = vec![p1, p2, p3, p4];

    let mut new_points: Vec<glam::Vec2> = Vec::new();

    for p in points {
        // println!("p:{}", p);
        let temp_x = p.x - cx;
        let temp_y = p.y - cy;
        // println!("cx: {}, cy: {}", cx, cy);

        let rotated = rotate_line(
            glam::Vec2::new(0., 0.),
            glam::Vec2::new(temp_x, temp_y),
            angle,
        );
        let rotated_x = rotated.x + cx;
        let rotated_y = rotated.y + cy;
        // println!("r p:{} {}", rotated_x, rotated_y);

        new_points.push(glam::Vec2::new(rotated_x, rotated_y));
    }
    new_points.push(new_points[0].clone());
    new_points
    // RotatedHitbox {
    //     p1: new_points[0],
    //     p2: new_points[1],
    //     p3: new_points[2],
    //     p4: new_points[3],
    // }
}

impl CheckCollision {
    pub fn two_rect(rect1: ggez::graphics::Rect, rect2: ggez::graphics::Rect) -> bool {
        let is_collision = rect1.x < rect2.x + rect2.w
            && rect1.x + rect1.h > rect2.x
            && rect1.y < rect2.y + rect2.h
            && rect1.h + rect1.y > rect2.y;
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
        entity_hitbox: ggez::graphics::Rect,
        delta_pos: glam::Vec2,
        bloc_list: &Vec<bloc::Bloc>,
    ) -> ggez::graphics::Rect {
        let mut next_pos = ggez::graphics::Rect::new(
            entity_hitbox.x + delta_pos.x,
            entity_hitbox.y + delta_pos.y,
            entity_hitbox.w,
            entity_hitbox.h,
        );
        for bloc in bloc_list {
            let tile = match bloc {
                bloc::Bloc::Air(a) => &a.tile,
                bloc::Bloc::Wall(w) => &w.tile,
            };

            if !tile.transparent {
                let dx = ggez::graphics::Rect::new(
                    next_pos.x,
                    entity_hitbox.y,
                    entity_hitbox.w,
                    entity_hitbox.h,
                );

                let dy = ggez::graphics::Rect::new(
                    entity_hitbox.x,
                    next_pos.y,
                    entity_hitbox.w,
                    entity_hitbox.h,
                );
                if CheckCollision::two_rect(tile.hitbox, dx) {
                    if entity_hitbox.x - next_pos.x > 0. {
                        // Collided left
                        next_pos.x = tile.hitbox.x + tile.hitbox.w;
                    } else {
                        // Collided right
                        next_pos.x = tile.hitbox.x - entity_hitbox.w;
                    }
                }
                if CheckCollision::two_rect(tile.hitbox, dy) {
                    if entity_hitbox.y - next_pos.y > 0. {
                        // Collided up
                        next_pos.y = tile.hitbox.y + tile.hitbox.h;
                    } else {
                        next_pos.y = tile.hitbox.y - entity_hitbox.h;
                    }
                }
            }
        }
        next_pos
        // ggez::graphics::Rect::new(delta_pos.x, delta_pos.y, entity_hitbox.w, entity_hitbox.h)
    }
}

impl RotatedHitbox {
    pub fn new(r: ggez::graphics::Rect, angle: f32) -> Self {
        // rotate_square(r, angle)
        RotatedHitbox {
            p1: glam::Vec2::new(0., 0.),
            p2: glam::Vec2::new(0., 0.),
            p3: glam::Vec2::new(0., 0.),
            p4: glam::Vec2::new(0., 0.),
        }
    }

    pub fn to_vec(&self) -> Vec<glam::Vec2> {
        vec![self.p1, self.p2, self.p3, self.p4]
    }
}
