use eframe::egui::{Button, Color32, RichText, Stroke, Ui, Vec2};
use eframe::Frame;
use crate::DataModels::{clear_logs, MainBody, Switch, write_log};

impl MainBody {
    pub fn render_side_panel(&mut self, ui : &mut Ui, frame: &mut Frame) -> () {
        ui.vertical(|ui| {
            ui.add_space(5f32);
            if ui.add_sized(Vec2::new(110f32, 10f32), Button::new(RichText::new("Конвертировать").small().monospace()).rounding(
                if self.position == Switch::Converter {10f32} else {5f32}
            ).stroke(
                if self.position == Switch::Converter {Stroke::new(1f32, Color32::GREEN)} else {Stroke::new(0f32, Color32::GREEN)}
            )).clicked() {
                self.position = Switch::Converter
            }
            ui.add_space(3f32);
            if ui.add_sized(Vec2::new(110f32, 10f32),Button::new(RichText::new("Фильтровать").small().monospace()).rounding(
                if self.position == Switch::Filter {10f32} else {5f32}
            ).stroke(
                if self.position == Switch::Filter {Stroke::new(1f32, Color32::GREEN)} else {Stroke::new(0f32, Color32::GREEN)}
            )).clicked() {
                self.position = Switch::Filter
            }
            ui.add_space(3f32);
            if ui.add_sized(Vec2::new(110f32, 10f32),Button::new(RichText::new("Очистить логи").small().monospace())).clicked() {
                clear_logs(self.logs.logs_sender.clone())
            }
            ui.add_space(3f32);
            if ui.add_sized(Vec2::new(110f32, 10f32),Button::new(RichText::new("Сбросить поля").small().monospace())).clicked() {
                self.clear_fields()
            };
            ui.add_space(175f32);
            ui.scope(|ui| {
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_RED;
                if ui.add_sized(Vec2::new(110f32, 10f32),Button::new(RichText::new("Выход.").small().monospace())).clicked() {
                    frame.close()
                };
            });
        });
    }



    pub fn clear_bottom(&mut self) -> () {self.logs.display_logs.clear()}

    pub fn clear_fields(&mut self) -> () {
        self.filter.main_body.clear();
        self.filter.filter.clear();
        self.converter.separator.clear();
        self.converter.input.clear();
        write_log("Все поля для ввода очищены.", self.logs.logs_sender.clone())
    }
}