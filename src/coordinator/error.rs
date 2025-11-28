use crate::coordinator::types::GuiMessage;

#[derive(Debug)]
pub enum CoordinatorError {
    GuiSendError(async_channel::SendError<GuiMessage>),
    HyprlandError(String),
    FeedbackError(String),
}

impl std::fmt::Display for CoordinatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GuiSendError(e) => write!(f, "GUI send error: {}", e),
            Self::HyprlandError(e) => write!(f, "Hyprland error: {}", e),
            Self::FeedbackError(e) => write!(f, "Feedback error: {}", e),
        }
    }
}

impl std::error::Error for CoordinatorError {}

impl From<async_channel::SendError<GuiMessage>> for CoordinatorError {
    fn from(e: async_channel::SendError<GuiMessage>) -> Self {
        Self::GuiSendError(e)
    }
}
