#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};

use eframe::egui::{self, Ui};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false),
        ..Default::default()
    };
    eframe::run_native(
        "Hello Kitty Work",
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
    fn init_session(&mut self) {
        self.session_start = Some(SystemTime::now())
    }
    fn stop_session(&mut self) {
        self.session_start = None
    }
    fn time_elapsed(&self) -> Option<Duration> {
        self.session_start.and_then(|s| s.elapsed().ok())
    }
    fn countdown_string(&self) -> String {
        match self.time_elapsed() {
            Some(t) => {
                let secs = t.as_secs();
                let rem = self.session_length - secs;
                format!("{}:{}", rem / 60, rem % 60)
            }
            None => "--:--".to_owned(),
        }
    }
}

impl Default for Pomo {
    fn default() -> Self {
        Self {
            project: "Arthur".to_owned(),
            session_start: None,
            session_length: 25 * 60,
        }
    }
}

fn mini_ui(pomo: &mut Pomo, ui: &mut Ui) {
    ui.heading(format!("{}", pomo.project));
    //ui.add(egui::Slider::new(&mut pomo.time_elapsed, 0..=120).text("age"));
    if ui.button("Start").clicked() {
        pomo.init_session()
    }
}

fn main_ui(pomo: &mut Pomo, ui: &mut Ui) {
    ui.heading("Hello Kitty Work");
    ui.horizontal(|ui| {
        let name_label = ui.label("Project name: ");
        ui.text_edit_singleline(&mut pomo.project)
            .labelled_by(name_label.id);
    });
    //ui.add(egui::Slider::new(&mut pomo.time_elapsed, 0..=120).text("age"));
    if ui.button("Start").clicked() {
        pomo.init_session()
    }
    ui.label(format!(
        "Project: '{}', Time: {}",
        pomo.project,
        pomo.countdown_string()
    ));
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
            ctx.request_repaint_after(Duration::from_millis(1000 - x.subsec_millis() as u64))
        });
    }
}
