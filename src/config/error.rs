pub enum ConfigError {
    CreateConfigDir(std::io::Error),
    Deserialization(toml::de::Error),
    Serialization(toml::ser::Error),
    HyprlandError, // TODO: update after refactoring out of anyhow for mod hyprland
    FileRead(std::io::Error),
    FileWrite(std::io::Error),
    NoSupportedTerminal,
    OpenUserDir(std::env::VarError),
    Unspecified,
}
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let prefix = "ConfigError";
        match self {
            Self::CreateConfigDir(e) => { write!(f, "{}: CreateConfigDir: {}", prefix, e) },
            Self::Deserialization(e) => { write!(f, "{}: Deserialization: {}", prefix, e) },
            Self::Serialization(e) => { write!(f, "{}: Serialization: {}", prefix, e) },
            Self::FileRead(e) => { write!(f, "{}: FileRead: {}", prefix, e) },
            Self::HyprlandError => { write!(f, "{}: HyprLandError", prefix) },
            Self::FileWrite(e) => { write!(f, "{}: FileWrite: {}", prefix, e) },
            Self::NoSupportedTerminal=> { write!(f, "{}: NoSupportedTerminal", prefix) },
            Self::OpenUserDir(e) => { write!(f, "{}: OpenUserDir: {}", prefix, e) },
            Self::Unspecified => { write!(f, "{}: Unspecified", prefix) },
        }
    }
}
