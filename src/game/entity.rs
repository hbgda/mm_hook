use std::{ffi::CStr, str::Utf8Error, ops::{Deref, DerefMut}};

use crate::scan_func_static;

use super::transform::Transform;

scan_func_static!(crate::patterns::ENTITY_GETENTITY, GET_ENTITY(*const u64) -> u64);

pub unsafe fn get_entity(handle_ptr: *const u64) -> Option<Entity> {
    match GET_ENTITY(handle_ptr) {
        0 => None,
        entity => Some(Entity(entity))
    }
}

// Entity Structure
// 0x68: ComponentList
// 0x70: ComponentListLen
// 0xB0 = EntityName

#[derive(Debug)]
pub struct Entity(pub u64);
impl Entity {
    pub unsafe fn get_name(&self) -> Result<&str, Utf8Error> {
        CStr::from_ptr(*((self.0 + 0xB0) as *const u64) as *const i8).to_str()
    }

    pub unsafe fn get_component(&self, name: &str) -> Option<u64> {
        let component_list = *((self.0 + 0x68) as *const u64);
        let list_len = *((self.0 + 0x78) as *const u16) as u64;
        for i in 0..list_len {
            let idx = 0x10 * i;
            let name_ptr = *((*((component_list + idx) as *const u64) + 0x60) as *const u64) as *const i8;
            let component_name = CStr::from_ptr(name_ptr).to_str().expect(&format!("Failed to read component name: {name_ptr:#x?}"));
            // println!("Name: {component_name}");
            if component_name == name {
                return Some(*((component_list + idx + 8) as *const u64))
            }
        }
        None
    }

    pub unsafe fn get_transform(&self) -> &Transform {
        &*(*(self.0 as *const u64) as *const Transform)
    }

    pub unsafe fn get_transform_mut(&self) -> &mut Transform {
        &mut *(*(self.0 as *const u64) as *mut Transform)

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