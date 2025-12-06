use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
    sync::mpsc::UnboundedSender,
};
use log::{debug, error, info};

use crate::coordinator::types::{CoordinatorMessage, HyprlandEventData};

/// Subscribe to Hyprland events
pub async fn subscribe_events(tx: UnboundedSender<CoordinatorMessage>) -> Result<()> {
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
        // let _ = HyprlandEvent::parse_event(&line);
        let _ = tx.send(CoordinatorMessage::HyprlandEvent(HyprlandEventData {
            raw_event: line,
        }));
    }

    Ok(())
}
