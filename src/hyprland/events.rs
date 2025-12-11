use anyhow::{Context, Result};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::UnixStream,
    sync::mpsc::UnboundedSender,
};
use log::{debug, error, info, warn};

use crate::{coordinator::types::{CoordinatorMessage, GuiMessage, HyprlandEventData}, hyprland::{event_parser::HyprlandEvent, state::HyprlandState, structs::{FullscreenEvent, LayerLevel, MonitorName, Namespace, WorkspaceId}}};

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
fn update_state(state: &mut HyprlandState, event: HyprlandEvent) -> Option<CoordinatorMessage> {
    match event {
        // rebuild state when wayland layers with shunpo namespace are created.
        // occurs on app launch for initial state, and on changes to or from deep sleep mode
        HyprlandEvent::Openlayer { nspace } => {
            if nspace == state.shunpo_namespace {
                if let Err(e) = state.rebuild() {
                    error!("An error occurred while rebuilding HyprlandState: {}", e);
                }
            }
        },
        // monitor focus change
        HyprlandEvent::Focusedmonv2 { mname, .. } => {
            state.update_focused_monitor(mname);
        },
        HyprlandEvent::Workspacev2 { .. } => {
            if let Err(e) = state.rebuild() {
                error!("Workspacev2: Failed to rebuild state: {}", e);
            }
            return check_and_retarget(state);
        }
        HyprlandEvent::Createworkspacev2 { .. } => {
            if let Err(e) = state.rebuild_workspaces() {
                error!("Createworkspacev2: Failed to rebuild workspaces: {}", e);
            }
        },
        HyprlandEvent::Destroyworkspacev2 { wid, .. } => {
            if let Err(e) = state.remove_workspace(wid) {
                error!("Destroyworkspacev2: Failed to remove workspace: {}", e);
            }
        },
        HyprlandEvent::Monitoraddedv2 { .. } => {
            if let Err(e) = state.rebuild_monitors() {
                error!("Monitoraddedv2: Failed to rebuild monitors: {}", e);
            }
        },
        HyprlandEvent::Monitorremovedv2 { mname, .. } => {
            if let Err(e) = state.remove_monitor(mname) {
                error!("Monitorremovedv2: Failed to remove monitor: {}", e);
            }
        },
        HyprlandEvent::Fullscreen { .. } => {
            if let Err(e) = state.rebuild() {
                error!("Fullscreen: Failed to rebuild state: {}", e);
            }
            return check_and_retarget(state);
        },
        _ => {},
    }
    None
}
fn check_and_retarget(state: &mut HyprlandState) -> Option<CoordinatorMessage> {
    if !state.shunpo_should_retarget().is_ok_and(|retarget| retarget) {
        return None;
    }

    match state.shunpo_get_target() {
        Ok((target_monitor, target_layer)) => {
            return Some(package_gui_message(target_monitor, target_layer))
        },
        Err(e) => {
            error!("shunpo_get_target failed: {}", e);
            None
        },
    }
}
fn package_gui_message(target_monitor: &MonitorName, target_layer: LayerLevel) -> CoordinatorMessage {
    CoordinatorMessage::HyprlandEvent(HyprlandEventData {
        gui_msg: GuiMessage::WaylandMonitorLayer {
            target_monitor: target_monitor.clone(),
            target_layer,
        }
    })
}
