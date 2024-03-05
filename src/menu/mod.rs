use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

use crate::game::scaleform::value::{self, Value};
use crate::logging::Logger;

pub mod item;
pub mod hooks;
use item::*;
use once_cell::sync::Lazy;


// TODO: Seperate this + other stuff into a standlone dll so multiple mods can actually work
static mut VALUE_CALLBACKS_MAP: Lazy<HashMap<u32, fn(f64)>> = Lazy::new(HashMap::new);
static mut BUTTON_CALLBACKS_MAP: Lazy<HashMap<u32, fn()>> = Lazy::new(HashMap::new); 
static mut OPTION_COUNT: AtomicU32 = AtomicU32::new(0);

fn gen_id(label: &str) -> u32 {
    let id = unsafe { &OPTION_COUNT }.fetch_add(1, Ordering::SeqCst);

    let mut opt_hasher = DefaultHasher::new();
    format!("MOD_OPT_{label}_{id}").hash(&mut opt_hasher);
    let opt_hash = opt_hasher.finish();

    ((opt_hash >> 32) as u32) | (opt_hash & 0xFFFF) as u32
}

fn register_value_callback(id: u32, cb: fn(f64)) {
    unsafe { &mut VALUE_CALLBACKS_MAP }.insert(id, cb);
}

fn register_button_callback(id: u32, cb: fn()) {
    unsafe { &mut BUTTON_CALLBACKS_MAP }.insert(id, cb);
}

pub fn emit_value_callback(id: u32, val: f64) -> bool {
    if let Some(cb) = unsafe { &VALUE_CALLBACKS_MAP }.get(&id) {
        cb(val);
        return true
    }
    false
}

pub fn emit_button_callback(id: u32) -> bool {
    if let Some(cb) = unsafe { &BUTTON_CALLBACKS_MAP }.get(&id) {
        cb();
        return true
    }
    false
}

pub struct  OptionsMenu {
    header: String,
    // item_id: u32,
    items: Vec<Box<dyn OptionItem>>,
}

#[derive(Clone, Copy)]
pub enum MenuWidth {
    Wide,
    Narrow,
    // Width(u32)
}

impl OptionsMenu {
    pub fn new(header: &str) -> OptionsMenu {
        let header = format!("{header}\0");
        OptionsMenu {
            header,
            items: Vec::new(),
        }
    }

    pub fn header(&self) -> &str {
        self.header.as_str()
    }

    fn add_item(&mut self, item: Box<dyn OptionItem>) {
        self.items.push(item);
    }

    pub unsafe fn to_value(&self) -> Value {
        let mut menu_value = value::create_value! {
            "header": self.header.as_ptr(),
            "width": MenuWidth::Wide as u32
        };

        let mut menu_items = Value::create_array();
        for item in self.items.iter() {
            menu_items.push_back(&item.to_value());
        }

        menu_value.set_member("items", &menu_items);    
        menu_value
    }
}

impl OptionsMenu {
    pub fn add_blank(&mut self) {
        self.add_item(Box::new(Blank { } ));
    }

    pub fn add_header(&mut self, label: &'static str) {
        self.add_item(Box::new(Header { label }))
    }

    pub fn add_button(&mut self, label: &'static str, desc: Option<&'static str>, on_clickled: fn()) {
        let id = gen_id(label);
        self.add_item(Box::new(Command { id, label, desc }));
        register_button_callback(id, on_clickled);
    }

    pub fn add_select(
        &mut self,
        label: String,
        desc: Option<String>,
        options: Vec<String>,
        selected: u32,
        default: u32,
        on_change: fn(f64)
    ) {
        let id = gen_id(&label);
        self.add_item(Box::new(Select::new(
            id, label, desc, options, selected, default,
        )));
        register_value_callback(id, on_change);
    }

    pub fn add_slider(
        &mut self,
        label: &'static str,
        desc: Option<&'static str>,
        value: f64,
        minimum: f64,
        maximum: f64,
        default: f64,
        on_change: fn(f64)
        // fidelity: u32,
    ) {
        let id = gen_id(label);
        self.add_item(Box::new(Slider {
            id, label, desc, value, minimum, maximum, default, // fidelity,
        }));
        register_value_callback(id, on_change)
    }

    pub fn add_colour(&mut self, label: &'static str, desc: Option<&'static str>, colours: Vec<u32>, selected: u32, default: u32) {
        let id = gen_id(label);
        self.add_item(Box::new(Colour {
            id, label, desc, selected, default, colours: colours.iter().enumerate().map(|(i, col)| {
                ColourElement {
                    colour_index: i as u32,
                    colour_type: ColourType::Normal,
                    colour_value: *col,
                    colour_name: "TODO\0"
                }
            }).collect()  
        }))
    }

    // pub fn add_keybind(&mut self, label: &'static str, desc: Option<&'static str>, binding: &'static str, is_icon_type: bool, locked: bool, can_reset: bool) {
    //     let id = self.item_id();
    //     self.add_item(Box::new(Keybind {
    //         id, label, desc, binding, is_icon_type, locked, can_reset
    //     }))
    // }
}
