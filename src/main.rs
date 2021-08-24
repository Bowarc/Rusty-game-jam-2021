use ggez;
use glam;

mod bloc;
mod camera;
mod input;
mod map;
mod monster;
mod physics;
mod player;

const GAMEPAD_DEAD_ZONE: f32 = 0.5;

struct Game {
    map: map::Map,
    player: player::Player,
    monster_list: Vec<monster::Monster>,
    camera: camera::Camera,
    window_size: glam::Vec2,
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
        let player = player::Player::new(player_spawn_pos.x, player_spawn_pos.y, 25., 25., &mut 0);

        // Create the camera
        let camera = camera::Camera::new(32., 18.);

        // Create the monsters (empty for now)
        let monster_list: Vec<monster::Monster> = Vec::new();

        Ok(Game {
            map: map,
            player: player,
            monster_list: monster_list,
            camera: camera,
            window_size: glam::Vec2::ZERO,
        })
    }
}
impl ggez::event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let mouse_pos = ggez::input::mouse::position(ctx);
        self.player.update_movements(&mut self.map.bloc_list, dt);
        self.player.update_los(
            mouse_pos,
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

        ggez::graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            ggez::event::KeyCode::Z => self.player.inputs.up = true,
            ggez::event::KeyCode::S => self.player.inputs.down = true,
            ggez::event::KeyCode::Q => self.player.inputs.left = true,
            ggez::event::KeyCode::D => self.player.inputs.right = true,
            ggez::event::KeyCode::Escape => ggez::event::quit(ctx),
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
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) {
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, _y: f32) {}

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, _character: char) {}

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
        if axis == ggez::event::Axis::LeftStickY {
            if value >= GAMEPAD_DEAD_ZONE {
                self.player.inputs.down = false;
                self.player.inputs.up = true;
            }
            else if value <= -GAMEPAD_DEAD_ZONE {
                self.player.inputs.up = false;
                self.player.inputs.down = true;
            }
            else {
                self.player.inputs.up = false;
                self.player.inputs.down = false;
            }
        }
        if axis == ggez::event::Axis::LeftStickX {
            if value >= GAMEPAD_DEAD_ZONE {
                self.player.inputs.left = false;
                self.player.inputs.right = true;
            }
            else if value <= -GAMEPAD_DEAD_ZONE {
                self.player.inputs.right = false;
                self.player.inputs.left = true;
            }
            else {
                self.player.inputs.right = false;
                self.player.inputs.left = false;
            }
        }
        println!("ctx: {:#?}\naxis: {:#?}\nvalue: {:#?}\nid: {:#?}\n", _ctx, axis, value, _id);
    }

    fn focus_event(&mut self, ctx: &mut ggez::Context, gained: bool) {
        if gained {
            println!("Gained focus");
            println!("-------------------------------");
        } else {
            println!("-------------------------------");
            println!("Lost focus");
            println!("FPS: {}", ggez::timer::fps(ctx));
            // println!("Frame time: {:?}", ggez::timer::delta(ctx));
        }
    }

    fn quit_event(&mut self, _ctx: &mut ggez::Context) -> bool {
        false
    }

    fn resize_event(&mut self, _ctx: &mut ggez::Context, width: f32, height: f32) {
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
