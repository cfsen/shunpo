# Shunpo

An application launcher and status bar for Hyprland, currently in testing. Shunpo aims to be low-config and distraction-free, with a minimalistic UI focused on readability.

## Features
- Dual mode UI layout: launcher and status bar in one application
- Smart visibility: uses layer-shell overlay but respects fullscreen applications
- Performant fuzzy finding, powered by [nucleo](https://github.com/helix-editor/nucleo)
- Supports launching CLI applications
- Built-in master volume control
- Minimal configuration: works out of the box

## Installation
**Note:** Shunpo is still in early testing and not yet ready for general use.

1. Clone this repository and build: `cargo build --release`
2. Set up a binding in your `hyprland.conf`:
```
   bind = $mainMod CTRL, SPACE, exec, TERM_PROGRAM=/usr/bin/ghostty /path/to/shunpo
```
   Shunpo is instance-aware via its own socket and will toggle between UI modes when another instance tries to run. Set `TERM_PROGRAM` to your preferred terminal.
3. Proceed to configuration

## Configuration

Configuration is located at `$HOME/.config/shunpo/config.toml` and is generated automatically on first launch.

| Key | Description | Example |
|:----|:------------|:--------|
| `monitor_priority` | Ordered list of monitors where Shunpo will try to place itself (find monitor names with `hyprctl monitors`) | `monitor_priority = ["DP-3", "DP-2"]` |
| `terminal_path` | Path to your preferred terminal | `terminal_path = "/usr/bin/ghostty"` |

## Commands

When Shunpo is running, these commands are available in the search field:

| Command | Description |
|:--------|:------------|
| `b [app]` | Launches a terminal running `[app]` |
| `:deepsleep` | Temporarily hide Shunpo by sending it to the background layer |
