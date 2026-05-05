use gtk4::{
    Application,

    prelude::*,
    glib::{self, ExitCode},
};
use log::{
    error,
};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

use crate::{
    coordinator::types::{
        CoordinatorMessage,
        GuiMessage,
        FeedbackData,
        SearchMessageData,
    },
    search::entity_model::{
        LauncherEntity,
        VirtualEntity,
    },
    ui_gtk4::{
        builder::build_ui,
        message_handler::handle_ui_message,
        updaters::toggle_ui_mode,
        types::{
            ShunpoState,
            ShunpoWidgets,
        },
    },
};

pub fn run_shunpo(
    gui_rx: async_channel::Receiver<GuiMessage>,
    search_tx: mpsc::UnboundedSender<String>,
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>,
) -> ExitCode {
    let app = Application::builder()
        .application_id("com.shunpo.launcher")
        .build();

    let gui_rx = Rc::new(RefCell::new(Some(gui_rx)));
    let state_rc = Rc::new(RefCell::new(ShunpoState::new()));

    app.connect_activate(move |app| {
        if let Some(rx) = gui_rx.borrow_mut().take() {


            let feedback = feedback_tx.clone();
            let widgets = build_ui(app, feedback, state_rc.clone());

            // initial setup
            let state = state_rc.borrow_mut();
            toggle_ui_mode(&widgets, &state);

            // ui to coordinator
            let ev_search_tx = search_tx.clone();
            let ev_feedback_ui_tx = feedback_tx.clone();
            widgets.search.connect_changed(move |entry| {
                handle_entry_change(entry, &ev_search_tx, &ev_feedback_ui_tx);
            });

            // coordinator to ui
            let feedback_ui_tx = feedback_tx.clone();
            let state_update = state_rc.clone();
            glib::spawn_future_local(async move {
                while let Ok(msg) = rx.recv().await {
                    {
                        let mut state = state_update.borrow_mut();
                        handle_ui_message(msg, &widgets, &mut state, &feedback_ui_tx);
                        update_ui(&widgets, &state); // update ui
                    }
                }
            });
        } else {
            app.active_window().map(|w| w.present());
        }
    });

    app.run()
}

fn handle_entry_change(
    entry: &gtk4::Entry,
    search_tx: &mpsc::UnboundedSender<String>,
    feedback_tx: &mpsc::UnboundedSender<CoordinatorMessage>,
) {
    let query = entry.text().to_string();
    if display_help_as_result(&query, &feedback_tx) {
        return;
    }

    if let Err(e) = search_tx.send(query) {
        error!("Failed to send search query: {}", e);
    }
}

fn display_help_as_result(query: &str, feedback_tx: &mpsc::UnboundedSender<CoordinatorMessage>) -> bool {
    let send_virtual_result = if query == ":q" {
        Some(compose_fb_help_msg(vec!["Quit shunpo".into()]))
    }
    else if query == ":deepsleep" {
        Some(compose_fb_help_msg(vec!["Hide shunpo".into()]))
    }
    else { None };

    if let Some(msg) = send_virtual_result {
        if let Err(e) = feedback_tx.send(msg) {
            error!("Failed to inject virtual entity to feedback: {}", e);
        }
        return true;
    }

    false
}

fn compose_fb_help_msg(text: Vec<String>) -> CoordinatorMessage {
    let results = text.into_iter()
        .map(compose_launcher_entity_from_string)
        .collect();

    CoordinatorMessage::Feedback(
        FeedbackData::GuiMessagePassthrough(
            GuiMessage::DisplayResults(
                SearchMessageData { results }
            )
        )
    )
}

fn compose_launcher_entity_from_string(command: String) -> LauncherEntity {
    LauncherEntity::from_virtual(&VirtualEntity::no_dispatch(command))
}

fn update_ui(widgets: &ShunpoWidgets, state: &ShunpoState) {
    toggle_ui_mode(&widgets, state);
}
