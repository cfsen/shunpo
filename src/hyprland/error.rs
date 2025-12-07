pub enum HyprError {
    EventParseFailed,

    MonitorFailedGetWorkspace,
    MonitorIdExists,
    MonitorIdNotFound,
    MonitorNoneFocused,

    ParseIntError,

    WorkspaceIdExists,
    WorkspaceIdNotFound,

    NotImplemented,
    Unspecified,
}
impl std::fmt::Display for HyprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HyprError::EventParseFailed => write!(f, "EventParseFailed"),
            HyprError::MonitorFailedGetWorkspace => write!(f, "MonitorFailedGetWorkspace"),
            HyprError::MonitorIdExists => write!(f, "MonitorIdExists"),
            HyprError::MonitorIdNotFound => write!(f, "MonitorIdNotFound"),
            HyprError::MonitorNoneFocused => write!(f, "MonitorNoneFocused"),
            HyprError::ParseIntError => write!(f, "ParseIntError"),
            HyprError::WorkspaceIdExists => write!(f, "WorkspaceIdExists"),
            HyprError::WorkspaceIdNotFound => write!(f, "WorkspaceIdNotFound"),
            HyprError::NotImplemented => write!(f, "NotImplemented"),
            HyprError::Unspecified => write!(f, "Unspecified"),
        }
    }
}
impl From<std::num::ParseIntError> for HyprError {
    fn from(_: std::num::ParseIntError) -> Self {
        HyprError::ParseIntError
    }
}
