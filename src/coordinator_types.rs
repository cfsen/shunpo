//
// coordinator inbound messages
//
pub enum CoordinatorMessage {
    HyprlandEvent(HyprlandEventData),
    ShunpoSocketEvent(ShunpoSocketEventData),
    RipgrepResult(RipgrepResultData),
    SearchMessage(SearchMessageData),
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
pub struct SearchMessageData {
    pub success: bool,
    pub results: String,
}

//
// coordinator outbound messages
//
pub enum GuiMessage {
    Wake,
    Sleep,
    DisplayResults(Vec<String>),
}

