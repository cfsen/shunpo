use gtk4::gdk::{Key, ModifierType};
use gtk4::glib::Propagation;
use gtk4::{Entry, EventControllerKey, EventSequenceState, GestureClick, ListBox, PropagationPhase, prelude::*};
use log::error;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

use crate::coordinator::types::GuiMessage;
use crate::{
    coordinator::types::{CoordinatorMessage, FeedbackData},
    search::entity_model::LauncherEntity,
    ui_gtk4::{helpers::result_data_from_idx, types::ShunpoState},
};

/// Disable clicking search results
pub fn click_sink(search: Entry) -> GestureClick {
    let gesture = GestureClick::new();
    gesture.set_propagation_phase(PropagationPhase::Capture);
    gesture.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(EventSequenceState::Claimed);
        search.grab_focus();
        search.set_position(-1);
    });
    gesture
}

/// Event handler for application
pub fn window_controller(
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>,
) -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, key, _, _| {
        // hide or quit on escape
        if key == Key::Escape {
            // TODO: allow exit behavior with config
            // std::process::exit(0);
            let _ = feedback_tx.send(CoordinatorMessage::Feedback(
                FeedbackData::GuiMessagePassthrough(GuiMessage::Sleep),
            ));
        }
        // prevent focus loss of search field
        if key == Key::Up || key == Key::Down || key == Key::Tab {
            return gtk4::glib::Propagation::Stop;
        }
        gtk4::glib::Propagation::Proceed
    });

    controller
}

/// Event handler for GUI text input field
pub fn search_controller(
    search: Entry,
    results: ListBox,
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>,
    state_rc: Rc<RefCell<ShunpoState>>,
) -> EventControllerKey {
    let search_controller = EventControllerKey::new();

    search_controller.connect_key_pressed(
        handle_keyboard_input_pressed(search.clone(), results.clone())
    );

    search_controller.connect_key_released(
        handle_keyboard_input_released(results.clone(), search.clone(), feedback_tx.clone(), state_rc.clone())
    );

    search_controller
}

/// Called when return key is pressed
fn send_input(
    search: &Entry,
    results: &ListBox,
    feedback_tx: &mpsc::UnboundedSender<CoordinatorMessage>,
    state_rc: &Rc<RefCell<ShunpoState>>,
) {
    let text = search.text();

    if text.is_empty() {
        let _ = feedback_tx.send(CoordinatorMessage::Feedback(
            FeedbackData::GuiMessagePassthrough(GuiMessage::Sleep),
        ));
        return;
    }
    else if text == ":q" {
        std::process::exit(0);
    }
    else if text == ":deepsleep" {
        let _ = feedback_tx.send(CoordinatorMessage::Feedback(
            FeedbackData::GuiMessagePassthrough(GuiMessage::DeepSleep),
        ));
        return;
    }

    let res: Option<LauncherEntity>;
    {
        let state = state_rc.borrow();
        res = result_data_from_idx(&results, &state);
    }

    if let Some(data) = res {
        let _ = feedback_tx.send(CoordinatorMessage::Feedback(FeedbackData::Run(data)));
    }
    else {
        error!("Failed to match listbox to state!");
    }
}

/// Routing for keyboard input to controllers on key release
fn handle_keyboard_input_released(
    results: ListBox,
    search: Entry,
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>,
    state_rc: Rc<RefCell<ShunpoState>>,
) -> impl Fn(&EventControllerKey, Key, u32, ModifierType) {
    move |_, key, _code, modifier| {
        fn mod_ctrl(m: ModifierType) -> bool { m.contains(ModifierType::CONTROL_MASK) }
        fn mod_alt(m: ModifierType) -> bool { m.contains(ModifierType::ALT_MASK) }

        if key == Key::Return {
            send_input(&search, &results, &feedback_tx, &state_rc);
        }
        else if mod_ctrl(modifier) && key == Key::a {
            hkb_caret_to_beginning(&search);
        }
        else if mod_alt(modifier) && key == Key::b {
            hkb_caret_word_back(&search);
        }
        else if mod_alt(modifier) && key == Key::f {
            hkb_caret_word_forward(&search);
        }
    }
}

/// Routing for keyboard input to controllers on key press
fn handle_keyboard_input_pressed(
    search: Entry,
    results: ListBox,
) -> impl Fn(&EventControllerKey, Key, u32, ModifierType) -> Propagation {
    move |_, key, _code, modifier| {
        fn mod_ctrl(m: ModifierType) -> bool { m.contains(ModifierType::CONTROL_MASK) }

        if mod_ctrl(modifier) && (key == Key::n || key == Key::p) {
            hkb_nav_results(key, &results)
        }
        else if mod_ctrl(modifier) && key == Key::w {
            hkb_edit_delete_word(&search)
        }
        else if mod_ctrl(modifier) && key == Key::e {
            hkb_caret_to_end(&search)
        }
        else {
            Propagation::Proceed
        }
    }
}

/// Move up and down results list with c^p, c^n
fn hkb_nav_results(key: Key, results: &ListBox) -> Propagation {
    let cur = if key == Key::n { 1 } else { -1 };

    if let Some(target_row_idx) = results.row_at_index(
        results
            .selected_row()
            .map_or_else(|| 0, |row| row.index() + cur),
    ) {
        results.select_row(Some(&target_row_idx));
    }

    gtk4::glib::Propagation::Stop
}

/// Delete word with c^w
fn hkb_edit_delete_word(search: &Entry) -> Propagation {
    match search.text().rsplit_once(" ") {
        Some((head, _tail)) => {
            search.set_text(head);
            search.set_position(-1);
        }
        None => {
            search.set_text("");
        }
    }

    Propagation::Proceed
}

/// Move caret to end of input with c^e
fn hkb_caret_to_end(search: &Entry) -> Propagation {
    search.set_position(-1);

    Propagation::Proceed
}

/// Move caret to beginning of input with c^a
fn hkb_caret_to_beginning(search: &Entry) {
    search.set_position(0);
}

/// Move caret back one word
fn hkb_caret_word_back(search: &Entry) {
    let caret = search.position() as usize;
    let term: String = search.text().chars().take(caret).collect();
    let position = match term.rfind(" ") {
        Some(p) => p,
        None => caret,
    } as i32;
    search.set_position(position);
}

/// Move caret forward one word
fn hkb_caret_word_forward(search: &Entry) {
    let caret = search.position() as usize;
    let term: String = search.text().chars().skip(caret).collect();
    let position = match term.find(" ") {
        Some(p) => caret + p + 1,
        None => caret,
    } as i32;
    search.set_position(position);
}
