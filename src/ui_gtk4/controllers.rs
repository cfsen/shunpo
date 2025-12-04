use gtk4::gdk::{
    Key,
    ModifierType
};
use gtk4::{
    Entry,
    EventControllerKey,
    ListBox,

    prelude::*,
};
use log::{
    error,
    info
};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

use crate::{
    coordinator::types::{
        CoordinatorMessage,
        FeedbackData,
    },
    search::entity_model::LauncherEntity,
    ui_gtk4::{
        types::ShunpoState,
        helpers::result_data_from_idx,
    },
};


pub fn window_controller() -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == Key::Escape {
            std::process::exit(0);
        }
        gtk4::glib::Propagation::Proceed
    });

    controller
}

pub fn search_controller(
    search: Entry,
    results: ListBox,
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>,
    state_rc: Rc<RefCell<ShunpoState>>,
) -> EventControllerKey {
    let search_controller = EventControllerKey::new();

    // results navigation
    search_controller.connect_key_pressed({
        let results = results.clone();
        move |_, key, _code, modifier| {
            if modifier.contains(ModifierType::CONTROL_MASK) {
                if key == Key::n || key == Key::p {
                    let cur = if key == Key::n { 1 } else { -1 };

                    if let Some(target_row_idx) = results.row_at_index(
                        results.selected_row().map_or_else(|| 0, |row| row.index()+cur)
                    ){
                        results.select_row(Some(&target_row_idx));
                    }
                    return gtk4::glib::Propagation::Stop;
                }
            }
            gtk4::glib::Propagation::Proceed
        }
    });

    search_controller.connect_key_released({
        move |_, key, _code, modifier| {
            if modifier.contains(ModifierType::CONTROL_MASK) && key == Key::w {
                // TODO: remove until first left whitespace
                search.set_text("");
            }

            if key != Key::Return {
                return;
            }

            let text = search.text();

            if text.is_empty() {
                let _ = feedback_tx.send(CoordinatorMessage::Feedback(
                    FeedbackData::Sleep
                ));
                return;
            }
            else if text == ":q" {
                std::process::exit(0);
            }
            else if text == ":deepsleep" {
                let _ = feedback_tx.send(CoordinatorMessage::Feedback(
                    FeedbackData::DeepSleep
                ));
                return;
            }

            let res: Option<LauncherEntity>;
            {
                let state = state_rc.borrow();
                res = result_data_from_idx(&results, &state);
            }
            if let Some(data) = res {
                info!("Lookup:");
                info!("name: {}", data.alias);
                info!("command: {:?}", data.command);
                info!("dispatcher: {:?}", data.dispatcher);
                let _ = feedback_tx.send(CoordinatorMessage::Feedback(
                    FeedbackData::Run(data)
                ));
            }
            else {
                error!("Failed to match listbox to state!");
            }
        }
    });

    search_controller
}
