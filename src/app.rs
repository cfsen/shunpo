use std::time::Duration;

use eframe;
use eframe::egui;
use egui::{Color32, Stroke, Vec2};
use log::debug;
use tokio::sync::mpsc;

use crate::coordinator_types::GuiMessage;
use crate::hyprland::hyprctl::is_client_visible;
use crate::state::{ShunpoMode, ShunpoState};
use crate::ui::{ShunpoWidgetClock, ShunpoWidgetSearch, ShunpoWidgetVolume};
use crate::keyboard_input;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Shunpo {
    state: ShunpoState,
    #[serde(skip)]
    event_rx: mpsc::UnboundedReceiver<GuiMessage>, // runtime-only, needs to be set on resume
    #[serde(skip)]
    search_tx: mpsc::UnboundedSender<String>,
}

impl Default for Shunpo {
    fn default() -> Self {
        let (_, event_rx) = mpsc::unbounded_channel(); // dummy to satisfy the requirements for default
        let (search_tx, _) = mpsc::unbounded_channel(); // dummy
        Self {
            state: ShunpoState::default(),
            event_rx,
            search_tx
        }
    }
}
impl Shunpo {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        rx: mpsc::UnboundedReceiver<GuiMessage>,
        tx: mpsc::UnboundedSender<String>,
    ) -> Self {
        let mut app: Shunpo = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // setup redraw timer
        let ctx = cc.egui_ctx.clone();
        tokio::spawn(redraw_timer(ctx));

        // setup caret
        cc.egui_ctx.style_mut(|style| {
            style.visuals.text_cursor.blink = false;
            style.visuals.text_cursor.stroke = Stroke::new(1.0, Color32::WHITE);
        });

        app.event_rx = rx;
        app.search_tx = tx;
        app
    }
}

impl eframe::App for Shunpo {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle hotkeys
        ctx.input(|i| {
            self.state.mode = keyboard_input::handle_input(&i, self.state.mode)
        });

        // TODO: move out
        // log hyprland events
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                GuiMessage::Wake => {
                    info!("GUI received wake!");
                },
                GuiMessage::Sleep => {
                    info!("GUI received sleep!");
                },
                _ => {
                    info!("Unhandled message from coordinator.");
                },
            }
        }

        match self.state.mode {
            ShunpoMode::Clock => {
                clock_ui(ctx, |ui| {
                    ui.add(ShunpoWidgetClock::new());
                });
            }
            ShunpoMode::Launcher => {
                main_launcher_ui(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_sized(Vec2::new(100.0, 100.0), ShunpoWidgetClock::new());
                        ui.add_sized(Vec2::new(600.0, 32.0), ShunpoWidgetSearch::new(&mut self.state));
                        ui.add_sized(Vec2::new(50.0, 100.0), ShunpoWidgetVolume::new(&mut self.state));
                    });

                    ui.label("placeholder search results");
                });
            }
        }
    }
}

fn main_launcher_ui(ctx: &egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::{CentralPanel, UiBuilder};

    let panel_frame = egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128))
        .corner_radius(0);

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect()
            .shrink(4.0);
        let mut content_ui = ui.new_child(UiBuilder::new().max_rect(app_rect));
        add_contents(&mut content_ui);
    });
}

fn clock_ui(ctx: &egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::{CentralPanel, UiBuilder};

    let panel_frame = egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 192))
        .corner_radius(0);

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect()
            .shrink(4.0);
        let mut content_ui = ui.new_child(UiBuilder::new().max_rect(app_rect));
        add_contents(&mut content_ui);
    });
}

async fn redraw_timer(ctx: egui::Context) {
    // request_repaint() must be called in order for the clock to update, when there is no user
    // input. however, requesting redraws if the app is not being rendered (on a hidden workspace)
    // will cause hyprland to raise an application not responding error, terminating the app.
    // polling on a timer to check for app visibility to work around this.
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;
        if is_client_visible("shunpo"){
            ctx.request_repaint();
        }
    }
}
