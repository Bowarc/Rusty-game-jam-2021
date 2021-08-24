use ggez;
use glam;
// #[derive(Clone, Copy)]

pub struct Camera {
    pub camera: ggez::graphics::Rect,
    pub scroll: glam::Vec2,
}

fn min(min: f32, nbr: f32) -> f32 {
    if min < nbr {
        min
    } else {
        nbr
    }
}

fn max(max: f32, nbr: f32) -> f32 {
    if max > nbr {
        max
    } else {
        nbr
    }
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Camera {
            camera: ggez::graphics::Rect::new(0.0, 0.0, width, height),
            scroll: glam::Vec2::new(0., 0.),
        }
    }
    pub fn set_focus(
        &mut self,
        target: (f32, f32),
        window_size: (f32, f32),
        map_size: (f32, f32),
        tile_size: f32,
    ) {
        // println!("Camera focusing on x:{} y:{}.", target.0, target.1);
        let mut x = -target.0 + (window_size.0 / 2.0);
        let mut y = -target.1 + (window_size.1 / 2.0);

        x = min(0.0, x);
        y = min(0.0, y);

        x = max((map_size.1 - self.camera.w) * -tile_size, x);
        y = max((map_size.0 - self.camera.h) * -tile_size, y);

        self.scroll = glam::Vec2::new(-x, -y);
        self.camera.x = x;
        self.camera.y = y;
    }
}
