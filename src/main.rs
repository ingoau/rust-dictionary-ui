use std::fmt::format;

use eframe::egui;
mod dict;

struct DictApp {
    inputted_text: String,
    definitions: Vec<dict::Definition>,
}

impl DictApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "Outfit".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                egui::FontData::from_static(include_bytes!("../assets/Outfit-Regular.ttf")),
            ),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "Outfit".to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("Outfit".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        egui_material_icons::initialize(&cc.egui_ctx);

        Self {
            inputted_text: String::new(),
            definitions: Vec::new(),
        }
    }

    fn search_ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        let text_input = ui.text_edit_singleline(&mut self.inputted_text);
        if (ui
            .button("Search ".to_owned() + egui_material_icons::icons::ICON_SEARCH)
            .clicked()
            || (text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
            && !self.inputted_text.is_empty()
        {
            let definitions = dict::get_defenition(&self.inputted_text);
            self.definitions = definitions;
            text_input.request_focus();
        }
    }
}

impl eframe::App for DictApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if (self.definitions.is_empty()) {
                ui.vertical_centered(|ui| {
                    egui::Frame::default().inner_margin(16).show(ui, |ui| {
                        ui.heading("Dictionary Thingy™");
                        ui.label("Enter a word to search for its definition.");
                        self.search_ui(ctx, _frame, ui);
                    })
                });
            } else {
                ui.horizontal(|ui| {
                    self.search_ui(ctx, _frame, ui);
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        for (i, def) in self.definitions.iter().enumerate() {
                            let title = format!("{}. {}", i + 1, def.word);
                            ui.push_id(title.clone(), |ui| {
                                egui::Frame::default().inner_margin(16).show(ui, |ui| {
                                    ui.heading(title);
                                    ui.horizontal(|ui| {
                                        for phonetic in &def.phonetics {
                                            ui.label(phonetic.text.as_deref().unwrap_or(""));
                                        }
                                    });
                                    for (_, meaning) in def.meanings.iter().enumerate() {
                                        ui.collapsing(meaning.part_of_speech.clone(), |ui| {
                                            for (i, definition) in
                                                meaning.definitions.iter().enumerate()
                                            {
                                                ui.push_id(&i, |ui| {
                                                    ui.label(definition.definition.clone());
                                                    if let Some(example) = &definition.example {
                                                        ui.label(example.clone());
                                                    }
                                                });
                                            }
                                        });
                                    }
                                });
                            });
                        }
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
