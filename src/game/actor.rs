use std::{ffi::CStr, str::Utf8Error};

use crate::{scan_func_static, utils};

use super::{transform::Transform, component::{ComponentEntry, Component}};

scan_func_static!(crate::patterns::ACTOR_GETACTOR, GET_ACTOR(*const u32) -> *const Actor);
scan_func_static!(crate::patterns::ACTOR_SPAWNACTOR, SPAWN_ACTOR(u64, *const ()) -> *const Actor);
scan_func_static!(crate::patterns::ACTOR_ENABLE, ENABLE_ACTOR(*const Actor));

/// Not sure fully what 2nd param is but it dictates position,
/// can use pointer to player transform to spawn on player
pub unsafe fn spawn_actor(actor_hash: u64, pos: *const ()) -> Option<*const Actor> {
    utils::option_ptr(SPAWN_ACTOR(actor_hash, pos))
}

pub unsafe fn get_actor<'l>(handle: &u32) -> Option<&'l Actor> {
    let actor = GET_ACTOR(handle);
    if actor == std::ptr::null() {
        return None;
    }

    Some(&*actor)
}

pub unsafe fn get_actor_mut<'l>(handle: &u32) -> Option<&'l mut Actor> {
    let actor = GET_ACTOR(handle) as *mut Actor;
    if actor == std::ptr::null_mut() {
        return None;
    }

    Some(&mut *actor)
}

#[repr(C)]
#[derive(Debug)]
pub struct Actor {
    transform: *const Transform,
    _pad: [u8; 0x60],
    component_list: *const [ComponentEntry; 0],
    component_count: u16,
    _pad1: [u8; 0x38],
    name: *const i8
}

impl Actor {
    pub unsafe fn enable(&self) {
        ENABLE_ACTOR(self)
    }

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

    pub unsafe fn transform(&self) -> &Transform {
        &*self.transform
    }

    pub unsafe fn transform_mut(&mut self) -> &mut Transform {
        &mut *(self.transform as *mut Transform)
    }

    pub unsafe fn set_transform(&mut self, transform: *const Transform) {
        self.transform = transform;
    }
}