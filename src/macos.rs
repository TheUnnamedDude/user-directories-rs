use std::env;
use std::path::PathBuf;

/*
TODO: Implement this with:
https://developer.apple.com/library/mac/documentation/Cocoa/Reference/Foundation/Miscellaneous/Foundation_Functions/index.html#//apple_ref/c/func/NSSearchPathForDirectoriesInDomains
and
https://developer.apple.com/library/mac/documentation/Cocoa/Reference/Foundation/Miscellaneous/Foundation_Constants/index.html#//apple_ref/doc/c_ref/NSSearchPathDirectory
*/

pub fn get_config_dir() -> Option<PathBuf> {
    get_subhome_path("Library/Application Support")
}

pub fn get_home_dir() -> Option<PathBuf> {
    if let Some(home) = get_env_var("HOME") {
        Some(PathBuf::from(home))
    } else {
        None
    }
}

pub fn get_documents_dir() -> Option<PathBuf> {
    get_subhome_path("Documents")
}

pub fn get_downloads_dir() -> Option<PathBuf> {
    get_subhome_path("Downloads")
}

pub fn get_desktop_dir() -> Option<PathBuf> {
    get_subhome_path("Desktop")
}

pub fn get_pictures_dir() -> Option<PathBuf> {
    get_subhome_path("Pictures")
}

pub fn get_videos_dir() -> Option<PathBuf> {
    get_subhome_path("Videos")
}

pub fn get_music_dir() -> Option<PathBuf> {
    get_subhome_path("Music")
}
pub fn get_applications_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/Applications"))
}

fn get_subhome_path(path: &'static str) -> Option<PathBuf> {
    get_home_dir().map(| mut home | {
        home.push(path);
        home
    })
}

fn get_env_var(env_var: &'static str) -> Option<String> {
    if let Ok(var) = env::var(env_var) {
        Some(var)
    } else {
        None
    }
}