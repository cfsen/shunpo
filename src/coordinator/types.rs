use crate::search::item_types::Executable;

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
// TODO: TODO_GENERIC RESULTS
// results should be a ready-to-dispatch type, that accepts all data sources
// and includes flags for how the dispatcher should deal with them.
pub struct SearchMessageData {
    pub success: bool,
    pub results: Vec<(u16, Executable)>, // TODO: TODO_GENERIC_RESULTS
}
pub enum FeedbackData {
    Sleep,
}

//
// coordinator outbound messages
//
pub enum GuiMessage {
    Wake,
    Sleep,
    DisplayResults(SearchMessageData),
}

