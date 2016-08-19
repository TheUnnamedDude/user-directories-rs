use std::path::PathBuf;
use std::ptr;

use shell32;
use ole32;
use winapi::shtypes::REFKNOWNFOLDERID;
use winapi::guiddef::GUID;

/*
FOLDERID_Desktop
{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}
*/
const FOLDERID_DESKTOP: REFKNOWNFOLDERID = &GUID {
    Data1: 0xB4BFCC3A,
    Data2: 0xDB2C,
    Data3: 0x424C,
    Data4: [0xB0, 0x29, 0x7F, 0xE9, 0x9A, 0x87, 0xC6, 0x41]
};

/*
FOLDERID_Documents
{FDD39AD0-238F-46AF-ADB4-6C85480369C7}
*/
const FOLDERID_DOCUMENTS: REFKNOWNFOLDERID = &GUID {
    Data1: 0xFDD39AD0,
    Data2: 0x238F,
    Data3: 0x46AF,
    Data4: [0xAD, 0xB4, 0x6C, 0x85, 0x48, 0x03, 0x69, 0xC7]
} as *const _;

/*
FOLDERID_Downloads
{374DE290-123F-4565-9164-39C4925E467B}
*/
const FOLDERID_DOWNLOADS: REFKNOWNFOLDERID = &GUID {
    Data1: 0x374DE290,
    Data2: 0x123F,
    Data3: 0x4565,
    Data4: [0x91, 0x64, 0x39, 0xC4, 0x92, 0x5E, 0x46, 0x7B]
};

/*
FOLDERID_Music
{4BD8D571-6D19-48D3-BE97-422220080E43}
*/
const FOLDERID_MUSIC: REFKNOWNFOLDERID = &GUID {
    Data1: 0x4BD8D571,
    Data2: 0x6D19,
    Data3: 0x48D3,
    Data4: [0xBE, 0x97, 0x42, 0x22, 0x20, 0x08, 0x0E, 0x43]
};

/*
FOLDERID_Pictures
{33E28130-4E1E-4676-835A-98395C3BC3BB}
*/
const FOLDERID_PICTURES: REFKNOWNFOLDERID = &GUID {
    Data1: 0x33E28130,
    Data2: 0x4E1E,
    Data3: 0x4676,
    Data4: [0x83, 0x5A, 0x98, 0x39, 0x5C, 0x3B, 0xC3, 0xBB]
};

/*
FOLDERID_Profile
{5E6C858F-0E22-4760-9AFE-EA3317B67173}
*/
const FOLDERID_PROFILE: REFKNOWNFOLDERID = &GUID {
    Data1: 0x5E6C858F,
    Data2: 0x0E22,
    Data3: 0x4760,
    Data4: [0x9A, 0xFE, 0xEA, 0x33, 0x17, 0xB6, 0x71, 0x73]
} as *const _;

/*
FOLDERID_RoamingAppData
{3EB685DB-65F9-4CF6-A03A-E3EF65729F3D}
*/
const FOLDERID_APPDATA: REFKNOWNFOLDERID = &GUID {
    Data1: 0x3EB685DB,
    Data2: 0x65F9,
    Data3: 0x4CF6,
    Data4: [0xA0, 0x3A, 0xE3, 0xEF, 0x65, 0x72, 0x9F, 0x3D]
} as *const _;

/*
FOLDERID_Videos
{18989B1D-99B5-455B-841C-AB7C74E4DDFC}
*/
const FOLDERID_VIDEOS: REFKNOWNFOLDERID = &GUID {
    Data1: 0x18989B1D,
    Data2: 0x99B5,
    Data3: 0x455B,
    Data4: [0x84, 0x1C, 0xAB, 0x7C, 0x74, 0xE4, 0xDD, 0xFC]
};


pub fn get_config_dir() -> Option<PathBuf> {
    get_path(FOLDERID_APPDATA)
}

pub fn get_home_dir() -> Option<PathBuf> {
    get_path(FOLDERID_PROFILE)
}

pub fn get_documents_dir() -> Option<PathBuf> {
    get_path(FOLDERID_DOCUMENTS)
}

pub fn get_downloads_dir() -> Option<PathBuf> {
    get_path(FOLDERID_DOWNLOADS)
}

pub fn get_desktop_dir() -> Option<PathBuf> {
    get_path(FOLDERID_DESKTOP)
}

pub fn get_pictures_dir() -> Option<PathBuf> {
    get_path(FOLDERID_PICTURES)
}

pub fn get_videos_dir() -> Option<PathBuf> {
    get_path(FOLDERID_VIDEOS)
}

pub fn get_music_dir() -> Option<PathBuf> {
    get_path(FOLDERID_MUSIC)
}
pub fn get_applications_dir() -> Option<PathBuf> {
    get_path(FOLDERID_APPDATA)
}

pub fn get_path(folder_id: REFKNOWNFOLDERID) -> Option<PathBuf> {
    unsafe {
        let mut path_ptr: *mut u16 = ptr::null_mut();
        shell32::SHGetKnownFolderPath(folder_id, 0, ptr::null_mut(), &mut path_ptr);
        let mut string_content = vec!();
        let mut i = 0;
        loop {
            let char = *path_ptr.offset(i);
            if char == 0 {
                break;
            }
            string_content.push(char);
            i += 1;
        }
        ole32::CoTaskMemFree(path_ptr as *mut _);
        Some(PathBuf::from(String::from_utf16_lossy(string_content.as_slice())))
    }
}