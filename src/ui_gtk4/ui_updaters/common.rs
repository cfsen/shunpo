use gtk4::prelude::{BoxExt, WidgetExt};

use crate::ui_gtk4::types::ShunpoWidgets;

pub fn clear_workspaces(widgets: &ShunpoWidgets) {
    while let Some(child) = widgets.workspaces.first_child() {
        widgets.workspaces.remove(&child);
    }
}

pub fn clear_results(widgets: &ShunpoWidgets) {
    while let Some(child) = widgets.results.first_child() {
        widgets.results.remove(&child);
    }
}
