use std::{ffi::{CStr, CString}, str::Utf8Error};

pub mod hero;
pub mod entity;

// Bad janky cringe
// pub(crate) const OFFSET_HERO_PTR: isize = 0x66F98B8;



// TODO: Pattern Scanning for offsets
pub(crate) const OFFSET_GET_ENTITY_FN: isize = 0x1AA8630;
pub(crate) const OFFSET_HERO_HANDLE_PTR: isize = 0x66EAE2C;