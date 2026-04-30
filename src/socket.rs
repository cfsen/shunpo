use log::{error, info};
use std::{fs, io::Write, path::Path};
use tokio::{io::AsyncReadExt, net::UnixListener, sync::mpsc};

use crate::{
    coordinator::types::{CoordinatorMessage, ShunpoSocketEventData}, socket_error::ShunpoSocketError,
};

struct ShunpoSocketPath {
    addr: String,
    dir: String,
}

fn get_shunpo_socket_path() -> Result<ShunpoSocketPath, ShunpoSocketError> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .map_err(|e| ShunpoSocketError::XdgRuntimeDir(e))?;

    let dir = format!("{}/shunpo", xdg_runtime_dir);
    let addr = format!("{}/.shunpo.sock", dir);


    Ok(ShunpoSocketPath {
        addr,
        dir,
    })
}

pub fn shunpo_socket(
    shunpo_tx: mpsc::UnboundedSender<CoordinatorMessage>
) -> Result<(), ShunpoSocketError> {
    let socket = get_shunpo_socket_path()?;

    // setup and cleanup
    fs::create_dir_all(&socket.dir)
        .map_err(|e| ShunpoSocketError::SocketCreateDir(e))?;

    let _ = fs::remove_file(&socket.addr);

    // setup socket
    let listener = UnixListener::bind(&socket.addr)
        .map_err(|e| ShunpoSocketError::SocketBind(e))?;

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
                let mut buf = [0u8; 32];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        recieve(&buf, &shunpo_tx);
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

fn recieve(buf: &[u8], shunpo_tx: &mpsc::UnboundedSender<CoordinatorMessage>) {
    if let Ok(opcode) = ShunpoSocketOp::try_from(&buf[0]) {
        info!("Received socket message: {}", opcode);
        let msg = match opcode {
            ShunpoSocketOp::Wakeup => CoordinatorMessage::ShunpoSocketEvent(
                ShunpoSocketEventData::ToggleUiMode
            ),
            ShunpoSocketOp::Hide => CoordinatorMessage::ShunpoSocketEvent(
                // TODO: impl send to background layer call
                ShunpoSocketEventData::ToggleUiMode
            )
        };

        if let Err(e) = shunpo_tx.send(msg) {
            error!("Shunpo socket failed to message coordinator: {}", e);
        }
    }
}

pub fn send_wakeup() -> Result<(), ShunpoSocketError> {
    let socket = get_shunpo_socket_path()?;

    if !Path::new(&socket.addr).exists() {
        return Err(ShunpoSocketError::LockHeldNoSocket)
    }

    // attempt to send wakeup to socket
    let mut stream = std::os::unix::net::UnixStream::connect(&socket.addr)
        .map_err(|e| ShunpoSocketError::StreamOpen(e))?;

    stream.write_all(&[ShunpoSocketOp::Wakeup as u8])
        .map_err(|e| ShunpoSocketError::StreamWrite(e))?;

    stream.flush()
        .map_err(|e| ShunpoSocketError::StreamFlush(e))?;

    info!("Sent wakeup message to running instance");
    Ok(())
}

#[repr(u8)]
enum ShunpoSocketOp {
    Wakeup = 0x00,
    Hide = 0x01,
}
impl std::fmt::Display for ShunpoSocketOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShunpoSocketOp::Wakeup => write!(f, "Wakeup"),
            ShunpoSocketOp::Hide => write!(f, "Hide"),
        }
    }
}
impl TryFrom<&u8> for ShunpoSocketOp {
    type Error = ShunpoSocketError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Wakeup),
            0x01 => Ok(Self::Hide),
            _ => Err(ShunpoSocketError::IOError),
        }
    }
}
