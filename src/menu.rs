use ggez_egui::EguiBackend;
use egui::Window;

#[derive(Default)]
pub struct Gui {
    pub egui_backend: EguiBackend,
    pub scale: f32,
    pub show: bool,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            show: true,
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
        egui::Window::new("Rusty game")
            .open(&mut true)
            .show(&egui_ctx, |ui| {
                ui.label("Main menu");
                if ui.button("play").clicked() {
                    self.show = false;
                }
                if ui.button("settings").clicked() {
                    self.settings_menu(window_size)
                }
                if ui.button("quit").clicked() {
                    ggez::event::quit(ggez_ctx);
                }
            });
    }

    pub fn settings_menu(&mut self, window_size: glam::Vec2) {
        let egui_ctx = self.egui_backend.get_context();
        Window::new("Rusty game").show(&egui_ctx, |ui| {
            ui.group(|ui| {
                ui.label("GUI size");
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.scale, 0.5..=1.5));
                    if ui.button("done").clicked() {
                        let (w, h) = (window_size[0], window_size[1]);
                        self.egui_backend.input.set_scale_factor(self.scale, (w, h));
                    }
                });
            });
        });
    }

    pub fn draw(&self, ctx: &mut ggez::Context, draw_offset: glam::Vec2) -> ggez::GameResult {
        ggez::graphics::draw(ctx, &self.egui_backend, (draw_offset,))?;
        Ok(())
    }
}
