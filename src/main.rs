mod hyprland;
mod system;

use eframe;
use eframe::egui;
use chrono::Local;
use anyhow::Result;
use log::{debug, info, error};
use tokio::sync::mpsc;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Shunpo {
    search: String,
    selected: usize,
    volume: i32,
    #[serde(skip)]
    event_rx: mpsc::UnboundedReceiver<String>, // runtime-only, needs to be set on resume
}

impl Default for Shunpo {
    fn default() -> Self {
        let (_tx, event_rx) = mpsc::unbounded_channel(); // dummy to satisfy the requirements for default
        Self {
            search: String::new(),
            selected: 0,
            volume: system::volume::get_volume().unwrap_or(0),
            event_rx
        }
    }
}
impl Shunpo {
    pub fn new(cc: &eframe::CreationContext<'_>, rx: mpsc::UnboundedReceiver<String>) -> Self {
        let mut app: Shunpo = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.event_rx = rx;
        app
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
                if ui.add(egui::Slider::new(&mut self.volume, 0..=100).orientation(egui::SliderOrientation::Vertical)).changed() {
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

            // log hyprland events
            while let Ok(event) = self.event_rx.try_recv() {
                if let Some((event_type, data)) = event.split_once(">>") {
                    match event_type {
                        "workspace" => debug!("Workspace changed: {}", data),
                        "activewindow" => debug!("Active window: {}", data),
                        _ => {}
                    }
                }
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    // setup logger
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("shunpo=debug"))
        .init();
    info!("Starting shunpo...");

    // setup event listener
    let (event_tx, event_rx) = mpsc::unbounded_channel();
    tokio::spawn(async {
        if let Err(e) = hyprland::events::subscribe_events(event_tx).await {
            error!("Error in Hyprland listener: {:?}", e);
        }
    });

    // setup app
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
        Box::new(|cc| Ok(Box::new(Shunpo::new(cc, event_rx)))),
    )
}
