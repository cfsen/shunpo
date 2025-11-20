use log::error;
use std::ffi::OsStr;
use std::{env, fs, path::PathBuf};
use std::os::unix::fs::PermissionsExt;
use nucleo::Utf32String;

use crate::search::item_types::{
    Executable,
    Dispatcher,
    ShellScript,
    DocumentDirectory,
};

pub fn populate_binaries() -> Vec<Executable> {
    let mut executables = Vec::new();

    let path_env = match env::var("PATH") {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to read PATH: {}", e);
            return executables;
        }
    };

    for path in find_executables_in_path(&path_env) {
        executables.push(Executable {
            name: searchable_utf32string_from_path(&path),
            path,
            dispatch_with: Dispatcher::Shell,
        });
    }
    executables
}
pub fn populate_scripts() -> Vec<ShellScript> {
    // TODO: populate list of shell scripts

    Vec::new()
}
pub fn populate_documents() -> Vec<DocumentDirectory> {
    // TODO: popluate list of directories for ripgrep

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
