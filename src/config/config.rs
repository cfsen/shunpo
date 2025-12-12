use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

use crate::{config::error::ConfigError, hyprland::{hyprctl::get_monitors, structs::MonitorName}};


#[derive(Clone, Serialize, Deserialize)]
pub struct ShunpoConfig {
    monitor_priority: Vec<MonitorName>,
    terminal_path: String,
}

impl ShunpoConfig {
    pub fn load_or_default() -> Result<Self, ConfigError>  {
        Self::load().or_else(|e| {
            error!("Failed to load config: {}", e);
            match e {
                // don't overwrite config on deserialization errors.
                ConfigError::Deserialization(_) => {
                    Err(e)
                },
                _ => {
                    warn!("Auto-generating new config.");
                    Self::auto_default().or_else(|p| {
                        error!("Failed to regenerate config: {}", p);
                        Err(p)
                    })
                },
            }
        })
    }
    fn load() -> Result<Self, ConfigError> {
        Self::load_config()
    }
    fn auto_default() -> Result<Self, ConfigError> {
        let terminal_path = Self::collect_terminals()?;
        let monitor_priority = Self::collect_monitors()?;

        let config = ShunpoConfig {
            monitor_priority,
            terminal_path,
        };

        config.save_config()?;

        Ok(config)
    }
}
//
// collections
//
impl ShunpoConfig {
    fn collect_monitors() -> Result<Vec<MonitorName>, ConfigError> {
        Ok(get_monitors()
            .map_err(|_| ConfigError::HyprlandError)?
            .into_iter()
            .map(|m| m.name)
            .collect::<Vec<MonitorName>>())
    }

    fn collect_terminals() -> Result<String, ConfigError> {
        let candidates = [
            "/usr/bin/ghostty",
            "/usr/bin/kitty",
            "/usr/bin/alacritty",
        ];

        candidates
            .iter()
            .find(|path| std::path::Path::new(path).exists())
            .map(|s| s.to_string())
            .ok_or(ConfigError::NoSupportedTerminal)
    }
}
//
// io
//
impl ShunpoConfig {
    fn config_path() -> Result<PathBuf, ConfigError> {
        let home = std::env::var("HOME")
            .map_err(|e| ConfigError::OpenUserDir(e))?;

        Ok(PathBuf::from(home)
            .join(".config")
            .join("shunpo")
            .join("config.toml"))
    }

    fn load_config() -> Result<ShunpoConfig, ConfigError> {
        Self::config_path()
            .and_then(|path| {
                fs::read_to_string(&path)
                    .map_err(|e| ConfigError::FileRead(e))
            })
            .and_then(|contents| {
                toml::from_str::<ShunpoConfig>(&contents)
                    .map_err(|e| ConfigError::Deserialization(e))
            })
    }

    fn save_config(&self) -> Result<(), ConfigError> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::CreateConfigDir(e))?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialization(e))?;

        fs::write(&path, contents)
            .map_err(|e| ConfigError::FileWrite(e))
    }
}
