use anyhow::{Context, Result};
use std::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

use crate::hyprland::structs::{Workspace, Client};

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
    serde_json::from_str(&output).context("Failed to parse workspaces")
}

/// Get all clients (windows)
pub fn get_clients() -> Result<Vec<Client>> {
    let output = hyprctl(&["clients"])?;
    serde_json::from_str(&output).context("Failed to parse clients")
}

/// Dispatch a Hyprland command
pub fn dispatch(cmd: &str) -> Result<()> {
    hyprctl(&["dispatch", cmd])?;
    Ok(())
}
