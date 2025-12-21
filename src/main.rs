#[cfg(test)]
mod tests;

mod config;
mod coordinator;
mod hyprland;
mod search;
mod socket;
mod system;
mod ui_gtk4;

use std::process::exit;

use gtk4::glib::ExitCode;
use log::{info, error};
use single_instance::SingleInstance;
use tokio::sync::mpsc;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

use crate::{
    config::config::ShunpoConfig, coordinator::{listener::coordinator_run, types::{CoordinatorMessage, GuiMessage}}, search::listener::setup_search_listener, socket::{send_wakeup, shunpo_socket}
};


fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

fn main() -> ExitCode {
    // setup logger
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("shunpo=info"))
        .init();
    info!("Starting shunpo...");

    let rt = Runtime::new().expect("Failed to create Tokio runtime.");
    let _guard = rt.enter();

    // shunpo socket to coordinator
    let (shunpo_tx, shunpo_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
    // if no other instance of shunpo, set up socket. else send wakeup to running instance and exit.
    let _instance = setup_shunpo_socket_or_exit(shunpo_tx); // must be kept in scope

    // load config or exit
    let Ok(config) = ShunpoConfig::load_or_default() else {
        error!("Exiting.");
        exit(1)
    };

    // hyprland event listener to coordinator
    let (event_tx, event_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
    runtime().spawn(async move {
        if let Err(e) = hyprland::events::subscribe_events(event_tx, config.clone()).await {
            error!("Error in Hyprland listener: {:?}", e);
        }
    });

    // search to coordinator
    let (search_coord_tx, search_coord_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();
    // anywhere to search
    let (search_tx, search_rx) = mpsc::unbounded_channel::<String>();
    // setup search
    let _search_worker = setup_search_listener(search_rx, search_coord_tx);

    // coordinator to gui
    let (gui_tx, gui_rx) = async_channel::unbounded::<GuiMessage>();
    // gui to coordinator 
    let (feedback_tx, feedback_rx) = mpsc::unbounded_channel::<CoordinatorMessage>();

    // setup coordinator
    runtime().spawn(async move {
        // Pass gui_tx into the coordinator so it can send messages
        coordinator_run(event_rx, shunpo_rx, search_coord_rx, gui_tx, feedback_rx).await;
    });

    // setup renderer
    // NOTE: UNSAFE
    // prevent loading heavy nvidia/vulkan libraries by modifying global process environment.
    // this setting is removed when applications are launched.
    unsafe {

        // TODO: TODO_PRESERVE_ENV
        // capture value of GSK_RENDERER (if any) and set it back when launching apps
        std::env::set_var("GSK_RENDERER", "cairo");
    }

    // run GTK on the main thread, passing the receiver
    ui_gtk4::main_gtk4::run_shunpo(gui_rx, search_tx, feedback_tx)
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
