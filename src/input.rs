use crate::physics::Pos2D;

use ggez::event::KeyCode;

#[derive(Default)]
pub struct Input {
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub pointing: Pos2D<f32>,
    pub rightpad: Pos2D<i32>,
    pub gamepad: bool,
    pub mouse_left: bool,
    pub mouse_right: bool,
    pub controler_south: bool,
    pub controler_east: bool,
    pub controler_west: bool,
    pub controler_north: bool,
    pub controler_right_trigger_1: bool,
    pub controler_left_trigger_1: bool,
    pub controler_right_trigger_2: bool,
    pub controler_left_trigger_2: bool,
    pub controler_start: bool,
    pub controler_select: bool,
    pub controler_mode: bool,
    pub controler_dpad_right: bool,
    pub controler_dpad_up: bool,
    pub controler_dpad_left: bool,
    pub controler_dpad_down: bool,
    pub controler_left_thumb: bool,
    pub controler_right_thumb: bool,
}

pub struct KeyMap {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub next_map: KeyCode,
    pub escape: KeyCode,
    pub inventory: KeyCode,
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            next_map: KeyCode::R,
            escape: KeyCode::Escape,
            inventory: KeyCode::E,
        }
    }
}
/*
fn egui_to_winit_key_code(key: Key) -> Option<KeyCode> {
    Some(match key {
        Key::Escape => KeyCode::Escape,
        Key::Insert => KeyCode::Insert,
        Key::Home => KeyCode::Home,
        Key::Delete => KeyCode::Delete,
        Key::End => KeyCode::End,
        Key::PageDown => KeyCode::PageDown,
        Key::PageUp => KeyCode::PageUp,
        Key::ArrowLeft => KeyCode::Left,
        Key::ArrowUp => KeyCode::Up,
        Key::ArrowRight => KeyCode::Right,
        Key::ArrowDown => KeyCode::Down,
        Key::Backspace => KeyCode::Back,
        Key::Enter => KeyCode::Return,
        Key::Tab => KeyCode::Tab,
        Key::Space => KeyCode::Space,

        Key::A => KeyCode::A,
        Key::B => KeyCode::B,
        Key::C => KeyCode::C,
        Key::D => KeyCode::D,
        Key::E => KeyCode::E,
        Key::F => KeyCode::F,
        Key::G => KeyCode::G,
        Key::H => KeyCode::H,
        Key::I => KeyCode::I,
        Key::J => KeyCode::J,
        Key::K => KeyCode::K,
        Key::L => KeyCode::L,
        Key::M => KeyCode::M,
        Key::N => KeyCode::N,
        Key::O => KeyCode::O,
        Key::P => KeyCode::P,
        Key::Q => KeyCode::Q,
        Key::R => KeyCode::R,
        Key::S => KeyCode::S,
        Key::T => KeyCode::T,
        Key::U => KeyCode::U,
        Key::V => KeyCode::V,
        Key::W => KeyCode::W,
        Key::X => KeyCode::X,
        Key::Z => KeyCode::Z,

        _ => {
            return None;
        }
    })
}
*/
