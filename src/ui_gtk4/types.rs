use gtk4::{ApplicationWindow, Entry, Label, ListBox, Scale, ScrolledWindow};

use crate::search::entity_model::LauncherEntity;

pub struct ShunpoWidgets {
    pub window: ApplicationWindow,
    pub clock: Label,
    pub volume: Scale,
    pub search: Entry,
    pub results: ListBox,
    pub results_window: ScrolledWindow,
}


pub struct ShunpoState {
    pub ui_mode: UIMode,
    pub results_data: Vec<LauncherEntity>,
}
impl ShunpoState {
    pub fn new() -> Self {
        Self {
            ui_mode: UIMode::Clock,
            results_data: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum UIMode {
    Launcher,
    Clock,
}
