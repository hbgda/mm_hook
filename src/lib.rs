use std::ffi::CString;

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
#[derive(PartialEq, Clone)]
pub struct ModInfo {
    pub title: String,
    pub version: String,
    pub author: String
}

#[repr(C)]
pub struct CModInfo {
    pub title: *mut i8,
    pub version: *mut i8,
    pub author: *mut i8
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

impl<'a> Into<CModInfo> for ModInfo {
    fn into(self) -> CModInfo {
        CModInfo { 
            title: CString::new(self.title).unwrap().into_raw(), 
            version: CString::new(self.version).unwrap().into_raw(), 
            author: CString::new(self.author).unwrap().into_raw() 
        }
    }
}

impl<'a> From<CModInfo> for ModInfo {
    fn from(value: CModInfo) -> Self {
        unsafe { 
            let title = CString::from_raw(value.title);
            let version = CString::from_raw(value.version);
            let author = CString::from_raw(value.author);
            ModInfo {
                title: title.to_str().unwrap().to_owned(),
                version: version.to_str().unwrap().to_owned(),
                author: author.to_str().unwrap().to_owned()
            }
        }
    }
}