use eframe::egui::{Button, Color32, Label, RichText, TextEdit, Ui};
use crate::custom::{toggle_excel_};
use crate::DataModels::{filter_and_write, MainBody};

impl MainBody {
    pub fn render_filter(&mut self, ui : &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new("Строка с кодами (для выреза)").color(Color32::WHITE).heading()))
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(TextEdit::singleline(&mut self.filter.filter))
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new("Строка с кодами (из которой вырезать)").color(Color32::WHITE).heading()))
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(TextEdit::singleline(&mut self.filter.main_body))
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new("Укажите путь к файлу для записи.").color(Color32::WHITE).heading()))
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(TextEdit::singleline(&mut self.way_to_file));
                ui.add_space(40f32);
                if ui.add(Button::new(RichText::new("Записать в файл."))).clicked() {
                    filter_and_write(self.filter.filter.clone(), self.filter.main_body.clone(), self.way_to_file.clone(), self.logs.logs_sender.clone(), self.excel_writer.clone())
                };
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(toggle_excel_(&mut self.excel_writer, self.logs.logs_sender.clone()));
                ui.add_space(5f32);
                ui.add(Label::new(RichText::new("EXCEL").color(Color32::WHITE).monospace().size(14f32)));
            });
        });
    }
}