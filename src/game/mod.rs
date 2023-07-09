use std::{ffi::{CStr, CString}, str::Utf8Error};

pub mod hero;
pub mod entity;
pub mod hud;

// Bad janky cringe
// pub(crate) const OFFSET_HERO_PTR: isize = 0x66F98B8;



// TODO: Pattern Scanning for offsets
pub(crate) const OFFSET_HERO_HANDLE_PTR: isize = 0x66EAE2C;
pub(crate) const OFFSET_PLAYERHUDMESSAGE_PTR: isize = 0x777D7E0;