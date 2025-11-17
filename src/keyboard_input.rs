use egui::InputState;
use log::{debug,error,info};

use crate::{hyprland::hyprctl, state::ShunpoMode};

pub fn handle_input(i: &InputState, mode: ShunpoMode) -> ShunpoMode {
    // TODO: for testing
    if i.key_pressed(egui::Key::F1) {
        match mode {
            ShunpoMode::Launcher => {
                set_clock_mode();
                return ShunpoMode::Clock
            }
            ShunpoMode::Clock => {
                set_launcher_mode();
                return ShunpoMode::Launcher
            }
        }
    }
    mode
}

// TODO: resizing/moving shunpo should be centralized somewhere with positions calculated
// at app init+on monitor res change.
fn set_clock_mode(){
    info!("Mode: Shunpo to clock mode.");
    if let Err(e) = hyprctl::resize_client_by_initialtitle("shunpo", 100, 100) {
        error!("Failed to move window: {}", e);
    }
    if let Err(e) = hyprctl::move_client_by_initialtitle("shunpo", 0, 1340) {
        error!("Failed to position window: {}", e);
    }
}

fn set_launcher_mode(){
    info!("Mode: Shunpo to launcher mode.");
    if let Err(e) = hyprctl::resize_client_by_initialtitle("shunpo", 800, 600) {
        error!("Failed to move window: {}", e);
    }
    if let Err(e) = hyprctl::move_client_by_initialtitle("shunpo", 880, 420) {
        error!("Failed to position window: {}", e);
    }
}
