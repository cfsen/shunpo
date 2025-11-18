mod app;
mod hyprland;
mod state;
mod socket;
mod system;
mod ui;

use std::process::exit;

use eframe;
use eframe::egui;
use chrono::Local;
use anyhow::Result;
use log::{info, error};
use single_instance::SingleInstance;
use tokio::sync::mpsc;

use crate::{app::Shunpo, socket::{shunpo_socket, send_wakeup}};

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    // setup logger
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("shunpo=debug"))
        .init();
    info!("Starting shunpo...");

    // ensure single instance and set up or notify shunpo socket
    let _instance = setup_shunpo_socket_or_exit();

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

fn setup_shunpo_socket_or_exit() -> SingleInstance {
    let instance = SingleInstance::new("shunpo")
        .unwrap_or_else(|e| {
            error!("SingleInstance error: {}", e);
            exit(1)
        });

    if instance.is_single() {
        match shunpo_socket() {
            Ok(_) => {
                info!("Shunpo socket started.");
                instance
            }
            Err(e) => {
                error!("Socket error: {}", e);
                exit(1)
            }
        }
    }
    else {
        match send_wakeup() {
            Ok(_) => {
                info!("The running instance of Shunpo has been notified to wake up.");
                info!("Exiting.");
                exit(0)
            }
            Err(e) => {
                error!("Failed to connect to socket: {}", e);
                exit(1)
            }
        }
    }
}
