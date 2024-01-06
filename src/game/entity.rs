use std::{ffi::CStr, str::Utf8Error};

use crate::scan_func_static;

use super::{transform::Transform, component::{ComponentEntry, Component}};

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

#[repr(C)]
#[derive(Debug)]
pub struct Entity {
    transform: *const Transform,
    _pad: [u8; 0x60],
    component_list: *const [ComponentEntry; 0],
    component_count: u16,
    _pad1: [u8; 0x38],
    name: *const i8
}

impl Entity {
    pub unsafe fn get_name(&self) -> Option<&str> {
        if let Ok(name) = CStr::from_ptr(self.name).to_str() {
            return Some(name)
        }
        None
    }
    
    pub unsafe fn get_components_sized(&self) -> Vec<&ComponentEntry> {
        (0..self.component_count as usize)
            .into_iter()
            .map(|i| (&*self.component_list).get_unchecked(i))
            .collect()
    }

    pub unsafe fn get_components_sized_mut(&mut self) -> Vec<&mut ComponentEntry> {
        (0..self.component_count as usize)
            .into_iter()
            .map(|i| (&mut *(self.component_list as *mut [ComponentEntry; 0])).get_unchecked_mut(i))
            .collect()
    }

    pub unsafe fn get_components(&self) -> &[ComponentEntry] {
        (&*self.component_list) as &[_]
    }

    pub unsafe fn get_components_mut(&mut self) -> &mut [ComponentEntry] {
        (&mut *(self.component_list as *mut [_; 0])) as &mut [_]
    }

    pub unsafe fn get_component_entry(&self, name: &str) -> Option<*const ComponentEntry> {
        let component_list = self.get_components();
        for i in 0..self.component_count as usize {
            let entry = component_list.get_unchecked(i);
            let entry_name = entry.info()?.get_name()?;
            if entry_name == name {
                return Some(entry)
            }
        }
        None
    }

    pub unsafe fn get_component<T: Component>(&self) -> Option<&T> {
        (&*(
            self.get_component_entry(T::NAME)?
        )).component()
    }

    pub unsafe fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        (&mut *(
            self.get_component_entry(T::NAME)? as *mut ComponentEntry
        )).component_mut()
    }

    pub unsafe fn get_component_by_name(&mut self, name: &str) -> Option<*const ()> {
        Some(
            (&*(
                self.get_component_entry(name)?
            )).component_ptr
        )
    }

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