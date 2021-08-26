use egui::Window;
use ggez_egui::EguiBackend;
use ggez::event::KeyCode;

pub struct Gui {
    pub egui_backend: EguiBackend,
    pub scale: f32,
    pub show_main: bool,
    pub show_settings: bool,
    pub freeze_game: bool,
    pub latest: KeyCode,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            egui_backend: EguiBackend::default(),
            scale: 2.0,
            show_main: true,
            show_settings: false,
            freeze_game: true,
            latest: KeyCode::Key0,
        }
    }

    pub fn main_menu(&mut self, window_size: glam::Vec2, ggez_ctx: &mut ggez::Context) {
        let egui_ctx = self.egui_backend.get_context();
        self.egui_backend
            .input
            .set_scale_factor(self.scale, window_size.into());
        egui::Window::new("Main menu")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
            .show(&egui_ctx, |ui| {
                if ui.button("play").clicked() {
                    self.show_main = false;
                    self.show_settings = false;
                    self.freeze_game = false;
                }
                if ui.button("settings").clicked() {
                    self.show_settings = true;
                    self.show_main = false;
                    self.freeze_game = true
                }
                if ui.button("quit").clicked() {
                    ggez::event::quit(ggez_ctx);
                }
            });
    }

    pub fn settings_menu(&mut self, window_size: glam::Vec2, keymap: &mut crate::input::KeyMap) {
        let egui_ctx = self.egui_backend.get_context();
        Window::new("Settings")
            .open(&mut true)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
            .show(&egui_ctx, |ui| {
                ui.group(|ui| {
                    ui.label("GUI size");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut self.scale, 0.5..=4.0));
                        if ui.button("done").clicked() {
                            let (w, h) = (window_size[0], window_size[1]);
                            self.egui_backend.input.set_scale_factor(self.scale, (w, h));
                        }
                    });
                });
                ui.group(|ui| {
                    ui.label("Input settings");
                    ui.horizontal(|ui| {
                        ui.label("Up");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.up)));
                        if response.changed() {
                            keymap.up = self.latest;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Down");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.down)));
                        if response.changed() {
                            keymap.down = self.latest;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Left");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.left)));
                        if response.changed() {
                            keymap.left = self.latest;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Right");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.right)));
                        if response.changed() {
                            keymap.right = self.latest;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Go to the next map (use when you're on the ladder)");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.next_map)));
                        if response.changed() {
                            keymap.next_map = self.latest;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Escape and pause. Go back to the main menu");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.escape)));
                        if response.changed() {
                            keymap.escape = self.latest;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Inventory");
                        let response = ui.add(egui::TextEdit::singleline(&mut format!("{:?}", keymap.inventory)));
                        if response.changed() {
                            keymap.inventory = self.latest;
                        }
                    });
                });
                if ui.button("Back").clicked() {
                    self.show_settings = false;
                    self.show_main = true;
                    self.freeze_game = true;
                }
            });
    }

    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        ggez::graphics::draw(ctx, &self.egui_backend, (draw_offset,))?;
        Ok(())
    }
}
