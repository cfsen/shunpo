pub enum ShunpoSocketError {
    IOError,
    LockHeldNoSocket,
    ParseIntError,
    SocketBind(std::io::Error),
    SocketCreateDir(std::io::Error),
    StreamFlush(std::io::Error),
    StreamOpen(std::io::Error),
    StreamWrite(std::io::Error),
    XdgRuntimeDir(std::env::VarError),
}
impl std::fmt::Display for ShunpoSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ShunpoSocketError::IOError => write!(f, "IOError"),
            ShunpoSocketError::LockHeldNoSocket => write!(f, "Lock held but no socket found. Try running shunpo again."),
            ShunpoSocketError::ParseIntError => write!(f, "ParseIntError"),
            ShunpoSocketError::SocketBind(e) => write!(f, "SocketBind: {}", e.to_string()),
            ShunpoSocketError::SocketCreateDir(e) => write!(f, "SocketCreateDir: {}", e.to_string()),
            ShunpoSocketError::StreamFlush(e) => write!(f, "StreamFlush: {}", e.to_string()),
            ShunpoSocketError::StreamOpen(e) => write!(f, "StreamOpen: {}", e.to_string()),
            ShunpoSocketError::StreamWrite(e) => write!(f, "StreamWrite: {}", e.to_string()),
            ShunpoSocketError::XdgRuntimeDir(e) => write!(f, "XdgRuntimeDir: {}", e.to_string()),
        }
    }
}
impl From<std::num::ParseIntError> for ShunpoSocketError {
    fn from(_: std::num::ParseIntError) -> Self {
        ShunpoSocketError::ParseIntError
    }
}
impl From<std::io::Error> for ShunpoSocketError {
    fn from(_: std::io::Error) -> Self {
        ShunpoSocketError::IOError
    }
}
