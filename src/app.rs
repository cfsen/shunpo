use eframe;
use eframe::egui;
use chrono::Local;
use log::debug;
use log::info;
use tokio::sync::mpsc;

use crate::state::ShunpoState;
use crate::system;
use crate::ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Shunpo {
    state: ShunpoState,
    #[serde(skip)]
    event_rx: mpsc::UnboundedReceiver<String>, // runtime-only, needs to be set on resume
}

impl Default for Shunpo {
    fn default() -> Self {
        let (_tx, event_rx) = mpsc::unbounded_channel(); // dummy to satisfy the requirements for default
        Self {
            state: ShunpoState::default(),
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
            ui.separator();

            // search

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
