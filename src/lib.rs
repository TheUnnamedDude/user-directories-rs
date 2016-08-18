#[cfg(target_os="linux")]
#[path="linux.rs"]
mod platform;

use std::path::PathBuf;

pub use platform::*;

pub fn find_directory(folder_type: &DirectoryType) -> Option<PathBuf> {
    match *folder_type {
        DirectoryType::Config => get_config_dir(),
        DirectoryType::Home => get_home_dir(),
        DirectoryType::Documents => get_documents_dir(),
        DirectoryType::Videos => get_videos_dir(),
        DirectoryType::Music => get_music_dir(),
        DirectoryType::Pictures => get_pictures_dir(),
        DirectoryType::Downloads => get_downloads_dir(),
        DirectoryType::Desktop => get_desktop_dir(),
        DirectoryType::Applications => get_applications_dir()
    }
}

pub enum DirectoryType {
    Config,
    Home,
    Documents,
    Videos,
    Music,
    Pictures,
    Downloads,
    Desktop,
    Applications
}

impl DirectoryType {
    pub fn all() -> [DirectoryType; 9] {
        [
            DirectoryType::Config,
            DirectoryType::Home,
            DirectoryType::Documents,
            DirectoryType::Videos,
            DirectoryType::Music,
            DirectoryType::Pictures,
            DirectoryType::Downloads,
            DirectoryType::Desktop,
            DirectoryType::Applications
        ]
    }
}