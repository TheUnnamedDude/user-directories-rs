use std::path::{Path, PathBuf};
use std::env;
use std::io::{prelude, Read};
use std::fs::File;

pub fn get_config_dir() -> Option<PathBuf> {
    if let Some(var) = get_env_var("XDG_CONFIG_HOME") {
        Some(PathBuf::from(var))
    } else {
        get_home_dir().map(| mut dir | {
            dir.push(".config");
            dir
        })
    }
}

pub fn get_home_dir() -> Option<PathBuf> {
    if let Some(home) = get_env_var("HOME") {
        Some(PathBuf::from(home))
    } else {
        None
    }
}

pub fn get_documents_dir() -> Option<PathBuf> {
    find_path("XDG_DOCUMENTS_DIR")
}

pub fn get_downloads_dir() -> Option<PathBuf> {
    find_path("XDG_DOWNLOAD_DIR")
}

pub fn get_desktop_dir() -> Option<PathBuf> {
    find_path("XDG_DESKTOP_DIR")
}

pub fn get_pictures_dir() -> Option<PathBuf> {
    find_path("XDG_PICTURES_DIR")
}

pub fn get_videos_dir() -> Option<PathBuf> {
    find_path("XDG_VIDEOS_DIR")
}

pub fn get_music_dir() -> Option<PathBuf> {
    find_path("XDG_MUSIC_DIR")
}
pub fn get_applications_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/opt"))
}

fn find_path(xdg_var: &'static str) -> Option<PathBuf> {
    let mut xdg_dir_file = if let Some(path) = get_config_dir() {
        if let Ok(file) = File::open(path.join("user-dirs.dirs")) {
            file
        } else {
            return None;
        }
    } else {
        return None;
    };

    let mut xdg_dir_config = String::new();
    if xdg_dir_file.read_to_string(&mut xdg_dir_config).is_err() {
        return None;
    }
    for line in xdg_dir_config.lines() {
        if line.starts_with(xdg_var) {
            if let Some(var) = parse_string(line) {
                if var.contains("$HOME") {
                    return if let Some(home_dir) = get_home_dir() {
                        Some(home_dir.join(var.replace("$HOME/", "")).to_path_buf())
                    } else {
                        None
                    }
                }
                return Some(PathBuf::from(var));
            }
        }
    }
    None
}

fn get_env_var(env_var: &'static str) -> Option<String> {
    if let Ok(var) = env::var(env_var) {
        Some(var)
    } else {
        None
    }
}

fn parse_string(string: &str) -> Option<String> {
    let start_index = string.find('"');
    let end_index = string.rfind('"');
    if start_index.is_none() || end_index.is_none() {
        return None;
    }
    Some(string[start_index.unwrap() + 1..end_index.unwrap()].to_owned())
}