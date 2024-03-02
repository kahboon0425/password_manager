#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use login_page::LoginPage;

mod add_password_form;
mod login_page;
mod password_view;
mod top_panel;

pub trait Window {
    fn show_window(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(MyApp {
                login_status: false,
                login_page: login_page::LoginPage::default(),
            })
        }),
    )
}

struct MyApp {
    login_status: bool,
    login_page: login_page::LoginPage,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.login_status == false {
            self.login_status = self.login_page.show(ctx);
        } else {
            // self.login_page
        }
    }
}
