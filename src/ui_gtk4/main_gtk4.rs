use gtk4::gdk;
use gtk4::gdk::Key;
use gtk4::gdk::ModifierType;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Entry;
use gtk4::EventControllerKey;
use gtk4::Label;
use gtk4::ListBox;
use gtk4::Orientation;
use gtk4::glib::{self, ExitCode};
use gtk4::prelude::*;
use gtk4_layer_shell::Edge;
use gtk4_layer_shell::Layer;
use gtk4_layer_shell::LayerShell;
use log::error;
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

use crate::coordinator::types::CoordinatorMessage;
use crate::coordinator::types::FeedbackData;
use crate::coordinator::types::GuiMessage;
use crate::ui_gtk4::message_handler::handle_ui_message;
use crate::ui_gtk4::types::ShunpoState;
use crate::ui_gtk4::types::ShunpoWidgets;
use crate::ui_gtk4::types::UIMode;
use crate::ui_gtk4::ui_widgets::clock;
use crate::ui_gtk4::ui_widgets::results;
use crate::ui_gtk4::ui_widgets::search;
use crate::ui_gtk4::ui_widgets::volume;

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
            let widgets = build_ui(app, feedback);

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
                    let mut state = state_update.borrow_mut();
                    handle_ui_message(msg, &widgets, &mut state);
                    update_ui(&widgets, &state); // update ui
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

fn toggle_ui_mode(widgets: &ShunpoWidgets, state: &ShunpoState) {
    match state.ui_mode {
        UIMode::Clock => {
            widgets.window.set_anchor(Edge::Top, false);
            widgets.window.set_anchor(Edge::Left, false);
            widgets.window.set_anchor(Edge::Right, true);
            widgets.window.set_anchor(Edge::Bottom, true);
            widgets.window.set_default_size(76,16);

            widgets.volume.set_visible(false);
            widgets.search.set_visible(false);
            widgets.results.set_visible(false);
            widgets.results_window.set_visible(false);
        },
        UIMode::Launcher => {
            widgets.window.set_anchor(Edge::Top, false);
            widgets.window.set_anchor(Edge::Left, false);
            widgets.window.set_anchor(Edge::Right, false);
            widgets.window.set_anchor(Edge::Bottom, false);
            widgets.window.set_default_size(800,600);

            widgets.volume.set_visible(true);
            widgets.search.set_visible(true);
            widgets.results.set_visible(true);
            widgets.results_window.set_visible(true);
        },
    };
}

pub fn build_ui(
    app: &Application,
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>
) -> ShunpoWidgets {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("shunpo")
        .build();

    // shell layer
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

    // float
    window.set_anchor(Edge::Top, false);
    window.set_anchor(Edge::Left, false);
    window.set_anchor(Edge::Right, false);
    window.set_anchor(Edge::Bottom, false);

    // launcher container
    let launcher_box = Box::new(Orientation::Vertical, 7);
    launcher_box.set_margin_top(7);
    launcher_box.set_margin_bottom(7);
    launcher_box.set_margin_start(7);
    launcher_box.set_margin_end(7);

    let header_box = Box::new(Orientation::Horizontal, 7);
    let clock = clock();
    let volume = volume();
    let search = search();
    let (results_window, results) = results();
    header_box.append(&clock);
    header_box.append(&search);
    header_box.append(&volume);

    launcher_box.append(&header_box);
    // launcher_box.append(&search);
    launcher_box.append(&results_window);

    // controllers
    let window_controller = window_controller();
    window.add_controller(window_controller);

    let search_controller = search_controller(search.clone(), results.clone(), feedback_tx.clone());
    search.add_controller(search_controller);

    window.set_child(Some(&launcher_box));
    window.present();

    ShunpoWidgets {
        window,
        clock,
        volume,
        search,
        results,
        results_window,
    }
}

fn window_controller() -> EventControllerKey {
    let controller = EventControllerKey::new();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == Key::Escape {
            std::process::exit(0);
        }
        gtk4::glib::Propagation::Proceed
    });

    controller
}

fn search_controller(search: Entry, results: ListBox, feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>) -> EventControllerKey {
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
            if key == Key::Return {
                let text = search.text();
                if text.is_empty() {
                    info!("Empty return");
                    let _ = feedback_tx.send(CoordinatorMessage::Feedback(FeedbackData::Sleep));
                } else if text == ":q" {
                    std::process::exit(0);
                } else {
                    info!("Launch currently selected, search term: {}", text);
                }
            }
        }
    });

    search_controller
}
