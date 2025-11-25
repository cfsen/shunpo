use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;
use tokio::sync::mpsc::UnboundedSender;
use log::{debug, error, info};

use crate::coordinator::types::{CoordinatorMessage, HyprlandEventData};
use crate::hyprland::hyprctl;

/// Subscribe to Hyprland events
pub async fn subscribe_events(tx: UnboundedSender<CoordinatorMessage>) -> Result<()> {
    // setup initial position
    let mut initialized = false;

    // get env vars
    let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .context("HYPRLAND_INSTANCE_SIGNATURE not set")?;
    let xdg_runtime_path = std::env::var("XDG_RUNTIME_DIR")
        .context("Unable to open socket")?;
    let socket_path = format!("{}/hypr/{}/.socket2.sock", xdg_runtime_path, signature);

    let stream = UnixStream::connect(&socket_path)
        .await
        .context("Failed to connect to Hyprland event socket")?;

    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    info!("Listening for Hyprland events...");

    while let Some(line) = lines.next_line().await? {
        debug!("Hyperland event: {}", line);
        if !initialized
        && line.len() > 16
        && &line.as_bytes()[..10] == b"openwindow"
        && &line.as_bytes()[&line.as_bytes().len()-6..] == b"shunpo"
        {
            initialized = true;
            shunpo_initial_position();
        }
        let _ = tx.send(CoordinatorMessage::HyprlandEvent(HyprlandEventData {
            raw_event: line,
        }));
    }

    Ok(())
}

fn shunpo_initial_position(){
    info!("Init: Shunpo to clock mode.");
    if let Err(e) = hyprctl::toggle_floating_by_initialtitle("shunpo") {
        error!("Failed to float window: {}", e);
    }
    if let Err(e) = hyprctl::resize_client_by_initialtitle("shunpo", 100, 100) {
        error!("Failed to move window: {}", e);
    }
    if let Err(e) = hyprctl::move_client_by_initialtitle("shunpo", 0, 1340) {
        error!("Failed to position window: {}", e);
    }
}
