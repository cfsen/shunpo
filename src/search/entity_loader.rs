use log::{debug, error};
use std::ffi::OsStr;
use std::{env, fs, path::PathBuf};
use std::os::unix::fs::PermissionsExt;
use nucleo::Utf32String;

use crate::search::entity_model::{Dispatcher, ExecutableEntity, ExecutableSource};
use crate::search::error::EntityError;

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

pub fn scan_desktop_executables(extra: Vec<PathBuf>) -> Vec<ExecutableEntity> {
    let locales = freedesktop_desktop_entry::get_languages_from_env();

    let desktop_paths = freedesktop_desktop_entry::default_paths()
        .chain(extra.into_iter());

    let entries = freedesktop_desktop_entry::Iter::new(desktop_paths)
        .entries(Some(&locales))
        .collect::<Vec<_>>();

    let mut executables = Vec::new();

    for entry in entries {
        let ui_name = entry.name(locales.as_slice())
            .map(|cow| cow.into_owned())
            .unwrap_or_else(|| "Unknown".to_string());


        // TODO: read filters from config
        if let Some(categories) = entry.categories() {
            if categories.iter().find(|x| x.contains("X-LSP-Plugins")).is_some() {
                continue;
            }
        }

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

pub fn scan_script_executables(config_paths: &Vec<PathBuf>) -> Vec<ExecutableEntity> {
    let mut all_scripts = Vec::<ExecutableEntity>::new();
    for dir_path in config_paths {
        if let Ok(scripts) = find_scripts_in_path(&dir_path) {
            for (path, ui_name) in scripts {
                all_scripts.push(
                    ExecutableEntity {
                        dispatcher: Dispatcher::Shell,
                        match_name: Utf32String::from(ui_name.clone()),
                        match_rank: None,
                        path,
                        ui_name,

                        source: ExecutableSource::ShellScript,
                        exec: "".to_string(),
                    }
                );
            }
        }
    }
    all_scripts
}

fn find_scripts_in_path(dir: &PathBuf) -> Result<Vec<(PathBuf, String)>, EntityError> {
    let paths = std::fs::read_dir(dir)
        .map_err(|_| EntityError::WIP)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| (dir_entry.path(), dir_entry.file_name()))
        .filter_map(|(path, file)| {
            if path.extension().map_or(false, |ext| ext == "sh") {
                Some((path, file.to_string_lossy().into_owned()))
            }
            else {
                None
            }
        })
        .collect::<Vec<(_, _)>>();
    Ok(paths)
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
