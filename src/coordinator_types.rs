//
// coordinator inbound messages
//
pub enum CoordinatorMessage {
    HyprlandEvent(HyprlandEventData),
    ShunpoSocketEvent(ShunpoSocketEventData),
    RipgrepResult(RipgrepResultData),
}
pub struct HyprlandEventData {
    pub raw_event: String,
}
pub enum ShunpoSocketEventData {
    Wake,
    Sleep,
}
pub struct RipgrepResultData {
    success: bool,
    results: Option<Vec<String>>,
}

//
// coordinator outbound messages
//
pub enum GuiMessage {
    Wake,
    Sleep,
    DisplayResults(Vec<String>),
}

