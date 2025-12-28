use chrono::Local;
use gtk4::prelude::*;
use gtk4::{
    Label, Orientation, 
    Entry, Scale, Adjustment, ListBox, ScrolledWindow, Align,
};
use gtk4::glib;
use log::error;

use crate::system;

pub fn volume() -> Scale {
    let volume_adj = Adjustment::new(50.0, 0.0, 100.0, 1.0, 10.0, 0.0);
    let volume_scale = Scale::new(Orientation::Horizontal, Some(&volume_adj));
    volume_scale.set_size_request(200, 20);
    volume_scale.set_draw_value(false);
    volume_scale.has_origin();

    if let Ok(vol) = system::volume::get_volume() {
        volume_adj.set_value(vol as f64);
    }

    // Connect volume change
    volume_scale.connect_value_changed(|scale| {
        let val = scale.value() as i32;
        if let Err(e) = system::volume::set_volume(val) {
            error!("Failed to set volume: {}", e);
        }
    });

    volume_scale
}

pub fn clock() -> Label {
    fn time() -> String {
        Local::now().format("%H:%M").to_string()
    }

    let clock_label = Label::new(Some(&time()));
    clock_label.set_halign(Align::Start);
    clock_label.add_css_class("clock-label");

    glib::timeout_add_seconds_local(1, {
        let clock_label = clock_label.clone();
        move || {
            clock_label.set_text(&time());
            glib::ControlFlow::Continue
        }
    });

    clock_label
}
pub fn workspaces() -> gtk4::Box {
    let container = gtk4::Box::new(Orientation::Horizontal, 0);
    container.add_css_class("workspaces");

    // TODO: fetch on startup
    let workspaces = vec![
        (1, false),
        (2, true),  // active
        (3, false),
    ];

    for (num, is_active) in workspaces {
        let ws_box = gtk4::Box::new(Orientation::Horizontal, 0);

        if is_active {
            ws_box.add_css_class("ws-active-bg");
        }
        else {
            ws_box.add_css_class("ws-inactive-bg");
        }

        let label = Label::new(Some(&num.to_string()));
        label.add_css_class("ws-label");
        ws_box.append(&label);

        container.append(&ws_box);
    }

    container
}
pub fn search() -> Entry {
    let search_entry = Entry::new();
    search_entry.set_hexpand(true);
    search_entry.set_placeholder_text(Some(""));
    search_entry
}

pub fn results() -> (ScrolledWindow, ListBox) {
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_min_content_height(300);

    let results_list = ListBox::new();
    results_list.set_selection_mode(gtk4::SelectionMode::Single);
    // TODO: styling
    scrolled_window.set_child(Some(&results_list));

    (scrolled_window, results_list)
}
