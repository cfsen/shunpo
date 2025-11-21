use egui::{Key, KeyboardShortcut, Modifiers};

use crate::system;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ShunpoState {
    pub volume: i32,
    pub search: String,
    pub send_search: bool,
    pub mode: ShunpoMode,

    pub key_shortcut_next: KeyboardShortcut,
    pub key_shortcut_prev: KeyboardShortcut,
    pub key_shortcut_volume_up: KeyboardShortcut,
    pub key_shortcut_volume_down: KeyboardShortcut,
}
impl Default for ShunpoState {
    fn default() -> Self {
        Self {
            search: String::new(),
            send_search: false,
            volume: system::volume::get_volume().unwrap_or(0),
            mode: ShunpoMode::Clock,

            key_shortcut_next: KeyboardShortcut::new(Modifiers::CTRL, Key::N),
            key_shortcut_prev: KeyboardShortcut::new(Modifiers::CTRL, Key::P),
            key_shortcut_volume_up: KeyboardShortcut::new(Modifiers::CTRL, Key::K),
            key_shortcut_volume_down: KeyboardShortcut::new(Modifiers::CTRL, Key::J),
        }
    }
}

#[derive(Copy,Clone,serde::Deserialize, serde::Serialize)]
pub enum ShunpoMode {
    Launcher,
    Clock,
}
