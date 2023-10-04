use std::{ffi::CStr, str::Utf8Error};

use crate::scan_func_static;

use super::transform::Transform;

scan_func_static!(crate::patterns::ENTITY_GETENTITY, GET_ENTITY(*const u64) -> *const Entity);

pub unsafe fn get_entity<'l>(handle_ptr: *const u64) -> Option<&'l Entity> {
    let entity = GET_ENTITY(handle_ptr);
    if entity == std::ptr::null() {
        return None;
    }

    Some(&*entity)
}

pub unsafe fn get_entity_mut<'l>(handle_ptr: *const u64) -> Option<&'l mut Entity> {
    let entity = GET_ENTITY(handle_ptr) as *mut Entity;
    if entity == std::ptr::null_mut() {
        return None;
    }

    Some(&mut *entity)
}

// Entity Structure
// 0x68: ComponentList
// 0x70: ComponentListLen
// 0xB0 = EntityName

#[repr(C)]
pub struct Register {
    _pad: [u8; 0x60],
    pub name: *const u8
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComponentEntry {
    pub register: *const Register,
    pub component: *const ()
}

#[repr(C)]
#[derive(Debug)]
pub struct Entity {
    transform: *const Transform,
    _pad: [u8; 0x60],
    component_list: *const [ComponentEntry; 0],
    component_count: u16,
    _pad1: [u8; 0x38],
    name: *const u8
}

impl Entity {
    pub unsafe fn get_name(&self) -> Result<&str, Utf8Error> {
        CStr::from_ptr(self.name as *const i8).to_str()
    }
    
    pub unsafe fn get_components_sized(&self) -> Vec<&ComponentEntry> {
        (0..self.component_count as usize)
            .into_iter()
            .map(|i| (&*self.component_list).get_unchecked(i))
            .collect()
    }

    pub unsafe fn get_components(&self) -> &[ComponentEntry] {
        (&*self.component_list) as &[_]
    }

    pub unsafe fn get_component_by_name(&self, name: &str) -> Option<*const ()> {
        let component_list = self.get_components();
        for i in 0..self.component_count as usize {
            let entry = component_list.get_unchecked(i);
            let entry_name = CStr::from_ptr((*entry.register).name as *const i8)
                .to_str()
                .expect(&format!("Failed to read name for component: {:#p}", entry.component));
            if entry_name == name {
                return Some(entry.component)
            }
        }
        None
    }

    pub unsafe fn get_component<T>(&self, name: &str) -> Option<&T> {
        Some(&*(self.get_component_by_name(name)? as *const T))
    }

    pub unsafe fn get_component_mut<T>(&mut self, name: &str) -> Option<&mut T> {
        Some(&mut *(self.get_component_by_name(name)? as *mut T))
    }

    // TODO: Maybe
    // pub unsafe fn get_component<T: Component>() -> Option<&T> {
    //     todo!()
    // }

    pub unsafe fn get_transform(&self) -> &Transform {
        &*self.transform
    }

    pub unsafe fn get_transform_mut(&mut self) -> &mut Transform {
        &mut *(self.transform as *mut Transform)
    }

    pub unsafe fn set_transform(&mut self, transform: *const Transform) {
        self.transform = transform;
    }
}