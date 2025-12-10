use freedesktop_desktop_entry::Locale;
use log::{debug, error, info};
use std::ffi::OsStr;
use std::{env, fs, path::PathBuf};
use std::os::unix::fs::PermissionsExt;
use nucleo::Utf32String;

use crate::search::entity_model::{Dispatcher, ExecutableEntity, ExecutableSource};

pub fn scan_path_executables() -> Vec<ExecutableEntity> {
    let mut executables = Vec::new();

    let path_env = match env::var("PATH") {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to read PATH: {}", e);
            return executables;
        }
    };

    for path in find_executables_in_path(&path_env) {
        executables.push(ExecutableEntity {
            dispatcher: Dispatcher::Shell,
            exec: name_from_path(&path),
            match_name: searchable_utf32string_from_path(&path),
            match_rank: None,
            ui_name: name_from_path(&path),
            path,

            source: ExecutableSource::PathBinary,
        });
    }
    executables
}

pub fn scan_desktop_executables() -> Vec<ExecutableEntity> {
    let locale = std::env::var("LANG")
        .ok()
        .and_then(|lang| lang.split('.').next().map(String::from))
        .and_then(|lang| lang.parse::<Locale>().ok())
        .unwrap_or_else(|| "en-US".parse().unwrap());
    let locales = vec![locale];  // owned vec

    let entries = freedesktop_desktop_entry::desktop_entries(locales.as_slice());
    let mut executables = Vec::new();

    for entry in entries {
        let ui_name = entry.name(locales.as_slice())
            .map(|cow| cow.into_owned())
            .unwrap_or_else(|| "Unknown".to_string());

        // TODO: other filtering

        let no_display = entry.no_display();
        if no_display {
            debug!("no_display=true for: {}", ui_name);
            continue;
        }

        let Some(exec) = entry.exec().map(|e| e.to_string()) else {
            error!("No exec defined for: {}", ui_name);
            continue;
        };

        let match_name = Utf32String::from(
            entry.name(locales.as_slice())
                .unwrap_or_else(|| std::borrow::Cow::Borrowed("Unknown"))
        );

        let dispatcher = if entry.terminal() { Dispatcher::Shell }
        else { Dispatcher::Hyprctl };

        // TODO: icons?
        // let icon = entry.icon();

        let path = entry.path;  // TODO: rename field: this is the working directory to run program in

        executables.push(ExecutableEntity {
            dispatcher,
            match_name,
            match_rank: None,
            path,
            ui_name,

            source: ExecutableSource::DesktopFile,
            exec,
        });
    }
    executables
}

pub fn scan_script_executables() -> Vec<ExecutableEntity> {
    Vec::new()
}

fn find_executables_in_path(path_env: &str) -> impl Iterator<Item = PathBuf> + '_ {
    path_env
        .split(':')
        .filter_map(|dir| fs::read_dir(dir).ok())
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            fs::metadata(path)
                .map(|m| m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
}
fn name_from_path(path: &PathBuf) -> String {
    path.file_name()
        .unwrap_or_else(|| {
            error!("Failed to extract file_name from: {}", path.to_string_lossy());
            OsStr::new("")
        })
        .to_string_lossy()
        .to_string()
}
fn searchable_utf32string_from_path(path: &PathBuf) -> Utf32String {
    Utf32String::from(
        path.file_name()
            .unwrap_or_else(|| {
                error!("Failed to extract file_name from: {}", path.to_string_lossy());
                OsStr::new("")
            })
            .to_string_lossy()
    )
}
