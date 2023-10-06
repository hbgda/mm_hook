pub mod macros;
pub mod game;
pub mod utils;
pub mod logging;
pub mod patterns;
pub mod keybinds;
pub mod settings;
pub mod overrides;

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

// #[derive(Eq, PartialEq, Hash)]
// pub struct SendPtr<T> {
//     pub ptr: *const T
// }
// unsafe impl<T> Send for SendPtr<T> {}


pub unsafe fn init() {
    game::hud::HOOK_HUD_CreatePlayerHUD_Intercept.enable()
        .expect("Failed to enable hook: PlayerHUD::Init()");

    overrides::init_scaleform();
    keybinds::hooks::enable();
}