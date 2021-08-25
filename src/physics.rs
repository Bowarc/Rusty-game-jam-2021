use ggez;
use ggez::mint;
use glam;

use crate::{bloc, monster, player};

pub trait EntityTrait {
    fn get_hitbox(&self) -> ggez::graphics::Rect;
    fn get_angle(&self) -> f32;
    fn ray_cast_bypass(&self) -> bool;
    fn rotated_hitbox(&self) -> Vec<glam::Vec2>;
    fn id(&self) -> i32;
    fn take_damage(&mut self, damage: i32);
}

#[derive(Debug, Clone)]
pub enum RayCastBlocType {
    Bloc(usize),
    Bot(usize),
    Player(usize),
    Other,
}
#[derive(Debug, Clone)]
pub enum RayCastResult {
    Ok((glam::Vec2, glam::Vec2), RayCastBlocType, f32),
    // (line),(bloc type, index), distance
    Fail,
}

pub enum CollisionResult {
    In,
    Touch,
    Out,
}

pub struct CheckCollision;
pub struct RayCasting;

pub struct LOS {
    pub angle: f32,
    pub end_point: glam::Vec2,
    pub result: RayCastResult,
}

#[derive(Debug, Clone, Copy)]
pub struct Pos2D<T> {
    pub x: T,
    pub y: T,
}
impl Default for Pos2D<f32> {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl Default for Pos2D<i32> {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl<T> From<mint::Point2<T>> for Pos2D<T> {
    fn from(p: mint::Point2<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}
impl<T> From<Pos2D<T>> for mint::Point2<T> {
    fn from(p: Pos2D<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}
impl From<Pos2D<f32>> for glam::Vec2 {
    fn from(p: Pos2D<f32>) -> Self {
        glam::Vec2::new(p.x, p.y)
    }
}

// pub struct RotatedHitbox {
//     p1: glam::Vec2,
//     p2: glam::Vec2,
//     p3: glam::Vec2,
//     p4: glam::Vec2,
// }

pub struct Circle {
    center: glam::Vec2,
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

impl LOS {
    pub fn default() -> Self {
        LOS {
            angle: 0.,
            end_point: glam::Vec2::new(0., 0.),
            result: RayCastResult::Fail,
        }
    }
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
        let dx = circle1.center.x - circle2.center.x;
        let dy = circle1.center.y - circle2.center.y;
        let distance = (dx * dx + dy * dy).sqrt();

        distance < circle1.radius + circle2.radius
    }

