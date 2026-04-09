use gtk4::{Label, Orientation, prelude::{BoxExt, WidgetExt}};

use crate::{coordinator::types::WorkspaceMessage, ui_gtk4::{types::{ShunpoState, ShunpoWidgets}, ui_updaters::common::clear_workspaces}};

pub fn update_active_workspace(
    workspaces: Vec<WorkspaceMessage>,
    widgets: &ShunpoWidgets,
    state: &mut ShunpoState,
) {
    clear_workspaces(widgets);

    // update state
    state.workspaces_data = workspaces.clone();

    // populate new workspace indicators
    for workspace in workspaces {
        let ws_box = gtk4::Box::new(Orientation::Horizontal, 0);
        ws_box.add_css_class(if workspace.focused { "ws-active-bg" } else { "ws-inactive-bg" });

        let label = Label::new(Some(&workspace.id));
        label.add_css_class("ws-label");
        ws_box.append(&label);

        widgets.workspaces.append(&ws_box);
    }
}
