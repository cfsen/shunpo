pub enum HyprError {
    EventParseFailed,
    HyprCtlDispatchFailure,
    HyprCtlDispatchTerm,
    HyprCtlExec(String),
    HyprCtlExecDecode(String),
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

    #[deprecated(note = "Replace with a specific error variant")]
    Unspecified,
}
impl std::fmt::Display for HyprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HyprError::EventParseFailed => write!(f, "EventParseFailed"),
            HyprError::HyprCtlDispatchFailure => write!(f, "HyprCtlDispatchFailure"),
            HyprError::HyprCtlDispatchTerm => write!(f, "HyprCtlDispatchTerm"),
            HyprError::HyprCtlExec(e) => write!(f, "HyprCtlExec: {}", e),
            HyprError::HyprCtlExecDecode(e) => write!(f, "HyprCtlExecDecode: {}", e),
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

            #[allow(deprecated)]
            HyprError::Unspecified => write!(f, "Unspecified"),
        }
    }
}
impl From<std::num::ParseIntError> for HyprError {
    fn from(_: std::num::ParseIntError) -> Self {
        HyprError::ParseIntError
    }
}
