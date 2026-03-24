use eframe::egui;

struct DictApp {
    inputted_text: String,
}

impl DictApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            inputted_text: String::new(),
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
                    text_input.request_focus();
                }
            })
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
