use std::{ffi::{CStr, CString}, str::Utf8Error};

use crate::{GameType, get_offset_ptr};

pub mod hero;

// TODO: Pattern Scanning for offsets
pub(crate) const OFFSET_GET_ENTITY_FN: isize = 0x1AA8630;
pub(crate) const OFFSET_HERO_PTR: isize = 0x66F98B8;

pub unsafe fn get_hero_entity() -> u64 {
    let p1: *const u64 = get_offset_ptr(OFFSET_HERO_PTR);
    let p2: *const u64 = dbg!(p1.read() as *const u64);
    let p3: u64 = dbg!(((p2.read() + 8) as *const u64).read());
    p3
}

pub unsafe fn get_entity(handle_ptr: *const u64) -> u64 {
    let func = crate::make_func!(crate::get_offset_ptr(OFFSET_GET_ENTITY_FN), [*const u64], u64);
    func(handle_ptr)
}

pub unsafe fn get_entity_name(entity: u64) -> Result<&'static str, Utf8Error> {
    CStr::from_ptr((entity + 0xB0) as *const i8).to_str()
}