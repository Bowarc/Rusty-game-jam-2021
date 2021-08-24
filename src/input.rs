use crate::physics::Pos2D;

#[derive(Default)]
pub struct Input {
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub pointing: Pos2D<f32>,
    pub rightpad: Pos2D<i32>,
    pub gamepad: bool,
}
