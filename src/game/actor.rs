use std::ffi::CStr;

use crate::{declare_native_func, patterns, utils};

use super::{transform::{Transform, SpatialData, Vector3}, component::{ComponentEntry, Component}};

declare_native_func!(
    utils::scan(patterns::ACTOR_GETACTOR).unwrap(),
    GET_ACTOR(*const u32) -> *const Actor
);
declare_native_func!(
    utils::scan(patterns::ACTOR_SPAWNACTOR).unwrap(),
    SPAWN_ACTOR(u64, *const SpatialData) -> *const Actor
);
declare_native_func!(
    utils::scan(patterns::ACTOR_ENABLE).unwrap(),
    ENABLE_ACTOR(*const Actor)
);

// scan_func_static!(crate::patterns::ACTOR_GETACTOR, GET_ACTOR(*const u32) -> *const Actor);
// scan_func_static!(crate::patterns::ACTOR_SPAWNACTOR, SPAWN_ACTOR(u64, *const SpatialData) -> *const Actor);
// scan_func_static!(crate::patterns::ACTOR_ENABLE, ENABLE_ACTOR(*const Actor));

pub unsafe fn get_actor<'l>(handle: &u32) -> Option<&'l Actor> {
    Some(
        &*(utils::option_ptr(
            GET_ACTOR(handle)
        )?)
    )
}

pub unsafe fn get_actor_mut<'l>(handle: &u32) -> Option<&'l mut Actor> {
    Some(get_actor(handle)?.as_mut())
}

#[repr(C)]
#[derive(Debug)]
pub struct Actor {
    transform: *const Transform,
    _0x8: u32,
    _handle_part: u32,
    _handle_part_2: u16,
    _0x12: [u8; 0x56],
    component_list: *const [ComponentEntry; 0],
    component_count: u16,
    _pad1: [u8; 0x38],
    name: *const i8
}

impl Actor {
    pub unsafe fn as_mut(&self) -> &mut Actor {
        &mut *(self as *const Actor as *mut Actor)
    }

    pub unsafe fn spawn_at_pos<'l>(actor_hash: u64, pos: Vector3) -> Option<&'l Actor> {
        Actor::spawn(actor_hash, SpatialData::from_pos(pos))
    }

    pub unsafe fn spawn<'l>(actor_hash: u64, spatial_data: SpatialData) -> Option<&'l Actor> {
        Some(
            &*(utils::option_ptr(
                SPAWN_ACTOR(actor_hash, &spatial_data)
            )?)
        )
    }

    pub unsafe fn enable(&self) {
        ENABLE_ACTOR(self)
    }

    pub unsafe fn get_name(&self) -> Option<&str> {
        if let Ok(name) = CStr::from_ptr(self.name).to_str() {
            return Some(name)
        }
        None
    }

    // ???
    pub unsafe fn handle(&self) -> u32 {
        ((self._handle_part_2 as u32) << 0x14) | self._handle_part
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