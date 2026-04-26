use gtk4::gdk::Monitor;
use gtk4::prelude::*;
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use log::info;
use tokio::sync::mpsc::UnboundedSender;

use crate::coordinator::types::{CoordinatorMessage, FeedbackData, GuiMessage};
use crate::hyprland::structs::{LayerLevel, MonitorName};
use crate::system;
use crate::ui_gtk4::errors::ShunpoGtk4Error;
use crate::ui_gtk4::types::{ShunpoState, ShunpoWidgets, UIMode};

pub fn ui_mode_from_gui_message(
    msg: GuiMessage,
    widgets: &ShunpoWidgets,
    state: &mut ShunpoState,
    feedback_tx: &UnboundedSender<CoordinatorMessage>
) {
    // shadow msg on toggle messages
    let msg = match msg {
        GuiMessage::ToggleUiMode => { 
            match state.ui_mode {
                UIMode::Launcher => GuiMessage::Sleep,
                UIMode::Clock => GuiMessage::Wake,
                _ => GuiMessage::Wake,
            }
        },
        _ => { msg }
    };

    let layer: Layer;
    let keyboard_mode: KeyboardMode;
    let ui_mode: UIMode;

    match msg {
        GuiMessage::Sleep => {
            layer = Layer::Overlay;
            keyboard_mode = KeyboardMode::None;
            if !state.ui_transition {
                // TODO: set timing from config
                let fb_tx = feedback_tx.clone();
                gtk4::glib::timeout_add_once(std::time::Duration::from_millis(200), move || {
                    let _ = fb_tx.send(
                        CoordinatorMessage::Feedback(
                            FeedbackData::GuiMessagePassthrough(GuiMessage::UiTransitionToClock)
                        )
                    );
                });
            }
            ui_mode = UIMode::ToClock;
        },
        GuiMessage::Wake => {
            layer = Layer::Overlay;
            keyboard_mode = KeyboardMode::Exclusive;

            if !state.ui_transition {
                // TODO: set timing from config
                let fb_tx = feedback_tx.clone();
                gtk4::glib::timeout_add_once(std::time::Duration::from_millis(200), move || {
                    let _ = fb_tx.send(
                        CoordinatorMessage::Feedback(
                            FeedbackData::GuiMessagePassthrough(GuiMessage::UiTransitionToLauncher)
                        )
                    );
                });
            }
            ui_mode = UIMode::ToLauncher;
        },
        GuiMessage::DeepSleep => {
            layer = Layer::Bottom;
            keyboard_mode = KeyboardMode::None;
            ui_mode = UIMode::Clock;
        },
        GuiMessage::WaylandMonitorLayer { ref target_monitor, target_layer } => {
            match target_layer {
                LayerLevel::Bottom => { 
                    layer = Layer::Bottom;
                    ui_mode = UIMode::Clock;
                    keyboard_mode = KeyboardMode::None;
                },
                LayerLevel::Overlay => {
                    layer = Layer::Overlay;
                    ui_mode = state.ui_mode.clone();
                    keyboard_mode = match ui_mode {
                        UIMode::Launcher => KeyboardMode::Exclusive,
                        _ => KeyboardMode::None,
                    };
                },
            }

            info!("Moving shunpo to monitor: {}: {}", &target_monitor, target_layer);
            widgets.window.set_layer(layer);
            if let Ok(monitor) = find_display(target_monitor) {
                widgets.window.set_monitor(Some(&monitor));
            }
        },
        GuiMessage::UiTransitionToClock => {
            panic!("UITransitionToClock invariant: GuiMessage should have been caught earlier.");
        },
        GuiMessage::UiTransitionToLauncher => {
            panic!("UiTransitionToLauncher invariant: GuiMessage should have been caught earlier.");
        },
        GuiMessage::UpdateWorkspace(_)=> {
            panic!("UI workspace invariant: GuiMessage::UpdateWorkspace should have been caught earlier.");
        },
        GuiMessage::ToggleUiMode => {
            panic!("UI mode switch invariant: GuiMessage::ToggleUiMode should have been translated.");
        },
        GuiMessage::DisplayResults(_) => {
            panic!("UI mode switch invariant: GuiMessage::DisplayResults");
        },
    };

    widgets.window.set_layer(layer);
    widgets.window.set_keyboard_mode(keyboard_mode);
    state.ui_mode = ui_mode;

    // set focus and clear search state
    match msg {
        GuiMessage::Wake => {
            widgets.search.grab_focus();
            widgets.search.set_text(""); // clear previous search
            if let Ok(vol) = system::volume::get_volume() { // update volume controller
                widgets.volume.set_value(vol.into());
            }
        },
        _ => {},
    }
}

fn find_display(target_name: &MonitorName) -> Result<Monitor, ShunpoGtk4Error>  {
    let display = gtk4::gdk::Display::default().ok_or(ShunpoGtk4Error::DefaultDisplay)?;
    let monitors = display.monitors();

    monitors
        .iter::<gtk4::gdk::Monitor>()
        .find_map(|m| {
            let monitor = m.ok()?;
            (monitor.connector()? == gtk4::glib::GString::from(target_name.to_string())).then_some(monitor)
        }).ok_or(ShunpoGtk4Error::FindMonitor)
}
