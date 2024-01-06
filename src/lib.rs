pub mod macros;
pub mod game;
pub mod utils;
pub mod logging;
pub mod patterns;
pub mod keybinds;
pub mod settings;
pub mod overrides;

pub mod impls {
    pub mod component {
        pub use {
            mm_hook_macros::Component,
            crate::game::component::Component
        };
    }
}

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
            Console::* 
        }
    }},
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
    keybinds::hooks::enable();
}