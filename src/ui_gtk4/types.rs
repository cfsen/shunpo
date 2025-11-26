use gtk4::{ApplicationWindow, Entry, Label, ListBox, Scale, ScrolledWindow};

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
}
impl ShunpoState {
    pub fn new() -> Self {
        Self {
            ui_mode: UIMode::Clock,
        }
    }
}

pub enum UIMode {
    Launcher,
    Clock,
}
