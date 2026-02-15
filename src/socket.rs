use anyhow::{Context, Result};
use log::{error, info};
use std::{fs, io::Write, path::Path};
use tokio::{io::AsyncReadExt, net::UnixListener, sync::mpsc};

use crate::coordinator::types::{CoordinatorMessage, ShunpoSocketEventData};

struct ShunpoSocketPath {
    addr: String,
    dir: String,
}

fn get_shunpo_socket_path() -> Result<ShunpoSocketPath> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .context("Failed reading env: XDG_RUNTIME_DIR.")?;

    let dir = format!("{}/shunpo", xdg_runtime_dir);
    let addr = format!("{}/.shunpo.sock", dir);


    Ok(ShunpoSocketPath {
        addr,
        dir,
    })
}

pub fn shunpo_socket(shunpo_tx: mpsc::UnboundedSender<CoordinatorMessage>) -> Result<()> {
    let socket = get_shunpo_socket_path()?;

    // setup and cleanup
    fs::create_dir_all(&socket.dir)?;
    let _ = fs::remove_file(&socket.addr);

    // setup socket
    let listener = UnixListener::bind(&socket.addr)
        .context("Failed to bind socket")?;

    // run listener
    tokio::spawn(async move {
        socket_listener(listener, shunpo_tx).await;
    });

    Ok(())
}

async fn socket_listener(listener: UnixListener, shunpo_tx: mpsc::UnboundedSender<CoordinatorMessage>) {
    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                let mut buf = [0u8; 64];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        // TODO: message parsing
                        let msg = String::from_utf8_lossy(&buf[..n]);
                        info!("Received socket message: {}", msg);
                        let _ = shunpo_tx.send(
                            CoordinatorMessage::ShunpoSocketEvent(ShunpoSocketEventData::ToggleUiMode)
                        );
                    }
                    _ => {}
                }
            }
            Err(e) => {
                info!("Error accepting connection: {}", e);
            }
        }
    }
}

pub fn send_wakeup() -> Result<()> {
    let socket = get_shunpo_socket_path()?;

    if !Path::new(&socket.addr).exists() {
        error!("Lock held but no socket found.");
        return Err(anyhow::anyhow!("Try running Shunpo again."))
    }

    // attempt to send wakeup to socket
    let mut stream = std::os::unix::net::UnixStream::connect(&socket.addr)
        .context("Failed to connect to existing instance")?;
    stream.write_all(b"wakeup")?;
    stream.flush()?;
    info!("Sent wakeup message to running instance");
    Ok(())
}
