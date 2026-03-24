use eframe::egui;

struct TodoApp {}

impl TodoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My To-Do List");
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Dictionary Thingy™",
        native_options,
        Box::new(|cc| Ok(Box::new(TodoApp::new(cc)))),
    )
}
