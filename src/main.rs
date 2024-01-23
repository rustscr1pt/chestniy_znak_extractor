#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use eframe::{App, Frame, NativeOptions};
use eframe::egui::{Context, Vec2};
use eframe::egui::panel::Side;
use eframe::egui::panel::TopBottomSide::Bottom;
use eframe::Theme::Dark;
use tokio::runtime::Runtime;
use crate::DataModels::{MainBody, return_central_frame, Switch};

pub const SCREEN_WIDTH : f32 = 600f32;
pub const SCREEN_HEIGHT : f32 = 300f32;

mod DataModels;
mod converter;
mod side_panel;
mod filter;
mod bottom_panel;
mod custom;

impl App for MainBody {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Ok(log) = self.logs.logs_receiver.try_recv() {
            if log == "$CLEARLOGSCOMMAND$$" {
                self.clear_bottom()
            }
            else {
                self.logs.display_logs.insert(0, log)
            }
        }

        eframe::egui::SidePanel::new(Side::Left, "Left").resizable(false).exact_width(110f32).show(ctx, |ui| {
            self.render_side_panel(ui, frame)
        });

        eframe::egui::TopBottomPanel::new(Bottom, "bottom").exact_height(120f32).resizable(false).show(ctx, |ui| {
            self.render_logs_bottom(ui)
        });

        eframe::egui::CentralPanel::default().frame(return_central_frame()).show(ctx, |ui| {
            match self.position {
                Switch::Converter => {self.render_converter(ui)}
                Switch::Filter => {self.render_filter(ui)}
            }
        });
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let _enter = runtime.enter();
    std::thread::spawn(move || {
        runtime.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    let mut window = NativeOptions::default();
    window.initial_window_size = Option::from(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    window.fullscreen = false;
    window.default_theme = Dark;
    window.follow_system_theme = false;
    window.resizable = false;
    window.max_window_size = Option::from(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    window.maximized = false;

    eframe::run_native("Extractor", window, Box::new(|_cc| Box::new(MainBody::new()))).unwrap()
}
