mod app;
mod coordinator;
mod coordinator_types;
mod hyprland;
mod keyboard_input;
mod search;
mod state;
mod socket;
mod system;
mod ui;

use std::process::exit;

use eframe;
use eframe::egui;
use anyhow::Result;
use log::{info, error};
use single_instance::SingleInstance;
use tokio::sync::mpsc;

use crate::{
    app::Shunpo,
    coordinator::coordinator_run,
    coordinator_types::CoordinatorMessage,
    search::listener::setup_search_listener,
    socket::{send_wakeup, shunpo_socket},
};

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    // setup logger
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("shunpo=debug"))
        .init();
    info!("Starting shunpo...");

    // shunpo socket to coordinator
    let (shunpo_tx, shunpo_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
    // if no other instance of shunpo, set up socket. else send wakeup to running instance and exit.
    let _instance = setup_shunpo_socket_or_exit(shunpo_tx); // must be kept in scope

    // hyprland event listener to coordinator
    let (event_tx, event_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
    tokio::spawn(async {
        if let Err(e) = hyprland::events::subscribe_events(event_tx).await {
            error!("Error in Hyprland listener: {:?}", e);
        }
    });

    // search to coordinator
    let (search_coord_tx, search_coord_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
    // anywhere to search
    let (search_tx, search_rx) = mpsc::unbounded_channel::<String>();
    // setup search
    let _search_worker = setup_search_listener(search_rx, search_coord_tx);

    // setup coordinator
    let gui_rx = coordinator_run(event_rx, shunpo_rx, search_coord_rx);

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
        Box::new(|cc| Ok(Box::new(Shunpo::new(cc, gui_rx, search_tx)))),
    )
}

fn setup_shunpo_socket_or_exit(shunpo_tx: mpsc::UnboundedSender<CoordinatorMessage>) -> SingleInstance {
    let instance = SingleInstance::new("shunpo")
        .unwrap_or_else(|e| {
            error!("SingleInstance error: {}", e);
            exit(1)
        });

    if instance.is_single() {
        match shunpo_socket(shunpo_tx) {
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
