use crate::coordinator::types::GuiMessage;
use crate::ui_gtk4::types::{ShunpoState, ShunpoWidgets};
use crate::ui_gtk4::ui_updaters::{
    search_results::update_results,
    workspace_widget::update_active_workspace,
    ui_mode::ui_mode_from_gui_message,
};

pub fn handle_ui_message(msg: GuiMessage, widgets: &ShunpoWidgets, state: &mut ShunpoState) {
    match msg {
        GuiMessage::UpdateWorkspace(workspaces) => {
            update_active_workspace(workspaces, widgets, state);
        },
        GuiMessage::DisplayResults(data) => {
            update_results(data, widgets, state);
        },
        _ => { ui_mode_from_gui_message(msg, widgets, state); },
    }
}
