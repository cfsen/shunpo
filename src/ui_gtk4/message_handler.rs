use gtk4::gdk::Monitor;
use gtk4::prelude::*;
use gtk4::{
    Label, Box, Orientation, 
    ListBoxRow
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use log::{info, warn};

use crate::coordinator::types::GuiMessage;
use crate::hyprland::structs::{LayerLevel, MonitorName};
use crate::ui_gtk4::errors::ShunpoGtk4Error;
use crate::ui_gtk4::types::{ShunpoState, ShunpoWidgets, UIMode};

pub fn handle_ui_message(msg: GuiMessage, widgets: &ShunpoWidgets, state: &mut ShunpoState) {
    match msg {
        GuiMessage::UpdateWorkspace(workspaces) => {
            info!("gtk received update workspace message");
            // clear existing
            while let Some(child) = widgets.workspaces.first_child() {
                widgets.workspaces.remove(&child);
            }

            // update state
            state.workspaces_data = workspaces.clone();

            // populate new workspace indicators
            for workspace in workspaces {
                let ws_box = gtk4::Box::new(Orientation::Horizontal, 0);
                ws_box.add_css_class(if workspace.focused { "ws-active-bg" } else { "ws-inactive-bg" });

                let label = Label::new(Some(&workspace.id));
                label.add_css_class("ws-label");
                ws_box.append(&label);

                widgets.workspaces.append(&ws_box);
            }
        },
        GuiMessage::DisplayResults(data) => {
            // clear existing results
            while let Some(child) = widgets.results.first_child() {
                widgets.results.remove(&child);
            }

            // update state
            state.results_data = data.results.clone();

            if data.results.len() == 0 {
                return;
            }

            // populate new results
            for entity  in data.results {
                let row = ListBoxRow::new();
                let box_ = Box::new(Orientation::Horizontal, 10);
                box_.set_margin_top(5);
                box_.set_margin_bottom(5);
                box_.set_margin_start(10);

                let label = Label::new(Some(&entity.alias.to_string()));
                box_.append(&label);

                row.set_child(Some(&box_));
                widgets.results.append(&row);
            }

            // select first result
            if let Some(target_row_idx) = widgets.results.row_at_index(
                widgets.results.selected_row().map_or_else(|| 0, |row| row.index())
            ){
                widgets.results.select_row(Some(&target_row_idx));
            }
        },
        _ => { ui_mode_from_gui_message(msg, widgets, state); },
    }
}

fn ui_mode_from_gui_message(msg: GuiMessage, widgets: &ShunpoWidgets, state: &mut ShunpoState) {
    // shadow msg on toggle messages
    let msg = match msg {
        GuiMessage::ToggleUiMode => { 
            match state.ui_mode {
                UIMode::Launcher => { GuiMessage::Sleep },
                UIMode::Clock => { GuiMessage::Wake },
            }
        },
        _ => { msg }
    };

    let layer: Layer;
    let keyboard_mode: KeyboardMode;
    let ui_mode: UIMode;

    match msg {
        GuiMessage::Sleep => {
            layer = Layer::Overlay;
            keyboard_mode = KeyboardMode::None;
            ui_mode = UIMode::Clock;
        },
        GuiMessage::Wake => {
            layer = Layer::Overlay;
            keyboard_mode = KeyboardMode::Exclusive;
            ui_mode = UIMode::Launcher;
        },
        GuiMessage::DeepSleep => {
            layer = Layer::Bottom;
            keyboard_mode = KeyboardMode::None;
            ui_mode = UIMode::Clock;
        },
        GuiMessage::WaylandMonitorLayer { ref target_monitor, target_layer } => {
            match target_layer {
                LayerLevel::Bottom => { 
                    layer = Layer::Bottom;
                    ui_mode = UIMode::Clock;
                    keyboard_mode = KeyboardMode::None;
                },
                LayerLevel::Overlay => {
                    layer = Layer::Overlay;
                    ui_mode = state.ui_mode.clone();
                    keyboard_mode = match ui_mode {
                        UIMode::Clock => KeyboardMode::None,
                        UIMode::Launcher => KeyboardMode::Exclusive,
                    };
                },
            }

            info!("Moving shunpo to monitor: {}: {}", &target_monitor, target_layer);
            widgets.window.set_layer(layer);
            if let Ok(monitor) = find_display(target_monitor) {
                widgets.window.set_monitor(Some(&monitor));
            }
        },
        GuiMessage::UpdateWorkspace(_)=> {
            panic!("UI workspace invariatn: GuiMessage::UpdateWorkspace should have been caught earlier.");
        },
        GuiMessage::ToggleUiMode => {
            panic!("UI mode switch invariant: GuiMessage::ToggleUiMode should have been translated.");
        },
        GuiMessage::DisplayResults(_) => {
            panic!("UI mode switch invariant: GuiMessage::DisplayResults");
        },
    };

    widgets.window.set_layer(layer);
    widgets.window.set_keyboard_mode(keyboard_mode);
    state.ui_mode = ui_mode;

    // set focus and clear search state
    match msg {
        GuiMessage::Wake => {
            widgets.search.grab_focus();
            widgets.search.set_text(""); // clear previous search
        },
        _ => {},
    }
}

fn find_display(target_name: &MonitorName) -> Result<Monitor, ShunpoGtk4Error>  {
    let display = gtk4::gdk::Display::default().ok_or(ShunpoGtk4Error::DefaultDisplay)?;
    let monitors = display.monitors();

    monitors
        .iter::<gtk4::gdk::Monitor>()
        .find_map(|m| {
            let monitor = m.ok()?;
            (monitor.connector()? == gtk4::glib::GString::from(target_name.to_string())).then_some(monitor)
        }).ok_or(ShunpoGtk4Error::FindMonitor)
}
