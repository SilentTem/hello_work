#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod config;
mod db;

use std::time::{Duration, SystemTime};

use eframe::egui::{self, FontId, RichText, Ui};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false),
        ..Default::default()
    };
    eframe::run_native(
        "Hello Work",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Pomo>::default())
        }),
    )
}

struct Pomo {
    project: String,
    session_length: u64,
    session_start: Option<SystemTime>,
}

impl Pomo {
    fn is_running(&self) -> bool {
        self.session_start.is_some()
    }
    fn init_session(&mut self) {
        self.session_start = Some(SystemTime::now())
    }
    fn cancel_session(&mut self) {
        self.session_start = None
    }
    fn finish_session(&mut self) {
        self.session_start = None
    }
    fn check_finished(&mut self) {
        self.time_elapsed().map(|elapsed| {
            if elapsed.as_secs() >= self.session_length {
                self.finish_session();
            }
        });
    }
    fn time_elapsed(&self) -> Option<Duration> {
        self.session_start.and_then(|s| s.elapsed().ok())
    }
    fn countdown_string(&self) -> String {
        match self.time_elapsed() {
            Some(t) => {
                let secs = t.as_secs();
                let rem = self.session_length - secs;
                format!("{:02}:{:02}", rem / 60, rem % 60)
            }
            None => "--:--".to_owned(),
        }
    }
}

impl Default for Pomo {
    fn default() -> Self {
        Self {
            project: "Studying".to_owned(),
            session_start: None,
            session_length: 25 * 60,
        }
    }
}

fn mini_ui(pomo: &mut Pomo, ui: &mut Ui) {
    ui.heading(format!("{}", pomo.project));
    ui.label(RichText::new(pomo.countdown_string()).font(FontId::proportional(40.0)));
}

fn main_ui(pomo: &mut Pomo, ui: &mut Ui) {
    ui.heading("Hello Work");
    ui.horizontal(|ui| {
        let name_label = ui.label("Project: ");
        ui.text_edit_singleline(&mut pomo.project)
            .labelled_by(name_label.id);
    });
    let button = ui.button(if pomo.is_running() { "Cancel" } else { "Start" });
    if button.clicked() {
        if !pomo.is_running() {
            pomo.init_session()
        } else {
            pomo.cancel_session();
        }
    }
    ui.label(RichText::new(pomo.countdown_string()).font(FontId::proportional(40.0)));
}

impl eframe::App for Pomo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.available_width() > 200.0 && ui.available_height() > 200.0 {
                main_ui(self, ui);
            } else {
                mini_ui(self, ui);
            }
        });

        // repaint once the timer ticks to a whole second
        self.time_elapsed().map(|x| {
            ctx.request_repaint_after(Duration::from_millis(1000 - x.subsec_millis() as u64));
        });
        self.check_finished();
    }
}
