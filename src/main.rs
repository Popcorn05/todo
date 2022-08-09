extern crate eframe;

use eframe::egui::{self, RichText};
use egui::{vec2, Vec2};

fn main() {
    let options = eframe::NativeOptions { // Set window options
        min_window_size: Some(vec2(500.0, 500.0)),
        decorated: false,
        transparent: true,
        ..Default::default()
    };
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
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure background is clear
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        custom_window(ctx, frame, "Todo", |ui| {
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

fn custom_window(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    //COME BACK HERE TO CONTINUE IMPLEMENTATION
    //https://github.com/emilk/egui/blob/master/examples/custom_window_frame/src/main.rs

    use egui::{emath::Align2, Stroke, Button, Id, Sense};
    use eframe::epaint::{Rect};

    let text_color = ctx.style().visuals.text_color(); // Get context text color
    let height = 28.0; // Height of title bar

    // Central panel holder
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            // Util variables
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint frame
            painter.rect(
                rect.shrink(1.0), 
                10.0, 
                ctx.style().visuals.window_fill(), 
                Stroke::new(1.0, text_color),
            );

            // Paint title
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                egui::FontId::proportional(height - 2.0),
                text_color,
            );

            // Paint line under title
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ], 
                Stroke::new(1.0, text_color),
            );

            // Close button
            let close_response = ui.put(
                Rect::from_min_size(rect.right_top() + vec2(-27.0, 1.0), Vec2::splat(height)),
                Button::new(RichText::new("X").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.quit();
            }

            // Interaction (i.e. drag move window)
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add window contents
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }.shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}