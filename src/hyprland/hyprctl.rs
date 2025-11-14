use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::process::Command;

use crate::hyprland::structs::{Client, Monitor, Workspace};

/// Execute a hyprctl command and return the output
pub fn hyprctl(args: &[&str]) -> Result<String> {
    let output = Command::new("hyprctl")
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
    from_json_or_panic(&output, "get_client")
}

/// Get all clients (windows)
pub fn get_clients() -> Result<Vec<Client>> {
    let output = hyprctl(&["clients"])?;
    from_json_or_panic(&output, "get_client")
}

/// Get all monitors
pub fn get_monitors() -> Result<Vec<Monitor>> {
    let output = hyprctl(&["monitors"])?;
    from_json_or_panic(&output, "get_client")
}

/// Dispatch a Hyprland command
pub fn dispatch(cmd: &str) -> Result<()> {
    hyprctl(&["dispatch", cmd])?;
    Ok(())
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
