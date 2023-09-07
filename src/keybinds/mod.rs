pub mod keybind;
pub mod hooks;

use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;

use crate::{scan_func_static, game::nx, make_hook, make_func};

use self::keybind::{Keybind, KeyCode, KeybindState};

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
    pub name: String,
    pub desc: String,
    pub event: fn(&KeybindState),
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
        name: String, 
        desc: String, 
        primary: KeyCode, 
        secondary: Option<KeyCode>, 
        locked: bool,
        event: fn(&KeybindState)
    ) {
        let name = format!("{name}\0");
        let desc = format!("{desc}\0");

        let secondary: u32 = match secondary {
            Some(k) => k as u32,
            None => 1000
        };

        let mut partial = KeybindData {
            idx: 0,
            name,
            desc,
            event,
            was_pressed: false
        };

        partial.idx = queue_create_keybind(
            partial.name.as_ptr(), 
            partial.desc.as_ptr(), 
            primary as u32, 
            secondary as u32, 
            locked
        );

        self.binds.push(partial);
    }

    pub unsafe fn poll(&mut self) {
        for data in self.binds.iter_mut() {
            let Some(bind) = get_keybind(data.idx) else { continue }; 
            let state = &*(*bind).state;
            if state.pressed == 1.0 && !data.was_pressed {
                (data.event)(state);
                data.was_pressed = true;
            }
            else if state.pressed != 1.0 {
                data.was_pressed = false;
            }
        }
    }
}
