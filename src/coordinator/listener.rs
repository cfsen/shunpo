use gtk4::glib;
use log::{debug, error, info};
use tokio::sync::mpsc;

use crate::{coordinator::types::{
    CoordinatorMessage, FeedbackData, GuiMessage, HyprlandEventData, RipgrepResultData, ShunpoSocketEventData
}, hyprland::hyprctl::{dispatch, dispatch_from_term}};

pub async fn coordinator_run(
    hyprland_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    shunpo_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    search_coord_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    gui_tx: async_channel::Sender<GuiMessage>,
    feedback_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
) {
    tokio::spawn(async move {
        if let Err(e) = coordinator_listener(
            gui_tx,
            hyprland_rx,
            shunpo_rx,
            search_coord_rx,
            feedback_rx
        ).await {
            error!("Coordinator loop exited with error: {:?}", e);
        }
    });
}

async fn coordinator_listener(
    gui_tx: async_channel::Sender<GuiMessage>,
    mut hyprland_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut shunpo_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut search_coord_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut feedback_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
) -> Result<(),Box<dyn std::error::Error + Send + Sync>> {
    loop {
        tokio::select! {
            Some(msg) = hyprland_rx.recv() => { handle_hyprland(msg); },
            Some(msg) = shunpo_rx.recv() => { let _ = handle_shunpo_socket(msg, &gui_tx).await?; },
            Some(msg) = search_coord_rx.recv() => { let _ = handle_search(msg, &gui_tx).await?; },
            Some(msg) = feedback_rx.recv() => { let _ = handle_feedback(msg, &gui_tx).await?; },

            else => {
                info!("All input channels closed. Exiting coordinator loop.");
                break;
            },
        }}
    Ok(())
}

fn handle_hyprland(msg: CoordinatorMessage) {
    match msg {
        CoordinatorMessage::HyprlandEvent(event) => {
            // TODO: hyprland event handling
            // TODO: shunpo hyprland client state tracking
            debug!("HyprlandEvent: {}", event.raw_event);
        }
        _ => {
            // TODO: log unexpected
        }
    }
}
async fn handle_shunpo_socket(
    msg: CoordinatorMessage,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg {
        CoordinatorMessage::ShunpoSocketEvent(event) => { 
            let gui_cmd = match event {
                ShunpoSocketEventData::Wake => GuiMessage::Wake,
                ShunpoSocketEventData::Sleep => GuiMessage::Sleep,
            };

            // TODO: error handling
            let _ = gui_tx.send(gui_cmd).await?;
        }
        _ => {
            // TODO: log unexpected
        }
    }
    Ok(())
}

async fn handle_search(
    msg: CoordinatorMessage,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg {
        CoordinatorMessage::SearchMessage(search) => {
            info!("SearchMessage:");
            info!("{}", search.success);
            let gui_cmd = GuiMessage::DisplayResults(search);

            // TODO: error handling
            let _ = gui_tx.send(gui_cmd).await?;
        }
        _ => {
            // TODO: log unexpected
        }
    }
    Ok(())
}

async fn handle_feedback(
    msg: CoordinatorMessage,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg {
        CoordinatorMessage::Feedback(event) => { 
            let gui_cmd = match event {
                FeedbackData::Sleep => { GuiMessage::Sleep },
                FeedbackData::Run(run) => { 
                    // TODO: handle running with/without terminal
                    let _ = dispatch_from_term(&run);
                    GuiMessage::Sleep
                },
            };

            // TODO: error handling
            let _ = gui_tx.send(gui_cmd).await?;
        }
        _ => {
            // TODO: log unexpected
        }
    }
    Ok(())
}
