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
            let tx = search_tx.clone();
            widgets.search.connect_changed(move |entry| {
                let query = entry.text().to_string();
                if let Err(e) = tx.send(query) {
                    error!("Failed to send search query: {}", e);
                }
            });

            // coordinator to ui
            let state_update = state_rc.clone();
            glib::spawn_future_local(async move {
                while let Ok(msg) = rx.recv().await {
                    {
                        let mut state = state_update.borrow_mut();
                        handle_ui_message(msg, &widgets, &mut state);
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

fn update_ui(widgets: &ShunpoWidgets, state: &ShunpoState) {
    toggle_ui_mode(&widgets, &state);
}
