use eframe;
use eframe::egui;
use chrono::Local;
use log::info;

use crate::state::ShunpoState;
use crate::system;

pub fn draw_clock(ui: &mut egui::Ui){
    ui.heading(
        Local::now()
            .format("%H:%M:%S").to_string()
    );
}
pub fn draw_volume_slider(ui: &mut egui::Ui, state: &mut ShunpoState){
    if ui.add(
        egui::Slider::new(&mut state.volume, 0..=100)
            .orientation(egui::SliderOrientation::Vertical)
    ).changed() {
        let _ = system::volume::set_volume(state.volume);
    }
}
pub fn draw_hypr_workspaces(){

}
pub fn draw_search(ui: &mut egui::Ui, state: &mut ShunpoState){
    let resp = ui.text_edit_singleline(&mut state.search);
    if resp.has_focus() {
        if ui.input_mut(|i| i.consume_shortcut(&state.key_shortcut_next)) {
            info!("Keyboard next");
        }
        if ui.input_mut(|i| i.consume_shortcut(&state.key_shortcut_prev)) {
            info!("Keyboard prev");
        }
    }
}
