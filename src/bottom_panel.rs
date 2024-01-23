use eframe::egui::{Color32, Label, RichText, ScrollArea, Ui};
use crate::DataModels::MainBody;
use crate::SCREEN_WIDTH;

impl MainBody {
    pub fn render_logs_bottom(&self, ui : &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(5f32);
            ui.add(Label::new(RichText::new("Логи").color(Color32::WHITE).monospace().heading().underline()));
            ScrollArea::vertical().auto_shrink([false, false]).max_width(SCREEN_WIDTH).show(ui, |ui| {
                match self.logs.display_logs.len() {
                    0 => {
                        ui.add(logs_formatter("Пока ничего нет."));
                    },
                    _ => {
                        for elements in &self.logs.display_logs {
                            ui.add(logs_formatter(elements.as_str()));
                        };
                    }
                }
            })
        });
    }
}

pub fn logs_formatter(text : &str) -> Label {
    return Label::new(RichText::new(format!("-> {}", text)).monospace().color(Color32::WHITE).size(12f32))
}