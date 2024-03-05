pub mod macros;
pub mod game;
pub mod utils;
pub mod logging;
pub mod patterns;
pub mod keybinds;
pub mod overrides;
pub mod menu;
pub mod hash;

pub mod windows {
    pub use windows::{s, Win32::{
        UI::{
            WindowsAndMessaging::*,
            Input::KeyboardAndMouse::*
        },
        Foundation::*,
        System::{
            SystemServices::*,
            LibraryLoader::*,
            Console::* 
        }
    }};
}
pub use {
    once_cell::sync::Lazy,
    paste::paste,
    retour::GenericDetour,
    canny,
    mm_hook_macros as proc_macros
};

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct ModInfo {
    pub title: &'static str,
    pub version: &'static str,
    pub author: &'static str
}

pub unsafe fn init() {
    overrides::init_scaleform();
    keybinds::hooks::enable();
    menu::hooks::enable();
}
