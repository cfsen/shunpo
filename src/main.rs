mod app;
mod hyprland;
mod system;

use eframe;
use eframe::egui;
use chrono::Local;
use anyhow::Result;
use log::{info, error};
use tokio::sync::mpsc;

use crate::app::Shunpo;

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
