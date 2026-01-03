#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use log::{error, info};
use serde::de::DeserializeOwned;
use std::{env, process::Command};

use crate::hyprland::structs::{Client, Layers, Monitor, Workspace};

/// Execute a hyprctl command and return the output
pub fn hyprctl(args: &[&str]) -> Result<String> {
    let output = Command::new("hyprctl")
        .env_remove("GSK_RENDERER") // TODO: TODO_PRESERVE_ENV
        .args(args)
        .arg("-j") // JSON output
        .output()
        .context("Failed to execute hyprctl")?;

    if !output.status.success() {
        anyhow::bail!("hyprctl command failed");
    }

    String::from_utf8(output.stdout).context("Invalid UTF-8 in hyprctl output")
}

/// Get all workspaces
pub fn get_workspaces() -> Result<Vec<Workspace>> {
    let output = hyprctl(&["workspaces"])?;
    from_json_or_panic(&output, "get_workspaces")
}

/// Get all clients (windows)
pub fn get_clients() -> Result<Vec<Client>> {
    let output = hyprctl(&["clients"])?;
    from_json_or_panic(&output, "get_clients")
}

/// Get all monitors
pub fn get_monitors() -> Result<Vec<Monitor>> {
    let output = hyprctl(&["monitors"])?;
    from_json_or_panic(&output, "get_monitors")
}

/// Get all layers
pub fn get_layers() -> Result<Layers> {
    let output = hyprctl(&["layers"])?;
    from_json_or_panic(&output, "get_layers")
}

/// Dispatch a Hyprland command
pub fn dispatch(cmd: &str) -> Result<()> {
    hyprctl(&["dispatch", "exec", cmd])?;
    Ok(())
}

/// Dispatch a terminal
pub fn dispatch_from_term(bin: &str) -> Result<()> {
    // TODO: TODO_PRESERVE_ENV
    if let Ok(term) = env::var("TERM_PROGRAM") {
        info!("Dispatching: {}", bin);
        hyprctl(&["dispatch", "exec", &format!("{} -e sh -c '{}'", term, bin)])?;
        Ok(())
    }
    else {
        error!("Failed to fetch environment variable: TERM_PROGRAM");
        Err(anyhow!("Failed to fetch environment variable: TERM_PROGRAM"))
    }
}

/// Toggle floating for a client
pub fn toggle_floating_by_initialtitle(initial_title: &str) -> Result<()> {
    hyprctl(&["dispatch", "togglefloating",
        &format!("initialtitle:{}", initial_title)]
    )?;
    Ok(())
}

/// Resize a client
pub fn resize_client_by_initialtitle(initial_title: &str, width: u16, height: u16) -> Result<()> {
    hyprctl(&["dispatch", "resizewindowpixel", "exact",
        &width.to_string(),
        &height.to_string(),
        &format!(",initialtitle:{}", initial_title)]
    )?;
    Ok(())
}

/// Move a client
pub fn move_client_by_initialtitle(initial_title: &str, width: u16, height: u16) -> Result<()> {
    hyprctl(&["dispatch", "movewindowpixel", "exact",
        &width.to_string(),
        &height.to_string(),
        &format!(",initialtitle:{}", initial_title)]
    )?;
    Ok(())
}

/// Check if client is currently on a visible workspace
pub fn is_client_visible(client_name: &str) -> bool {
    let (Ok(monitors), Ok(clients)) = (get_monitors(), get_clients()) else {
        return false;
    };

    // find client_name
    let Some(cli) = clients.iter().find(|c| c.initial_title == client_name) else {
        return false;
    };

    // check if client_name's workspace is active
    if !monitors.iter().any(|m| cli.workspace.id == m.active_workspace.id) {
        return false;
    }

    // check for other clients in fullscreen on client_name's workspace
    if clients.iter().any(|c|
        c.fullscreen_client == 2 &&
        c.initial_title != client_name &&
        c.workspace.id == cli.workspace.id
    ) {
        return false;
    };

    true
}

/// Helper for debugging, if Hyprland updates change the JSON schema
pub fn from_json_or_panic<T: DeserializeOwned>(input: &str, context: &str) -> Result<T> {
    match serde_json::from_str::<T>(input) {
        Ok(v) => Ok(v),
        Err(err) => panic!(
            "Failed to parse {}:\n{}\n\n--- RAW OUTPUT BEGIN ---\n{}\n--- RAW OUTPUT END ---",
            context,
            err,
            input
        ),
    }
}
