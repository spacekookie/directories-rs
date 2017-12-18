use std;
use std::path::PathBuf;

extern crate winapi;
use self::winapi::um::knownfolders;
use self::winapi::um::combaseapi;
use self::winapi::um::shlobj;
use self::winapi::um::shtypes;
use self::winapi::um::winnt;

use BaseDirectories;

pub fn base_directories() -> BaseDirectories {
    let home_dir         = unsafe { known_folder(&knownfolders::FOLDERID_UserProfiles) };
    let data_dir         = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) };
    let data_roaming_dir = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) };
    let desktop_dir      = unsafe { known_folder(&knownfolders::FOLDERID_Desktop) };
    let documents_dir    = unsafe { known_folder(&knownfolders::FOLDERID_Documents) };
    let download_dir     = unsafe { known_folder(&knownfolders::FOLDERID_Downloads) };
    let music_dir        = unsafe { known_folder(&knownfolders::FOLDERID_Music) };
    let pictures_dir     = unsafe { known_folder(&knownfolders::FOLDERID_Pictures) };
    let public_dir       = unsafe { known_folder(&knownfolders::FOLDERID_Public) };
    let templates_dir    = unsafe { known_folder(&knownfolders::FOLDERID_Templates) };
    let videos_dir       = unsafe { known_folder(&knownfolders::FOLDERID_Videos) };

    let cache_dir        = data_dir.join("\\cache");
    let config_dir       = data_roaming_dir.clone();

    BaseDirectories {
        home_dir:         home_dir,
        cache_dir:        cache_dir,
        config_dir:       config_dir,
        data_dir:         data_dir,
        data_roaming_dir: data_roaming_dir,
        runtime_dir:      None,
        desktop_dir:      desktop_dir,
        documents_dir:    documents_dir,
        download_dir:     download_dir,
        music_dir:        music_dir,
        pictures_dir:     pictures_dir,
        public_dir:       public_dir,
        templates_dir:    Some(templates_dir),
        videos_dir:       videos_dir,
        executables_dir:  None,
        fonts_dir:        None
    }
}

unsafe fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> PathBuf {
    let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
    let _result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
    let len = length_of_u16_string(path_ptr);
    let path = std::slice::from_raw_parts(path_ptr, len);
    let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
    combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
    PathBuf::from(ostr)
}

unsafe fn length_of_u16_string(ptr: *mut u16) -> usize {
    let mut index = 0;
    while *ptr.offset(index as isize) != 0 as u16 {
        index += 1;
    }
    index
}