#![deny(unsafe_code)]
use ggez;
use glam;

mod bloc;
mod camera;
mod id;
mod input;
mod map;
mod menu;
mod monster;
mod physics;
mod player;
mod weapon;

const GAMEPAD_DEAD_ZONE: f32 = 0.5;
const GAMEPAD_SPEED: f32 = 400.;
const DEFAULT_WINDOW_SIZE: (f32, f32) = (1920., 1080.);

struct Game {
    map: map::Map,
    player: player::Player,
    monster_manager: monster::MonsterManager,
    camera: camera::Camera,
    window_size: glam::Vec2,
    menu: menu::Gui,
    id_manager: id::IdManager,
    keymap: input::KeyMap,
}

impl Game {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let mut id_manager = id::IdManager::new();
        // set the tile size
        let tile_size = 60.;

        // load the map
        let mut map = map::Map::new(tile_size, ctx);
        map.gen_new_map(ctx, &mut id_manager)?;

        // Create the player
        let player_spawn_pos = glam::Vec2::new(
            tile_size * map.spawn.x + tile_size / 2. - 30. / 2.,
            tile_size * map.spawn.y + tile_size / 2. - 30. / 2.,
        );
        let player = player::Player::new(
            player_spawn_pos.x,
            player_spawn_pos.y,
            30.,
            30.,
            &mut id_manager,
            ctx,
        );

        // Create the camera
        let mut camera = camera::Camera::new(32., 18.);

        let focus = player.hitbox.center();
        camera.set_focus(
            (focus.x, focus.y),
            (DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1),
            (map.total_rows, map.total_cols),
            tile_size,
        );

        // Create the monsters
        let mut monster_manager = monster::MonsterManager::new();

        for _ in 0..1 {
            monster_manager.new_bot(
                monster::MonsterType::TestBot,
                &mut id_manager,
                (
                    map.map_file_content.clone(),
                    map.ghost_tiles.clone(),
                    map.tile_size,
                ),
            );
        }

        // Create main menu
        let main_menu = menu::Gui::new();

