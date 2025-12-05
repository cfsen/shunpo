use gtk4::prelude::*;
use gtk4::{
    Label, Box, Orientation, 
    ListBoxRow
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use crate::coordinator::types::GuiMessage;
use crate::ui_gtk4::types::{ShunpoState, ShunpoWidgets, UIMode};

pub fn handle_ui_message(msg: GuiMessage, widgets: &ShunpoWidgets, state: &mut ShunpoState) {
    match msg {
        GuiMessage::DisplayResults(data) => {
            // clear existing results
            while let Some(child) = widgets.results.first_child() {
                widgets.results.remove(&child);
            }

            // update state
            state.results_data = data.results.clone();

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
