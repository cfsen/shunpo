mod hyprland;
mod system;

use eframe;
use eframe::egui;
use chrono::Local;
use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Shunpo {
    search: String,
    selected: usize,
    volume: i32,
}

impl Default for Shunpo {
    fn default() -> Self {
        Self {
            search: String::new(),
            selected: 0,
            volume: system::volume::get_volume().unwrap_or(0),
        }
    }
}
impl Shunpo {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for Shunpo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // clock
            let time = Local::now().format("%H:%M:%S").to_string();
            ui.heading(time);
            ui.separator();

            // volume
            ui.horizontal(|ui| {
                ui.label("Vol:");
                if ui.add(egui::Slider::new(&mut self.volume, 0..=100)).changed() {
                    let _ = system::volume::set_volume(self.volume);
                }
            });
            ui.separator();

            // search
            let resp = ui.text_edit_singleline(&mut self.search);

            // handle keys
            if resp.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::J)) {
                    self.selected += 1;
                }
                if ui.input(|i| i.key_pressed(egui::Key::K)) && self.selected > 0 {
                    self.selected -= 1;
                }
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    std::process::exit(0);
                }
            }

            ui.label(format!("Selected: {}", self.selected));
            ui.label("TODO: results here");
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("shunpo")
            .with_inner_size([400.0, 300.0])
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "shunpo",
        options,
        Box::new(|cc| Ok(Box::new(Shunpo::new(cc)))),
    )
}
