use std::{ffi::CStr, str::Utf8Error, ops::{Deref, DerefMut}};

use super::OFFSET_GET_ENTITY_FN;

// Entity Structure
// 0x68: ComponentList
// 0x70: ComponentListLen
// 0xB0 = EntityName

#[derive(Debug)]
pub struct Entity(pub u64);
impl Entity {
    pub unsafe fn get(handle_ptr: *const u64) -> Option<Entity> {
        let func = crate::make_func!(crate::get_offset_ptr(OFFSET_GET_ENTITY_FN), [*const u64], u64);
        match func(handle_ptr) {
            0 => None,
            entity => Some(Entity(entity))
        }
    }

    pub unsafe fn name(&self) -> Result<&str, Utf8Error> {
        CStr::from_ptr(((self.0 + 0xB0) as *const u64).read() as *const i8).to_str()
    }

    pub unsafe fn get_component(&self, name: &str) -> Option<u64> {
        let component_list = ((self.0 + 0x68) as *const u64).read();
        let list_len = ((self.0 + 0x78) as *const u16).read() as u64;
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
}

impl Deref for Entity {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Entity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}