pub struct CheckCollision;

pub struct Point2 {
    x: f32,
    y: f32,
}

pub fn get_diagonal_size(w: f32, h: f32, ts: f32) -> f32 {
    ((w.powf(2.) + h.powf(2.)).sqrt()) * ts
}

impl CheckCollision {}
