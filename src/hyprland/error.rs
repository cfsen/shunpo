pub enum HyprError {
    EventParseFailed,
    HyprCtlFetchLayers,
    HyprCtlFetchMonitors,
    HyprCtlFetchWorkspaces,
    MonitorIdNotFound,
    ParseIntError,
    ShunpoInvariantAllFullscreen,
    ShunpoInvariantTargetTopScore,
    ShunpoNotFound,
    ShunpoTargetNoSolution,
    WorkspaceIdNotFound,

    Unspecified,
}
impl std::fmt::Display for HyprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HyprError::EventParseFailed => write!(f, "EventParseFailed"),
            HyprError::HyprCtlFetchLayers => write!(f, "HyprCtlFetchLayers"),
            HyprError::HyprCtlFetchMonitors => write!(f, "HyprCtlFetchMonitors"),
            HyprError::HyprCtlFetchWorkspaces => write!(f, "HyprCtlFetchWorkspaces"),
            HyprError::MonitorIdNotFound => write!(f, "MonitorIdNotFound"),
            HyprError::ParseIntError => write!(f, "ParseIntError"),
            HyprError::ShunpoInvariantAllFullscreen => write!(f, "ShunpoInvariantAllFullscreen"),
            HyprError::ShunpoInvariantTargetTopScore => write!(f, "ShunpoInvariantTopScore"),
            HyprError::ShunpoNotFound => write!(f, "ShunpoNotFound"),
            HyprError::ShunpoTargetNoSolution => write!(f, "ShunpoTargetNoSolution"),
            HyprError::WorkspaceIdNotFound => write!(f, "WorkspaceIdNotFound"),
            HyprError::Unspecified => write!(f, "Unspecified"),
        }
    }
}
impl From<std::num::ParseIntError> for HyprError {
    fn from(_: std::num::ParseIntError) -> Self {
        HyprError::ParseIntError
    }
}
