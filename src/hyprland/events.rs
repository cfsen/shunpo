use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;
use tokio::sync::mpsc::UnboundedSender;
use log::{debug, info};

/// Subscribe to Hyprland events
pub async fn subscribe_events(tx: UnboundedSender<String>) -> Result<()> {
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
        let _ = tx.send(line);
    }

    Ok(())
}
