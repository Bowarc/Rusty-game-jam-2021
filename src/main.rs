use ggez;
use glam;

mod bloc;
mod camera;
mod input;
mod map;
mod monster;
mod physics;
mod player;
mod menu;

const GAMEPAD_DEAD_ZONE: f32 = 0.5;
const GAMEPAD_SPEED: f32 = 400.;

struct Game {
    map: map::Map,
    player: player::Player,
    monster_list: Vec<monster::Monster>,
    camera: camera::Camera,
    window_size: glam::Vec2,
    menu: menu::Gui,
}

impl Game {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let mut id = 0;
        // set the tile size
        let tile_size = 60.;

        // load the map
        let mut map = map::Map::new(tile_size, &mut id);
        map.load_new_map("game_jam_map_test_1".to_string(), ctx)?;

        // Create the player
        let player_spawn_pos = glam::Vec2::new(tile_size * 5., tile_size * 5.);
        let player = player::Player::new(player_spawn_pos.x, player_spawn_pos.y, 25., 25., 0);

        // Create the camera
        let camera = camera::Camera::new(32., 18.);

        // Create the monsters (empty for now)
        let monster_list: Vec<monster::Monster> = Vec::new();

        let main_menu = menu::Gui::new();

        Ok(Game {
            map: map,
            player: player,
            monster_list: monster_list,
            camera: camera,
            window_size: glam::Vec2::ZERO,
            menu: main_menu,
        })
    }
}
impl ggez::event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Update mouse/gamepad position
        if self.player.inputs.gamepad {
            match self.player.inputs.rightpad.x {
                1 => self.player.inputs.pointing.x += GAMEPAD_SPEED,
                -1 => self.player.inputs.pointing.x -= GAMEPAD_SPEED,
                _ => {}
            }
            match self.player.inputs.rightpad.y {
                1 => self.player.inputs.pointing.y += GAMEPAD_SPEED,
                -1 => self.player.inputs.pointing.y -= GAMEPAD_SPEED,
                _ => {}
            }
        }

        // Update menu
        self.menu.main_menu(self.window_size, ctx);

        // Update player
        self.player.update_movements(&mut self.map.bloc_list, dt);
        self.player.update_los(
            self.camera.scroll,
            &mut self.map.bloc_list,
            &mut self.monster_list,
        );

        self.camera.set_focus(
            (
                self.player.hitbox.x + (self.player.hitbox.w / 2.),
                self.player.hitbox.y + (self.player.hitbox.h / 2.),
            ),
            (self.window_size.x, self.window_size.y),
            (self.map.total_rows, self.map.total_cols),
            self.map.tile_size,
        );
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // ggez::graphics::clear(ctx, ggez::graphics::Color::from_rgba(100, 100, 100, 255));
        ggez::graphics::clear(ctx, ggez::graphics::Color::BLACK);
        let draw_offset = glam::Vec2::new(-self.camera.scroll.x, -self.camera.scroll.y);
        self.map.draw(ctx, draw_offset)?;
        self.player.draw(ctx, draw_offset)?;
        if self.menu.show {
            self.menu.draw(ctx, draw_offset)?;
        }

        ggez::graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        self.menu.egui_backend.input.key_down_event(keycode, keymod);
        match keycode {
            ggez::event::KeyCode::Z => self.player.inputs.up = true,
            ggez::event::KeyCode::S => self.player.inputs.down = true,
            ggez::event::KeyCode::Q => self.player.inputs.left = true,
            ggez::event::KeyCode::D => self.player.inputs.right = true,
            ggez::event::KeyCode::Escape => self.menu.show = true,
            _ => (),
        }
    }
    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
    ) {
        match keycode {
            ggez::event::KeyCode::Z => self.player.inputs.up = false,
            ggez::event::KeyCode::S => self.player.inputs.down = false,
            ggez::event::KeyCode::Q => self.player.inputs.left = false,
            ggez::event::KeyCode::D => self.player.inputs.right = false,

            _ => (),
        }
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.menu.egui_backend.input.mouse_button_down_event(button);
        match button {
            ggez::input::mouse::MouseButton::Left => {}
            ggez::input::mouse::MouseButton::Right => {}
            _ => (),
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.menu.egui_backend.input.mouse_button_up_event(button);
        match button {
            ggez::input::mouse::MouseButton::Left => {
                // self
                //     .player
                //     .inputs
                //     .get_mut(&physics::Input::MouseLeft)
                //     .unwrap() = false;
            }
            ggez::input::mouse::MouseButton::Right => {
                // self
                //     .player
                //     .inputs
                //     .get_mut(&physics::Input::MouseRight)
                //     .unwrap() = false;
            }
            _ => (),
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) {
        self.menu.egui_backend.input.mouse_motion_event(x, y);
        self.player.inputs.pointing = physics::Pos2D {x: x, y: y};
        self.player.inputs.gamepad = false;
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32) {
        self.menu.egui_backend.input.mouse_wheel_event(x, y);
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, character: char) {
        self.menu.egui_backend.input.text_input_event(character);
    }

    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _btn: ggez::event::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) {
        println!("ctx: {:#?}\nbtn: {:#?}\nbid: {:#?}\n", _ctx, _btn, _id);
    }

    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _btn: ggez::event::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) {
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut ggez::Context,
        axis: ggez::event::Axis,
        value: f32,
        _id: ggez::input::gamepad::GamepadId,
    ) {
        self.player.inputs.gamepad = true;
        match axis {
            ggez::event::Axis::LeftStickY => {
                if value >= GAMEPAD_DEAD_ZONE {
                    self.player.inputs.down = false;
                    self.player.inputs.up = true;
                } else if value <= -GAMEPAD_DEAD_ZONE {
                    self.player.inputs.up = false;
                    self.player.inputs.down = true;
                } else {
                    self.player.inputs.up = false;
                    self.player.inputs.down = false;
                }
            }
            ggez::event::Axis::LeftStickX => {
                if value >= GAMEPAD_DEAD_ZONE {
                    self.player.inputs.left = false;
                    self.player.inputs.right = true;
                } else if value <= -GAMEPAD_DEAD_ZONE {
                    self.player.inputs.right = false;
                    self.player.inputs.left = true;
                } else {
                    self.player.inputs.right = false;
                    self.player.inputs.left = false;
                }
            }
            ggez::event::Axis::RightStickX => {
                if value >= GAMEPAD_DEAD_ZONE {
                    self.player.inputs.rightpad.x = 1;
                } else if value <= -GAMEPAD_DEAD_ZONE {
                    self.player.inputs.rightpad.x = -1;
                } else {
                    self.player.inputs.rightpad.x = 0;
                }
            }
            ggez::event::Axis::RightStickY => {
                if value >= GAMEPAD_DEAD_ZONE {
                    self.player.inputs.rightpad.y = -1;
                } else if value <= -GAMEPAD_DEAD_ZONE {
                    self.player.inputs.rightpad.y = 1;
                } else {
                    self.player.inputs.rightpad.y = 0;
                }
            }
            _ => {}
        }
    }

    fn focus_event(&mut self, _ctx: &mut ggez::Context, _gained: bool) {}

    fn quit_event(&mut self, _ctx: &mut ggez::Context) -> bool {
        false
    }

    fn resize_event(&mut self, _ctx: &mut ggez::Context, width: f32, height: f32) {
        self.menu.egui_backend.input.resize_event(width, height);
        self.window_size = glam::Vec2::new(width, height);
        println!("Resized to {}x{}", width, height);
    }
}

fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("test game 1", "ggez")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("The game title")
                .vsync(false),
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1920., 1080.)
                .fullscreen_type(ggez::conf::FullscreenType::Desktop)
                .resizable(true),
        );
    // maybe resource dir

    let (mut ctx, events_loop) = cb.build()?;

    let game = Game::new(&mut ctx)?;
    ggez::event::run(ctx, events_loop, game)
}
