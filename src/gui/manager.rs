use egui::{Ui, Window};

pub struct GuiManager {}

impl GuiManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, ui: &mut Ui) {
        Window::new("Game Engine").show(ui.ctx(), |ui| {
            if ui.button("Load Asset").clicked() {
                println!("Load Asset Button Clicked");
            }
        });
    }
}
