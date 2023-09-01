pub mod keybind;
pub mod hooks;

use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;

use crate::{scan_func_static, game::nx, make_hook, make_func};

use self::keybind::Keybind;

scan_func_static!(crate::patterns::KEYBIND_CREATEKEYBIND, CREATE_KEYBIND(*const (), u32, u32, *const u8, u32, u32) -> *const Keybind);

pub struct PartialBind {
    idx: u32,
    name: *const u8,
    desc: *const u8,
    primary: u32,
    secondary: u32,
    locked: bool,
}

static mut KEYBIND_INDEX: u32 = 0x50;
static mut KEYBIND_QUEUE: Vec<PartialBind> = Vec::new();
static mut CREATED_KEYBINDS: Lazy<HashMap<u32, *const Keybind>> = Lazy::new(HashMap::new);

pub unsafe fn queue_create_keybind(        
    name: *const u8, 
    desc: *const u8, 
    primary: u32, 
    secondary: u32, 
    locked: bool,
) -> u32 {
    let idx = next_index();
    KEYBIND_QUEUE.push(
        PartialBind {
            idx,
            name,
            desc,
            primary,
            secondary,
            locked,
        }
    );
    idx
}

pub unsafe fn get_keybind(idx: u32) -> Option<*const Keybind> {
    CREATED_KEYBINDS.get(&idx).copied()
}

pub unsafe fn next_index() -> u32 {
    let ret = KEYBIND_INDEX;
    KEYBIND_INDEX += 1;
    ret
}


struct KeybindData {
    pub idx: u32,
    pub event: fn(),
    pub was_pressed: bool
}

pub struct KeybindManager {
    binds: Vec<KeybindData>,
}

impl KeybindManager {
    pub fn new() -> KeybindManager {
        KeybindManager { binds: Vec::new() }
    }

    pub unsafe fn add_keybind(
        &mut self,
        name: &'static str, 
        desc: &'static str, 
        primary: u32, 
        secondary: Option<u32>, 
        locked: bool,
        event: fn()
    ) {
        let secondary = secondary.unwrap_or(1000);
        let idx = queue_create_keybind(name.as_ptr(), desc.as_ptr(), primary, secondary, locked);
        self.binds.push(
            KeybindData {
                idx,
                event,
                was_pressed: false
            }
        );
    }

    pub unsafe fn poll(&mut self) {
        for data in self.binds.iter_mut() {
            let Some(bind) = get_keybind(data.idx) else { continue }; 
            if (*(*bind).info).pressed && !data.was_pressed {
                (data.event)();
                data.was_pressed = true;
            }
            else if !(*(*bind).info).pressed {
                data.was_pressed = false;
            }
        }
    }
}
