use std::ffi::{c_char, CStr, CString};

pub mod macros;
pub mod game;
pub mod utils;

pub use {
    windows::{s, Win32::{
        UI::{
            WindowsAndMessaging::*,
            Input::KeyboardAndMouse::*
        },
        Foundation::*,
        System::SystemServices::*
    }},
    once_cell::sync::Lazy,
    paste::paste,
    retour::GenericDetour
};

#[repr(C)]
pub struct ModInfo {
    pub title: &'static str,
    pub version: &'static str,
    pub author: &'static str
}

#[repr(C)]
pub struct CModInfo {
    pub title: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char
}

impl Into<CModInfo> for ModInfo {
    fn into(self) -> CModInfo {
        CModInfo { 
            title: CString::new(self.title).unwrap().into_raw(), 
            version: CString::new(self.version).unwrap().into_raw(), 
            author: CString::new(self.author).unwrap().into_raw() 
        }
    }
}