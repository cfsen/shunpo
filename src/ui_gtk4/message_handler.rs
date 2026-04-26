use gtk4_layer_shell::LayerShell;
use tokio::sync::mpsc::UnboundedSender;

use crate::coordinator::types::{CoordinatorMessage, GuiMessage};
use crate::ui_gtk4::types::{ShunpoState, ShunpoWidgets, UIMode};
use crate::ui_gtk4::ui_updaters::{
    search_results::update_results,
    workspace_widget::update_active_workspace,
    ui_mode::ui_mode_from_gui_message,
};

pub fn handle_ui_message(
    msg: GuiMessage,
    widgets: &ShunpoWidgets,
    state: &mut ShunpoState,
    feedback_tx: &UnboundedSender<CoordinatorMessage>
) {
    match msg {
        GuiMessage::UpdateWorkspace(workspaces) => {
            update_active_workspace(workspaces, widgets, state);
        },
        GuiMessage::DisplayResults(data) => {
            update_results(data, widgets, state);
        },
        GuiMessage::UiTransitionToLauncher => {
            widgets.window.set_layer(gtk4_layer_shell::Layer::Overlay);
            widgets.window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
            state.ui_mode = UIMode::Launcher;
        },
        GuiMessage::UiTransitionToClock => {
            widgets.window.set_layer(gtk4_layer_shell::Layer::Overlay);
            widgets.window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);
            state.ui_mode = UIMode::Clock;
        },
        _ => { ui_mode_from_gui_message(msg, widgets, state, &feedback_tx); },
    }
}