    pub fn get_closest_point(line: (glam::Vec2, glam::Vec2), point: glam::Vec2) -> glam::Vec2 {
        let a_to_b = (line.1.x - line.0.x, line.1.y - line.0.y);

        let perpendicular = (-a_to_b.1, a_to_b.0);

        let q = glam::Vec2::new(point.x + perpendicular.0, point.y + perpendicular.1);

        glam::Vec2::new(
            ((line.0.x * line.1.y - line.0.y * line.1.x) * (point.x - q.x)
                - (line.0.x - line.1.x) * (point.x * q.y - point.y * q.x))
                / ((line.0.x - line.1.x) * (point.y - q.y)
                    - (line.0.y - line.1.y) * (point.y - q.y)),
            ((line.0.x * line.1.y - line.0.y * line.1.x) * (point.y - q.y)
                - (line.0.y - line.1.y) * (point.x * q.y - point.y * q.x))
                / ((line.0.x - line.1.x) * (point.y - q.y)
                    - (line.0.y - line.1.y) * (point.y - q.y)),
        )
    }
    pub fn point_in_circle(point: glam::Vec2, circle: Circle) -> CollisionResult {
        let dist_point_circle_center = RayCasting::get_distance(circle.center, point);

        if dist_point_circle_center > circle.radius {
            // The point is outside the circle
            CollisionResult::Out
        } else if dist_point_circle_center < circle.radius {
            // The point is in the circle
            CollisionResult::In
        } else {
            // The point is on the circle ring
            CollisionResult::Touch
        }
    }
    pub fn line_cross_circle(line: (glam::Vec2, glam::Vec2), circle: Circle) -> CollisionResult {
        let closest_point = CheckCollision::get_closest_point(line, circle.center);

        let collision_result = CheckCollision::point_in_circle(closest_point, circle);
        // let result = collision_result{
        //     CollisionResult::Out => {
        //         // The line doesn't cross the circle
        //     }
        //     CollisionResult::In => {
        //         // The line does cross the circle
        //     }
        //     CollisionResult::Touch => {
        //         // The line touches the circle
        //     }
        // };
        collision_result
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
                bloc::Bloc::Lava(l) => &l.tile,
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

impl RayCasting {
    pub fn get_distance(pt1: glam::Vec2, pt2: glam::Vec2) -> f32 {
        ((pt1.x - pt2.x).powf(2.) + (pt1.y - pt2.y).powf(2.)).sqrt()
    }
    pub fn ccw(a: glam::Vec2, b: glam::Vec2, c: glam::Vec2) -> bool {
        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
    }
    pub fn intersect(a: glam::Vec2, b: glam::Vec2, c: glam::Vec2, d: glam::Vec2) -> bool {
        RayCasting::ccw(a, c, d) != RayCasting::ccw(b, c, d)
            && RayCasting::ccw(a, b, c) != RayCasting::ccw(a, b, d)
    }
    pub fn check_line_interact(l1: (glam::Vec2, glam::Vec2), l2: (glam::Vec2, glam::Vec2)) -> bool {
        let a = glam::Vec2::new(l1.0.x, l1.0.y);
        let b = glam::Vec2::new(l1.1.x, l1.1.y);

        let c = glam::Vec2::new(l2.0.x, l2.0.y);
        let d = glam::Vec2::new(l2.1.x, l2.1.y);

        RayCasting::intersect(a, b, c, d)
    }
    pub fn get_intersection_point(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
    ) -> glam::Vec2 {
        let px = ((((x1 * y2) - (y1 * x2)) * (x3 - x4)) - ((x1 - x2) * ((x3 * y4) - (y3 * x4))))
            / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));

        let py = ((((x1 * y2) - (y1 * x2)) * (y3 - y4)) - ((y1 - y2) * ((x3 * y4) - (y3 * x4))))
            / (((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4)));
        glam::Vec2::new(px, py)
    }
    pub fn check_line_rect_intersection_points(
        // Handle rotated hitboxes
        line: (glam::Vec2, glam::Vec2),
        rect: Vec<glam::Vec2>,
    ) -> Vec<glam::Vec2> {
        let mut result: Vec<glam::Vec2> = Vec::new();
        let (line_1, line_2) = line;

        let rect_lines = [
            (rect[0].x, rect[0].y, rect[1].x, rect[1].y), // topleft, topright
            (rect[1].x, rect[1].y, rect[2].x, rect[2].y), // topright, botright
            (rect[2].x, rect[2].y, rect[3].x, rect[3].y), // botright, botleft
            (rect[3].x, rect[3].y, rect[0].x, rect[0].y), // botleft, topleft
        ]; // not 100% sure but i think lmao

        // let (mut pX, mut pY) = (0.0, 0.0);

        for r in rect_lines.iter() {
            let (rx1, ry1, rx2, ry2) = r.clone();
            let new_r = (glam::Vec2::new(r.0, r.1), glam::Vec2::new(r.2, r.3));
            if RayCasting::check_line_interact(line, new_r) {
                result.push(RayCasting::get_intersection_point(
                    line_1.x, line_1.y, line_2.x, line_2.y, rx1, ry1, rx2, ry2,
                ));
            }
        }
        result
    }

