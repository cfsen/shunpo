use log::{debug, error, info};
use tokio::sync::mpsc;

use crate::coordinator_types::{
    CoordinatorMessage,
    HyprlandEventData,
    ShunpoSocketEventData,
    RipgrepResultData,
    GuiMessage,
};

pub fn coordinator_run(
    hyprland_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    shunpo_rx: mpsc::UnboundedReceiver<CoordinatorMessage>
) -> mpsc::UnboundedReceiver<GuiMessage> {

    let (gui_tx, gui_rx) = mpsc::unbounded_channel::<GuiMessage>();

    tokio::spawn(async move {
        if let Err(e) = coordinator_listener(gui_tx, hyprland_rx, shunpo_rx).await {
            error!("Coordinator loop exited with error: {:?}", e);
        }
    });

    gui_rx
}

async fn coordinator_listener(
    gui_tx: mpsc::UnboundedSender<GuiMessage>,
    mut hyprland_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut shunpo_rx: mpsc::UnboundedReceiver<CoordinatorMessage>
) -> Result<(),Box<dyn std::error::Error + Send + Sync>> {
    loop {
        tokio::select! {
            Some(msg) = hyprland_rx.recv() => {
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
            },

            Some(msg) = shunpo_rx.recv() => {
                match msg {
                    CoordinatorMessage::ShunpoSocketEvent(event) => { 
                        let gui_cmd = match event {
                            ShunpoSocketEventData::Wake => GuiMessage::Wake,
                            ShunpoSocketEventData::Sleep => GuiMessage::Sleep,
                        };
                        gui_tx.send(gui_cmd)?;
                    }
                    _ => {
                        // TODO: log unexpected
                    }
                }
            },

            else => {
                info!("All input channels closed. Exiting coordinator loop.");
                break;
            }
        }
    }
    Ok(())
}
