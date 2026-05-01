# Shunpo

A fast application launcher and status bar for Hyprland optimized for productivity oriented keyboard-first users.

## Features
- Minimalist launcher and status bar in one unified application
- Minimal screenspace footprint: Lives on Wayland's overlay layer, enabling applications to use the full height or width of your monitors.
- Hyprland integration: Listens for Hyprland events, repositioning to another monitor when an application enters fullscreen.
- Ripgrep support: search through projects and notes and open them directly from the launcher
- Terminal applications: Supports launching CLI applications too
- Fast: Performant fuzzy finding powered by [nucleo](https://github.com/helix-editor/nucleo)
- Volume control (pactl)

## Installation
**Note:** Shunpo is currently in testing emphasizing UX issue discovery.

1. Clone this repository and build: `cargo build --release`
2. Set up a binding in your `hyprland.conf`:
```
   bind = $mainMod CTRL, SPACE, exec, /path/to/shunpo
```
Shunpo is instance-aware via its own socket and will toggle between UI modes if another instance tries to run.

3. Launch Shunpo to generate your settings file: `~/.config/shunpo/config.toml`
4. Quit Shunpo with `:q` and adjust the configuration. Setting up monitor priority is highly recommended.

## Configuration
**Note:** When making changes to your configuration, it's recommended to launch Shunpo from a terminal in order to display any errors. If Shunpo fails to validate your config, it will revert to using defaults.

Configuration is located at `$HOME/.config/shunpo/config.toml` and is generated automatically on first launch.

| Key | Description | Example |
|:----|:------------|:--------|
| `version` | Shunpo version for this configuration. | `version = "0.1.0"` |
| `monitor_priority` | Ordered list of monitors where Shunpo will try to place itself (find monitor names with `hyprctl monitors`) | `monitor_priority = ["DP-3", "DP-2"]` |
| `terminal_path` | Path to your preferred terminal | `terminal_path = "/usr/bin/ghostty"` |
| `desktop_entries_paths` | Path to your .desktop files | - |
| `ripgrep_paths` | For use with `rg ` command. No subdirectories will be searched. | - |
| `script_paths` | For use with `! ` command. No subdirectories will be searched. | - |

## Commands

When Shunpo is running and in launcher mode, the following commands are available in the search field:

| Command | Description |
|:--------|:------------|
| `:q` | Quit Shunpo |
| `b [app]` | Launches a terminal running `[app]` |
| `:deepsleep` | Hide Shunpo by sending it to the background layer |
| `rg [term]` | Ripgrep through paths in `ripgrep_paths` for `[term]` |
| `! [script]` | Search scripts in `script_paths` paths for `[script]` |

## Keyboard shortcuts

When Shunpo is running and in launcher mode, the following keyboard shortcuts are available:

| Keystroke | Description |
|:--------|:------------|
| `Esc` | Switch to clock mode |
| `Return` | Empty search field: switch to clock mode |
| `Ctrl+p` | Select result above |
| `Ctrl+n` | Select result below |
| `Alt+b` | Move caret to end of preivous word |
| `Alt+f` | Move caret to beginning of next word |
| `Ctrl+a` | Move caret to start |
| `Ctrl+e` | Move caret to end |
| `Ctrl+w` | Delete word |
