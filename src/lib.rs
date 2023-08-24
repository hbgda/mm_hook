pub mod macros;
pub mod game;
pub mod utils;
pub mod logging;
pub mod patterns;

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
    canny
};

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct ModInfo {
    pub title: &'static str,
    pub version: &'static str,
    pub author: &'static str
}