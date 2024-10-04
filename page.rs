use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "EmptyApp", 
        options, 
        Box::new(|_cc| Ok(Box::new(EmptyApp::default())))
        );
}

enum Page {
    HomePage,
    TestPage,
}

struct EmptyApp {
    current_page: Page,
}

impl Default for EmptyApp {
    fn default() -> Self {
        Self {
            current_page: Page::HomePage,
        }
    }
}

impl eframe::App for EmptyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.current_page {
            Page::HomePage => self.show_home_page(ctx),
            Page::TestPage => self.show_test_page(ctx),
        }
    }
}

impl EmptyApp {
    fn show_home_page(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Home");
            if ui.button("Go to test page").clicked() {
                self.current_page = Page::TestPage;
            }
        });
    }

    fn show_test_page(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Test");
            if ui.button("Go to home page").clicked() {
                self.current_page = Page::HomePage;
            }
        });
    }
}