        Ok(Game {
            map: map,
            player: player,
            monster_manager: monster_manager,
            camera: camera,
            window_size: glam::Vec2::ZERO,
            menu: main_menu,
            id_manager: id_manager,
            keymap: input::KeyMap::default(),
        })
    }
}
impl ggez::event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Update menu
        if self.menu.show_main {
            self.menu.main_menu(self.window_size, ctx);
        }
        if self.menu.show_settings {
            self.menu.settings_menu(self.window_size, &mut self.keymap);
        }
        if !self.menu.freeze_game {
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

            // Update player
            self.player.update_movements(
                &mut self.map.bloc_list,
                dt,
                &mut self.id_manager,
                &mut self.monster_manager,
                ctx,
            );
            self.player.update_los(
                self.camera.scroll,
                &mut self.map.bloc_list,
                &mut self.monster_manager.monster_list,
            );
            // let player_vec = Vec::new().push(self.player);
            self.map.bloc_effects(&mut self.player);

            // Update the monsters
            while self.monster_manager.monster_list.len() < 30 {
                self.monster_manager.new_bot(
                    monster::MonsterType::TestBot,
                    &mut self.id_manager,
                    (
                        self.map.map_file_content.clone(),
                        self.map.ghost_tiles.clone(),
                        self.map.tile_size,
                    ),
                );
            }
            self.monster_manager
                .update(glam::Vec2::from(self.player.hitbox.center()));
            self.monster_manager.update_movements(
                dt,
                &self.map.bloc_list,
                (
                    self.map.map_file_content.clone(),
                    self.map.ghost_tiles.clone(),
                    self.map.tile_size,
                ),
                (self.map.total_rows, self.map.total_cols),
            );
            for index in 0..self.monster_manager.monster_list.len() {
                self.map
                    .bloc_effects(&mut self.monster_manager.monster_list[index])
            }

            // Update the camera
            // let focus =
            // physics::EntityTrait::get_hitbox(&self.monster_manager.monster_list[0]).center();
            let focus = self.player.hitbox.center();
            self.camera.set_focus(
                (focus.x, focus.y),
                (self.window_size.x, self.window_size.y),
                (self.map.total_rows, self.map.total_cols),
                self.map.tile_size,
            );
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, ggez::graphics::Color::BLACK);
        let draw_offset = glam::Vec2::new(-self.camera.scroll.x, -self.camera.scroll.y);
        self.map.draw(ctx, draw_offset)?;
        self.monster_manager.draw_monsters(ctx, draw_offset)?;
        self.player.draw(ctx, draw_offset)?;
        if self.menu.show_main || self.menu.show_settings {
            self.menu.draw(ctx, draw_offset)?;
        }

        // Draw the GUI
        let font = ggez::graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;

        let level_dest = glam::Vec2::new(10.0, 10.0);
        let hp_dest = glam::Vec2::new(200.0, 10.0);
        let fps_dest = glam::Vec2::new(10.0, 50.0);

        let level_str = format!("Level: {}", self.map.difficulty);
        let hp_str = format!("HP: {}", self.player.hp);
        let fps_str = format!("Fps: {:.2}", ggez::timer::fps(ctx));

        let player_hp_color = ggez::graphics::Color::from_rgb(
            std::cmp::min(
                255,
                (255 - (255 * ((self.player.hp - (100 - self.player.hp)) / 100))) as u8,
            ),
            std::cmp::min(255, (255 * (self.player.hp / (100 / 2))) as u8),
            0,
        );

        let hp_text_fragment = ggez::graphics::TextFragment::new(hp_str).color(player_hp_color);
        let level_display = ggez::graphics::Text::new((level_str, font, 32.0));
        let hp_display = ggez::graphics::Text::new((hp_text_fragment, font, 32.0));
        let fps_display = ggez::graphics::Text::new((fps_str, font, 32.0));
        ggez::graphics::draw(
            ctx,
            &level_display,
            (level_dest, 0.0, ggez::graphics::Color::WHITE),
        )?;
        ggez::graphics::draw(
            ctx,
            &hp_display,
            (hp_dest, 0.0, ggez::graphics::Color::WHITE),
        )?;
        ggez::graphics::draw(
            ctx,
            &fps_display,
            (fps_dest, 0.0, ggez::graphics::Color::WHITE),
        )?;

        ggez::graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) -> () {
        self.menu.egui_backend.input.key_down_event(keycode, keymod);
        if self.menu.show_settings {
            self.menu.latest = keycode;
        }

        if keycode == self.keymap.up {
            self.player.inputs.up = true;
        } else if keycode == self.keymap.down {
            self.player.inputs.down = true;
        } else if keycode == self.keymap.left {
            self.player.inputs.left = true;
        } else if keycode == self.keymap.right {
            self.player.inputs.right = true;
        } else if keycode == self.keymap.next_map {
            let distance_from_end = physics::RayCasting::get_distance(
                glam::Vec2::from(self.player.hitbox.center()),
                glam::Vec2::new(
                    self.map.end.x * self.map.tile_size + (self.map.tile_size / 2.),
                    self.map.end.y * self.map.tile_size + (self.map.tile_size / 2.),
                ),
            );
            if distance_from_end < self.map.tile_size {
                self.map.difficulty += 1;
                self.map.gen_new_map(ctx, &mut self.id_manager).unwrap();
                self.player.hitbox.x = self.map.spawn.x * self.map.tile_size;
                self.player.hitbox.y = self.map.spawn.y * self.map.tile_size;
            }
        } else if keycode == self.keymap.escape {
            if !self.menu.show_main && !self.menu.show_settings {
                self.menu.show_main = true;
                self.menu.freeze_game = true
            } else if self.menu.show_settings {
                self.menu.show_settings = false;
                self.menu.show_main = true;
                self.menu.freeze_game = true
            } else if self.menu.show_main {
                self.menu.show_main = true;
                self.menu.freeze_game = true
            }
        }
    }
    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
    ) {
        if keycode == self.keymap.up {
            self.player.inputs.up = false;
        } else if keycode == self.keymap.down {
            self.player.inputs.down = false;
        } else if keycode == self.keymap.left {
            self.player.inputs.left = false;
        } else if keycode == self.keymap.right {
            self.player.inputs.right = false;
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
            ggez::input::mouse::MouseButton::Left => self.player.inputs.mouse_left = true,
            ggez::input::mouse::MouseButton::Right => self.player.inputs.mouse_right = true,
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
            ggez::input::mouse::MouseButton::Left => self.player.inputs.mouse_left = false,
            ggez::input::mouse::MouseButton::Right => self.player.inputs.mouse_right = false,
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut ggez::Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.menu.egui_backend.input.mouse_motion_event(x, y);
        self.player.inputs.pointing = physics::Pos2D { x: x, y: y };
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
        btn: ggez::event::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) {
        match btn {
            ggez::event::Button::South => self.player.inputs.controler_south = true,
            ggez::event::Button::North => self.player.inputs.controler_north = true,
            ggez::event::Button::East => self.player.inputs.controler_east = true,
            ggez::event::Button::West => self.player.inputs.controler_west = true,
            ggez::event::Button::Select => self.player.inputs.controler_select = true,
            ggez::event::Button::Start => self.player.inputs.controler_start = true,
            ggez::event::Button::Mode => self.player.inputs.controler_mode = true,
            ggez::event::Button::RightTrigger => {
                self.player.inputs.controler_right_trigger_1 = true
            }
            ggez::event::Button::LeftTrigger => self.player.inputs.controler_left_trigger_1 = true,
            ggez::event::Button::RightTrigger2 => {
                self.player.inputs.controler_right_trigger_2 = true
            }
            ggez::event::Button::LeftTrigger2 => self.player.inputs.controler_left_trigger_2 = true,
            ggez::event::Button::DPadRight => self.player.inputs.controler_dpad_right = true,
            ggez::event::Button::DPadUp => self.player.inputs.controler_dpad_up = true,
            ggez::event::Button::DPadLeft => self.player.inputs.controler_dpad_left = true,
            ggez::event::Button::DPadDown => self.player.inputs.controler_dpad_down = true,
            ggez::event::Button::LeftThumb => self.player.inputs.controler_left_thumb = true,
            ggez::event::Button::RightThumb => self.player.inputs.controler_right_thumb = true,
            _ => (),
        }
    }

    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        btn: ggez::event::Button,
        _id: ggez::input::gamepad::GamepadId,
    ) {
        match btn {
            ggez::event::Button::South => self.player.inputs.controler_south = false,
            ggez::event::Button::North => self.player.inputs.controler_north = false,
            ggez::event::Button::East => self.player.inputs.controler_east = false,
            ggez::event::Button::West => self.player.inputs.controler_west = false,
            ggez::event::Button::Select => self.player.inputs.controler_select = false,
            ggez::event::Button::Start => self.player.inputs.controler_start = false,
            ggez::event::Button::Mode => self.player.inputs.controler_mode = false,
            ggez::event::Button::RightTrigger => {
                self.player.inputs.controler_right_trigger_1 = false
            }
            ggez::event::Button::LeftTrigger => self.player.inputs.controler_left_trigger_1 = false,
            ggez::event::Button::RightTrigger2 => {
                self.player.inputs.controler_right_trigger_2 = false
            }
            ggez::event::Button::LeftTrigger2 => {
                self.player.inputs.controler_left_trigger_2 = false
            }
            ggez::event::Button::DPadRight => self.player.inputs.controler_dpad_right = false,
            ggez::event::Button::DPadUp => self.player.inputs.controler_dpad_up = false,
            ggez::event::Button::DPadLeft => self.player.inputs.controler_dpad_left = false,
            ggez::event::Button::DPadDown => self.player.inputs.controler_dpad_down = false,
            ggez::event::Button::LeftThumb => self.player.inputs.controler_left_thumb = false,
            ggez::event::Button::RightThumb => self.player.inputs.controler_right_thumb = false,
            _ => (),
        }
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
                .dimensions(DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1)
                .fullscreen_type(ggez::conf::FullscreenType::Desktop)
                .resizable(true),
        );
    // maybe resource dir

    let (mut ctx, events_loop) = cb.build()?;

    let game = Game::new(&mut ctx)?;
    ggez::event::run(ctx, events_loop, game)
}
