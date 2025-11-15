use std::time::Duration;

use eframe;
use eframe::egui;
use log::debug;
use log::info;
use tokio::sync::mpsc;

use crate::hyprland::hyprctl::is_client_visible;
use crate::state::ShunpoState;
use crate::ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Shunpo {
    state: ShunpoState,
    #[serde(skip)]
    event_rx: mpsc::UnboundedReceiver<String>, // runtime-only, needs to be set on resume
}

impl Default for Shunpo {
    fn default() -> Self {
        let (_tx, event_rx) = mpsc::unbounded_channel(); // dummy to satisfy the requirements for default
        Self {
            state: ShunpoState::default(),
            event_rx
        }
    }
}
impl Shunpo {
    pub fn new(cc: &eframe::CreationContext<'_>, rx: mpsc::UnboundedReceiver<String>) -> Self {
        let mut app: Shunpo = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // setup redraw timer
        let ctx = cc.egui_ctx.clone();
        tokio::spawn(redraw_timer(ctx));

        app.event_rx = rx;
        app
    }
}

impl eframe::App for Shunpo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // clock
            ui::draw_clock(ui);
            ui.separator();

            // volume
            ui::draw_volume_slider(ui, &mut self.state);
            ui.separator();

            // search
            ui::draw_search(ui, &mut self.state);

            // log hyprland events
            while let Ok(event) = self.event_rx.try_recv() {
                if let Some((event_type, data)) = event.split_once(">>") {
                    match event_type {
                        "workspace" => debug!("Workspace changed: {}", data),
                        "activewindow" => debug!("Active window: {}", data),
                        _ => {}
                    }
                }
            }
        });
    }
}

async fn redraw_timer(ctx: egui::Context) {
    // request_repaint() must be called in order for the clock to update, when there is no user
    // input. however, requesting redraws if the app is not being rendered (on a hidden workspace)
    // will cause hyprland to raise an application not responding error, terminating the app.
    // polling on a timer to check for app visibility to work around this.
    //
    // TODO: will still trigger ANR if another client is in fullscreen over it
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;
        if is_client_visible("shunpo"){
            info!("request repaint");
            ctx.request_repaint();
        }
    }
}
