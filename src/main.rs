use std::fmt::format;

use eframe::egui;
mod dict;

struct DictApp {
    inputted_text: String,
    definitions: Vec<dict::Definition>,
}

impl DictApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            inputted_text: String::new(),
            definitions: Vec::new(),
        }
    }
}

impl eframe::App for DictApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let text_input = ui.text_edit_singleline(&mut self.inputted_text);

                if (ui.button("Search").clicked()
                    || (text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                    && !self.inputted_text.is_empty()
                {
                    let definitions = dict::get_defenition(&self.inputted_text);
                    self.definitions = definitions;
                    text_input.request_focus();
                }
            });
            ui.separator();
            for (i, def) in self.definitions.iter().enumerate() {
                let title = format!("{}. {}", i + 1, def.word);
                ui.collapsing(title, |ui| {
                    ui.label("test");
                });
            }
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Dictionary Thingy™",
        native_options,
        Box::new(|cc| Ok(Box::new(DictApp::new(cc)))),
    )
}
