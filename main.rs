use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "EmptyApp", 
        options, 
        Box::new(|_cc| Ok(Box::new(EmptyApp::default())))
        );
}

struct EmptyApp;

impl Default for EmptyApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for EmptyApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }
}
