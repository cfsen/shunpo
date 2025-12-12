use gtk4::prelude::*;
use gtk4_layer_shell::{
    Edge,
    LayerShell,
};

use crate::{
    ui_gtk4::{
        types::{
            ShunpoState,
            ShunpoWidgets,
            UIMode,
        },
    },
};

pub fn toggle_ui_mode(widgets: &ShunpoWidgets, state: &ShunpoState) {
    match state.ui_mode {
        UIMode::Clock => {
            widgets.window.set_anchor(Edge::Top, false);
            widgets.window.set_anchor(Edge::Left, false);
            widgets.window.set_anchor(Edge::Right, true);
            widgets.window.set_anchor(Edge::Bottom, true);
            widgets.window.set_default_size(70,10);

            widgets.volume.set_visible(false);
            widgets.workspaces.set_visible(true);
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
            widgets.workspaces.set_visible(false);
            widgets.search.set_visible(true);
            widgets.results.set_visible(true);
            widgets.results_window.set_visible(true);
        },
    };
}
