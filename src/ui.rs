use eframe;
use eframe::egui;
use chrono::Local;
use egui::{Color32, FontFamily, FontId, Id, Response, TextEdit, Ui, Widget};
use log::info;

use crate::search::item_types::Executable;
use crate::state::ShunpoState;
use crate::system;

// clock
pub struct ShunpoWidgetClock;
impl ShunpoWidgetClock {
    pub fn new() -> Self { Self }
}
impl Widget for ShunpoWidgetClock {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.style_mut().override_font_id = Some(egui::FontId::new(
            24.0,
            egui::FontFamily::Proportional,
        ));

        let response = ui.heading(
            Local::now()
                .format("%H:%M:%S").to_string()
        );

        ui.style_mut().override_font_id = None;
        response
    }
}

// volume
pub struct ShunpoWidgetVolume<'a> {
    state: &'a mut ShunpoState, // mutable reference to app state
}
impl<'a> ShunpoWidgetVolume<'a> {
    pub fn new(state: &'a mut ShunpoState) -> Self {
        Self { state }
    }
}
impl<'a> Widget for ShunpoWidgetVolume<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let resp = ui.add(
            eframe::egui::Slider::new(&mut self.state.volume, 0..=100)
                .orientation(egui::SliderOrientation::Vertical),
        );

        if resp.changed() {
            let _ = system::volume::set_volume(self.state.volume);
        }

        resp
    }
}

// search
pub struct ShunpoWidgetSearch<'a> {
    state: &'a mut ShunpoState,
}
impl<'a> ShunpoWidgetSearch<'a> {
    pub fn new(state: &'a mut ShunpoState) -> Self {
        Self { state }
    }
}
impl<'a> Widget for ShunpoWidgetSearch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let wid = ui.make_persistent_id("shunpo_search");

        let resp = ui.add(
            TextEdit::singleline(&mut self.state.search)
                .id(wid)
                .frame(false)
                .font(FontId::new(24.0, FontFamily::Monospace))
                .text_color(Color32::WHITE)
        );

        if resp.has_focus() {
            if ui.input_mut(|i| i.consume_shortcut(&self.state.key_shortcut_next)) {
                log::info!("Keyboard next");
            }
            if ui.input_mut(|i| i.consume_shortcut(&self.state.key_shortcut_prev)) {
                log::info!("Keyboard prev");
            }
        }

        if resp.changed() {
            self.state.send_search = true;
        }

        if !ui.memory(|m| m.has_focus(wid)) {
            ui.memory_mut(|m| m.request_focus(wid));
        }

        resp
    }
}

// search result
pub struct ShunpoWidgetSearchResult<'a> {
    search_results: &'a Vec<(u16, Executable)>,
}

impl<'a> ShunpoWidgetSearchResult<'a> {
    pub fn new(search_results: &'a Vec<(u16, Executable)>) -> Self {
        Self { search_results }
    }
}

impl<'a> Widget for ShunpoWidgetSearchResult<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.style_mut().override_font_id = Some(egui::FontId::new(
            24.0,
            egui::FontFamily::Proportional,
        ));

        let resp = ui.vertical(|ui| {
            for (_, res) in self.search_results {
                ui.label(&res.name.to_string());
            }
        });

        ui.style_mut().override_font_id = None;

        resp.response
    }
}
