use gtk4::{ApplicationWindow, Entry, Label, ListBox, Scale, ScrolledWindow};

use crate::{coordinator::types::WorkspaceMessage, search::entity_model::LauncherEntity};

pub struct ShunpoWidgets {
    pub window: ApplicationWindow,
    #[allow(dead_code)]
    pub clock: Label,
    pub workspaces: gtk4::Box,
    pub volume: Scale,
    pub search: Entry,
    pub results: ListBox,
    pub results_window: ScrolledWindow,
}


pub struct ShunpoState {
    pub ui_mode: UIMode,
    pub workspaces_data: Vec<WorkspaceMessage>,
    pub results_data: Vec<LauncherEntity>,
}
impl ShunpoState {
    pub fn new() -> Self {
        Self {
            ui_mode: UIMode::Launcher,
            results_data: Vec::new(),
            workspaces_data: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum UIMode {
    Launcher,
    Clock,
}
