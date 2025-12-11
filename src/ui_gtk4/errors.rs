pub enum ShunpoGtk4Error {
    DefaultDisplay,
    FindMonitor,
}
impl std::fmt::Display for ShunpoGtk4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DefaultDisplay => write!(f, "DefaultDisplay"),
            Self::FindMonitor => write!(f, "FindMonitor"),
        }
    }
}
