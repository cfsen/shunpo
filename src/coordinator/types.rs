use crate::{hyprland::structs::{LayerLevel, MonitorName}, search::entity_model::LauncherEntity};

//
// coordinator inbound messages
//
pub enum CoordinatorMessage {
    HyprlandEvent(HyprlandEventData),
    ShunpoSocketEvent(ShunpoSocketEventData),
    RipgrepResult(RipgrepResultData),
    SearchMessage(SearchMessageData),
    Feedback(FeedbackData),
}
pub struct HyprlandEventData {
    pub gui_msg: GuiMessage,
}
pub enum ShunpoSocketEventData {
    ToggleUiMode,
}
pub struct RipgrepResultData {
    success: bool,
    results: Option<Vec<String>>,
}
pub struct SearchMessageData {
    pub success: bool,
    pub results: Vec<LauncherEntity>,
}
pub enum FeedbackData {
    GuiMessagePassthrough(GuiMessage),
    Run(LauncherEntity),
}

//
// coordinator outbound messages
//
pub enum GuiMessage {
    ToggleUiMode,
    Wake,
    Sleep,
    DeepSleep,
    DisplayResults(SearchMessageData),
    WaylandMonitorLayer { target_monitor: MonitorName, target_layer: LayerLevel },
}

