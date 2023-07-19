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
    pub title: &'static str,
    pub version: &'static str,
    pub author: &'static str
}

impl ModInfo {
    pub fn new(title: &'static str, version: &'static str, author: &'static str) -> ModInfo {
        ModInfo {
            title,
            version,
            author
        }
    }
}

// impl Into<CModInfo> for ModInfo {
//     fn into(self) -> CModInfo {
//         CModInfo { 
//             title: self.title.into as *const i8, 
//             version: self.version.as_ptr() as *const i8, 
//             author: self.author.as_ptr() as *const i8
//         }
//     }
// }

// impl<'a> From<CModInfo> for ModInfo {
//     fn from(value: CModInfo) -> Self {
//         unsafe { 
//             let title = CStr::from_ptr(value.title);
//             let version = CStr::from_ptr(value.version);
//             let author = CStr::from_ptr(value.author);
//             ModInfo {
//                 title: title.to_str().expect("Failed to cast ModInfo::Title").to_owned(),
//                 version: version.to_str().expect("Failed to cast ModInfo::Version").to_owned(),
//                 author: author.to_str().expect("Failed to cast ModInfo::Author").to_owned()
//             }
//         }
//     }
// }