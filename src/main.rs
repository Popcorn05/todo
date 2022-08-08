extern crate eframe;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default(); // Set window options
    eframe::run_native(
    "Todo", // Name
    options, // Options
    Box::new(|cc| Box::new(TodoApp::new(cc)))); // App hand over
}

// App definition
#[derive(Default)]
struct TodoApp {}

impl TodoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default() // Return
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}