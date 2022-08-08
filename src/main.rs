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
struct TodoApp {
    buttons: Vec<String>,
}

impl TodoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut me = Self::default(); // Return
        me.add_button(); // Push first button
        me
    }

    fn add_button(&mut self) {
        self.buttons.push("Button".to_string());
    }
}

// Update impl
impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(egui::Visuals::dark()); // Set visuals to dark mode

            ui.heading("Hello World!"); // Hello world text

            for i in 0..self.buttons.len() {
                if ui.button(self.buttons[i].as_str()).clicked() {
                    self.add_button(); // On click, add button
                }
            }
        });
    }
}