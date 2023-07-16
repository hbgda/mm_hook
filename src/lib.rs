use std::ffi::{CString, CStr};

pub mod macros;
pub mod game;
pub mod utils;
pub mod logging;

pub use {
    windows::{s, Win32::{
        UI::{
            WindowsAndMessaging::*,
            Input::KeyboardAndMouse::*
        },
        Foundation::*,
        System::{
            SystemServices::*,
            LibraryLoader::*,
        }
    }},
    once_cell::sync::Lazy,
    paste::paste,
    retour::GenericDetour
};

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct ModInfo {
    pub title: String,
    pub version: String,
    pub author: String
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CModInfo {
    pub title: *const i8,
    pub version: *const i8,
    pub author: *const i8
}

impl ModInfo {
    pub fn new(title: &str, version: &str, author: &str) -> ModInfo {
        ModInfo {
            title: title.to_string(),
            version: version.to_string(),
            author: author.to_string()
        }
    }
}

impl Into<CModInfo> for ModInfo {
    fn into(self) -> CModInfo {
        CModInfo { 
            title: format!("{}\0", self.title).as_ptr() as *const i8, 
            version: format!("{}\0", self.version).as_ptr() as *const i8, 
            author: format!("{}\0", self.author).as_ptr() as *const i8
        }
    }
}

impl<'a> From<CModInfo> for ModInfo {
    fn from(value: CModInfo) -> Self {
        unsafe { 
            let title = CStr::from_ptr(value.title);
            let version = CStr::from_ptr(value.version);
            let author = CStr::from_ptr(value.author);
            ModInfo {
                title: title.to_str().expect("Failed to cast ModInfo::Title").to_owned(),
                version: version.to_str().expect("Failed to cast ModInfo::Version").to_owned(),
                author: author.to_str().expect("Failed to cast ModInfo::Author").to_owned()
            }
        }
    }
}