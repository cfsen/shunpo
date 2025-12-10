use gtk4::{
    Application,
    ApplicationWindow,
    Box,
    Orientation,

    prelude::*,
};
use gtk4_layer_shell::{
    Edge,
    Layer,
    LayerShell,
};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

use crate::{
    coordinator::types::CoordinatorMessage,
    ui_gtk4::{
        controllers::{
            search_controller,
            window_controller,
        },
        types::{
            ShunpoState,
            ShunpoWidgets,
        },
        ui_widgets::{
            clock,
            results,
            search,
            volume,
        },
    },
};


pub fn build_ui(
    app: &Application,
    feedback_tx: mpsc::UnboundedSender<CoordinatorMessage>,
    state: Rc<RefCell<ShunpoState>>,
) -> ShunpoWidgets {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(800)
        .default_height(600)
        .title("shunpo")
        .build();

    // shell layer
    window.init_layer_shell();
    window.set_namespace(Some("shunpo"));
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
    launcher_box.append(&results_window);

    // controllers
    let window_controller = window_controller();
    window.add_controller(window_controller);

    let search_controller = search_controller(
        search.clone(),
        results.clone(),
        feedback_tx.clone(),
        state,
    );
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
