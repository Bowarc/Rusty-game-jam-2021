use ggez_egui::EguiBackend;
use egui::Window;

#[derive(Default)]
pub struct Gui {
    pub egui_backend: EguiBackend,
    pub scale: f32,
    pub show_main: bool,
    pub show_settings: bool,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            show_main: true,
            scale: 2.0,
            ..Default::default()
        }
    }

    pub fn main_menu(
        &mut self,
        window_size: glam::Vec2,
        ggez_ctx: &mut ggez::Context
    ) {
        let egui_ctx = self.egui_backend.get_context();
        self.egui_backend.input.set_scale_factor(self.scale, window_size.into());
        egui::Window::new("Main menu")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0., 0.))
            .show(&egui_ctx, |ui| {
                if ui.button("play").clicked() {
                    self.show_main = false;
                    self.show_settings = false;
                }
                if ui.button("settings").clicked() {
                    self.show_settings = true;
                    self.show_main = false;
                }
                if ui.button("quit").clicked() {
                    ggez::event::quit(ggez_ctx);
                }
            });
    }

    pub fn settings_menu(&mut self, window_size: glam::Vec2) {
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
                if ui.button("Back").clicked() {
                    self.show_settings = false;
                    self.show_main = true;
                }
            });
    }

    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        ggez::graphics::draw(ctx, &self.egui_backend, (draw_offset,))?;
        Ok(())
    }
}
