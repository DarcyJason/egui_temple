use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "EmptyApp", 
        options, 
        Box::new(|_cc| Box::new(EmptyApp::default()))
        );
}

struct EmptyApp;

impl Default for EmptyApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for EmptyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    }
}
