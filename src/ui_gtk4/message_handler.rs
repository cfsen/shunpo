use gtk4::prelude::*;
use gtk4::{
    Label, Box, Orientation, 
    ListBoxRow
};
use log::info;

use crate::coordinator::types::GuiMessage;
use crate::ui_gtk4::types::{ShunpoState, ShunpoWidgets, UIMode};

pub fn handle_ui_message(msg: GuiMessage, widgets: &ShunpoWidgets, state: &mut ShunpoState) {
    match msg {
        GuiMessage::Sleep => {
            info!("Sleep message received by UI event handler.");
            state.ui_mode = UIMode::Clock;
        },
        GuiMessage::Wake => {
            info!("Wake message received by UI event handler.");
            state.ui_mode = UIMode::Launcher;
            widgets.window.present();
            widgets.search.grab_focus();
            widgets.search.set_text(""); // clear previous search
        }
        GuiMessage::DisplayResults(data) => {
            // clear existing results
            while let Some(child) = widgets.results.first_child() {
                widgets.results.remove(&child);
            }

            // populate new results
            for (_score, exe) in data.results {
                let row = ListBoxRow::new();
                let box_ = Box::new(Orientation::Horizontal, 10);
                box_.set_margin_top(5);
                box_.set_margin_bottom(5);
                box_.set_margin_start(10);

                let label = Label::new(Some(&exe.name.to_string()));
                box_.append(&label);

                row.set_child(Some(&box_));
                widgets.results.append(&row);
            }
        }
    }
}
