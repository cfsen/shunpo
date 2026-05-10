use std::{ffi::OsString, path::PathBuf};

use hw_rs::hyprwire::{
    hyprpaper,
    error::HyprwireError,
};

use crate::config::config::ShunpoConfig;

pub fn set_random_wallpaper(config: &ShunpoConfig) {
    for monitor in &config.monitor_priority {
        let Some(wall_path) = config.wallpaper_paths.get(random_index(config.wallpaper_paths.len())) 
        else { continue };

        let Ok(wallpapers) = get_wallpapers_in_path(&PathBuf::from(wall_path))
        else { continue };

        let Some(wall) = wallpapers.get(random_index(wallpapers.len()))
        else { continue };

        let Some(wall_str) = wall.path.to_str()
        else { continue };

        if let Err(set_err) = set_wallpaper(wall_str, &monitor.to_string()) {
            log::error!("Error while setting random wallpaper: {}", set_err);
        }
    }
}

fn random_index(high: usize) -> usize {
    gtk4::glib::random_int_range(0, high as i32) as usize
}

fn get_wallpapers_in_path(dir: &PathBuf) -> Result<Vec<Wallpaper>, PaperClientError> {
    let path = std::fs::read_dir(dir)
        .map_err(|_| PaperClientError::WIP)?
        .filter_map(|r| r.ok())
        .map(|wallpaper| (wallpaper.path(), wallpaper.file_name(), wallpaper.file_type()))
        .filter_map(|(path, file_name, file_type)| add_validated(path, file_name, file_type))
        .collect::<Vec<Wallpaper>>();
    Ok(path)
}

fn add_validated(
    path: PathBuf,
    file_name: OsString,
    file_type: Result<std::fs::FileType, std::io::Error>,
) -> Option<Wallpaper> {
    if !check_is_file(&file_type) {
        return None;
    }
    if !check_img_ext(&path) {
        return None;
    }
    Some(Wallpaper {
        path,
        _file_name: file_name.to_string_lossy().into_owned(),
    })
}

fn check_is_file(file_type: &Result<std::fs::FileType, std::io::Error>) -> bool {
    if let Ok(ftype) = file_type {
        return ftype.is_file();
    }
    false
}

const SHUNPO_VALID_WALLPAPER_EXT: [&str; 2] = ["png", "jpg"];

fn check_img_ext(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        for valid_ext in SHUNPO_VALID_WALLPAPER_EXT {
            if ext == valid_ext {
                return true;
            }
        }
    }
    false
}

pub fn set_wallpaper(path: &str, monitor_id: &str) -> Result<(), HyprwireError> {
    hyprpaper::HyprpaperIPC::set_wallpaper(path, monitor_id)
}

pub struct Wallpaper {
    path: PathBuf,
    _file_name: String,
}

pub enum PaperClientError {
    WIP
}