    pub fn ray_cast<E: EntityTrait>(
        line_of_sight: (glam::Vec2, glam::Vec2),
        entity_list: &Vec<E>,
    ) -> ((glam::Vec2, glam::Vec2), Option<usize>, bool) {
        // let mut r_lists_index = None;
        let mut r_item_index = None;
        let mut is_hit = false;

        let los_startpoint = glam::Vec2::new(line_of_sight.0.x, line_of_sight.0.y);
        let mut los_endpoint = glam::Vec2::new(line_of_sight.1.x, line_of_sight.1.y);

        for (index, entity) in entity_list.iter().enumerate() {
            if !entity.ray_cast_bypass() {
                // let lines
                let interaction_points0 = RayCasting::check_line_rect_intersection_points(
                    line_of_sight,
                    entity.rotated_hitbox(),
                );

                let interaction_points = interaction_points0;
                if interaction_points.len() > 0 {
                    is_hit = true;
                    for pt in interaction_points {
                        let dist_1 = RayCasting::get_distance(los_startpoint, los_endpoint);
                        let dist_2 = RayCasting::get_distance(los_startpoint, pt);
                        if dist_1 > dist_2 {
                            los_endpoint = pt.clone();
                            r_item_index = Some(index.clone());
                            let hitted_id = entity.id();
                        }
                    }
                }
            }
        }
        ((los_startpoint, los_endpoint.clone()), r_item_index, is_hit)
    }
    pub fn ray_cast_tile_monster(
        los: (glam::Vec2, glam::Vec2),
        blocs: &Vec<bloc::Bloc>,
        bots: &Vec<monster::Monster>,
    ) -> RayCastResult {
        // let malist: Vec<RayCastBlocType> = vec![RayCastBlocType::Wall, RayCastBlocType::Other];
        let mut min_d = RayCasting::get_distance(los.0, los.1);
        // make a enum result with, as params, the r index, distance, etc..
        // returns
        let mut is_hit: bool = false;

        let mut hit_type = RayCastBlocType::Other;
        let mut new_los = los;

        let (tile_shot, tile_index, tile_is_hit) = RayCasting::ray_cast(los, blocs);
        if tile_is_hit {
            is_hit = true;
            min_d = RayCasting::get_distance(tile_shot.0, tile_shot.1);
            new_los = (
                glam::Vec2::new(los.0.x, los.0.y),
                glam::Vec2::new(tile_shot.1.x, tile_shot.1.y),
            );
            hit_type = RayCastBlocType::Bloc(tile_index.unwrap());
        }
        let (bot_shot, bot_index, bot_is_hit) = RayCasting::ray_cast(tile_shot, bots);
        if bot_is_hit {
            is_hit = true;
            let d = RayCasting::get_distance(bot_shot.0, bot_shot.1);
            if d < min_d {
                // the if bot is closes to the player than the tile
                min_d = d;
                hit_type = RayCastBlocType::Bot(bot_index.unwrap());
                new_los = (
                    glam::Vec2::new(los.0.x, los.0.y),
                    glam::Vec2::new(bot_shot.1.x, bot_shot.1.y),
                );
            }
        };
        if is_hit {
            let result = RayCastResult::Ok(new_los, hit_type, min_d);
            result
        } else {
            let result = RayCastResult::Fail;
            result
        }
    }
    pub fn ray_cast_tile_player(
        los: (glam::Vec2, glam::Vec2),
        blocs: &Vec<bloc::Bloc>,
        players: &Vec<player::Player>,
    ) -> RayCastResult {
        // let malist: Vec<RayCastBlocType> = vec![RayCastBlocType::Wall, RayCastBlocType::Other];
        let mut min_d = RayCasting::get_distance(los.0, los.1);
        // make a enum result with, as params, the r index, distance, etc..
        // returns
        let mut is_hit: bool = false;

        let mut hit_type = RayCastBlocType::Other;
        let mut new_los = los;

        let (tile_shot, tile_index, tile_is_hit) = RayCasting::ray_cast(los, blocs);
        if tile_is_hit {
            is_hit = true;
            min_d = RayCasting::get_distance(tile_shot.0, tile_shot.1);
            new_los = (
                glam::Vec2::new(los.0.x, los.0.y),
                glam::Vec2::new(tile_shot.1.x, tile_shot.1.y),
            );
            hit_type = RayCastBlocType::Bloc(tile_index.unwrap());
        }
        let (player_shot, player_index, player_is_hit) = RayCasting::ray_cast(tile_shot, players);
        if player_is_hit {
            is_hit = true;
            let d = RayCasting::get_distance(player_shot.0, player_shot.1);
            if d < min_d {
                // the if bot is closes to the player than the tile
                min_d = d;
                hit_type = RayCastBlocType::Player(player_index.unwrap());
                new_los = (
                    glam::Vec2::new(los.0.x, los.0.y),
                    glam::Vec2::new(player_shot.1.x, player_shot.1.y),
                );
            }
        };
        if is_hit {
            let result = RayCastResult::Ok(new_los, hit_type, min_d);
            result
        } else {
            let result = RayCastResult::Fail;
            result
        }
    }
}

// impl RotatedHitbox {
//     pub fn new(r: ggez::graphics::Rect, angle: f32) -> Self {
//         // rotate_square(r, angle)
//         RotatedHitbox {
//             p1: glam::Vec2::new(0., 0.),
//             p2: glam::Vec2::new(0., 0.),
//             p3: glam::Vec2::new(0., 0.),
//             p4: glam::Vec2::new(0., 0.),
//         }
//     }

//     pub fn to_vec(&self) -> Vec<glam::Vec2> {
//         vec![self.p1, self.p2, self.p3, self.p4]
//     }
// }
