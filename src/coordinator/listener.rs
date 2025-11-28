use gtk4::glib;
use log::{debug, error, info};
use tokio::sync::mpsc;

use crate::{coordinator::{error::CoordinatorError, types::{
    CoordinatorMessage, FeedbackData, GuiMessage, HyprlandEventData, RipgrepResultData, SearchMessageData, ShunpoSocketEventData
}}, hyprland::hyprctl::{dispatch, dispatch_from_term}};

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
            Some(CoordinatorMessage::HyprlandEvent(msg)) = hyprland_rx.recv()
            => { log_error(handle_hyprland(msg), "Hyprland handler"); },

            Some(CoordinatorMessage::ShunpoSocketEvent(msg)) = shunpo_rx.recv()
            => { log_error(handle_shunpo_socket(msg, &gui_tx).await, "Socket handler"); },

            Some(CoordinatorMessage::SearchMessage(msg)) = search_coord_rx.recv()
            => { log_error(handle_search(msg, &gui_tx).await, "Search handler"); },

            Some(CoordinatorMessage::Feedback(msg)) = feedback_rx.recv()
            => { log_error(handle_feedback(msg, &gui_tx).await, "Feedback handler"); },

            else => {
                info!("All input channels closed. Exiting coordinator loop.");
                break;
            },
        }}
    Ok(())
}

//
// handlers
//

fn handle_hyprland(msg: HyprlandEventData) -> Result<(), CoordinatorError> {
    debug!("HyprlandEvent: {}", msg.raw_event);
    Ok(())
}

async fn handle_shunpo_socket(
    msg: ShunpoSocketEventData,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    let gui_cmd = match msg {
        ShunpoSocketEventData::Wake => GuiMessage::Wake,
        ShunpoSocketEventData::Sleep => GuiMessage::Sleep,
    };

    gui_tx.send(gui_cmd).await?;
    Ok(())
}

async fn handle_search(
    msg: SearchMessageData,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    let gui_cmd = GuiMessage::DisplayResults(msg);

    gui_tx.send(gui_cmd).await?;
    Ok(())
}

async fn handle_feedback(
    msg: FeedbackData,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    let gui_cmd = match msg {
        FeedbackData::Sleep => { GuiMessage::Sleep },
        FeedbackData::Run(run) => {
            // TODO: handle running with/without terminal

            if let Err(dispatch_err) = dispatch_from_term(&run) {
                return Err(CoordinatorError::FeedbackError(
                    format!("Hyprland terminal dispatcher: {}", dispatch_err)
                ));
            }

            // TODO: focus opened window (coordinate with hyprland event listener)

            GuiMessage::Sleep
        },
    };

    gui_tx.send(gui_cmd).await?;
    Ok(())
}

//
// helpers
//

fn log_error<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) {
    if let Err(e) = result {
        error!("{}: {}", context, e);
    }
}
