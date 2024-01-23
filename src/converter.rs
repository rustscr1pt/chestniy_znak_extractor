use eframe::egui::{Button, Color32, Label, RichText, TextEdit, Ui};
use crate::custom::{toggle, toggle_excel_};
use crate::DataModels::{convert_and_write, MainBody};

impl MainBody {
    pub fn render_converter(&mut self, ui : &mut Ui) -> () {
        ui.vertical(|ui| {
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new("Внесите строку с кодами.").color(Color32::WHITE).heading()))
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(TextEdit::singleline(&mut self.converter.input))
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new("Введите разделяющие символы.").color(Color32::WHITE).heading()))
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(TextEdit::singleline(&mut self.converter.separator))
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(Label::new(RichText::new("Укажите путь к файлу для записи.").color(Color32::WHITE).heading()));
            });
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(TextEdit::singleline(&mut self.way_to_file));
                ui.add_space(40f32);
                if ui.add(Button::new(RichText::new("Записать в файл."))).clicked() {
                    convert_and_write(self.converter.input.clone(), self.way_to_file.clone(), self.converter.separator.clone(), self.logs.logs_sender.clone(), self.excel_writer.clone())
                };
            });
            ui.add_space(5f32);
            ui.horizontal(|ui| {
                ui.add_space(10f32);
                ui.add(toggle(&mut self.converter.smart_mode, self.logs.logs_sender.clone()));
                ui.add_space(5f32);
                ui.add(Label::new(RichText::new("Обрезать лишнее.").color(Color32::WHITE).monospace().size(14f32)));
                ui.add_space(10f32);
                ui.add(toggle_excel_(&mut self.excel_writer, self.logs.logs_sender.clone()));
                ui.add_space(5f32);
                ui.add(Label::new(RichText::new("EXCEL").color(Color32::WHITE).monospace().size(14f32)));
            });
        });
    }
}