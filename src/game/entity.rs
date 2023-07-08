use std::{ffi::CStr, str::Utf8Error};

use super::OFFSET_GET_ENTITY_FN;

// Entity Structure
// 0x68: ComponentList
// 0x70: ComponentListLen
// 0xB0 = EntityName

pub unsafe fn get_entity(handle_ptr: *const u64) -> u64 {
    let func = crate::make_func!(crate::get_offset_ptr(OFFSET_GET_ENTITY_FN), [*const u64], u64);
    func(handle_ptr)
}

pub unsafe fn get_entity_name(entity: u64) -> Result<&'static str, Utf8Error> {
    CStr::from_ptr(((entity + 0xB0) as *const u64).read() as *const i8).to_str()
}

pub unsafe fn get_component_by_name(entity: u64, name: &str) -> Option<u64> {
    let component_list = ((entity + 0x68) as *const u64).read();
    let list_len = ((entity + 0x78) as *const u16).read() as u64;
    for i in 0..list_len {
        let idx = 0x10 * i;
        let name_ptr = ((((component_list + idx) as *const u64).read() + 0x60) as *const u64).read() as *const i8;
        let component_name = CStr::from_ptr(name_ptr).to_str().expect(&format!("Failed to read component name: {name_ptr:#x?}"));
        // println!("Name: {component_name}");
        if component_name == name {
            return Some(((component_list + idx + 8) as *const u64).read())
        }
    }
    None
}