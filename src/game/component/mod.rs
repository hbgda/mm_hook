pub mod hero;

use std::ffi::CStr;

pub trait Component {
    const NAME: &'static str;
}

#[repr(C)]
pub struct ComponentInfo {
    _pad: [u8; 0x60],
    name_ptr: *const i8
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComponentEntry {
    pub info_ptr: *const ComponentInfo,
    pub component_ptr: *const ()
}

impl ComponentInfo {
    pub unsafe fn get_name(&self) -> Option<&str> {
        if self.name_ptr.is_null() {
            return None
        }
        if let Ok(name) = CStr::from_ptr(self.name_ptr).to_str() {
            return Some(name)
        }
        None
    }
}

impl ComponentEntry {
    pub unsafe fn info(&self) -> Option<&ComponentInfo> {
        if self.info_ptr.is_null() {
            return None
        }
        Some(&*self.info_ptr)
    }

    pub unsafe fn component<T>(&self) -> Option<&T> {
        if self.component_ptr.is_null() {
            return None
        }
        Some(&*(self.component_ptr as *const T))
    }

    pub unsafe fn component_mut<T>(&mut self) -> Option<&mut T> {
        if self.component_ptr.is_null() {
            return None
        }
        Some(&mut *(self.component_ptr as *mut T))
    }
}