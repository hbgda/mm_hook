use crate::{game::scaleform::value::{self, Value}, utils};

pub unsafe fn item_base(id: u32, label: &'static str, desc: &'static str) -> Value {
    value::create_value! {
        "id": id,
        "label": label,
        "desc": desc,
        "available": true,
        "active": true
    }
}

enum OptionType {
    Select,
    Slider,
    Colour = 3,
    Command,
    Header,
    Blank,
    Keybind = 9,
}

pub trait OptionItem {
    unsafe fn to_value(&self) -> Value;
}

impl<T> From<T> for Value
where T:
    OptionItem + Sized 
{
    fn from(value: T) -> Self {
        unsafe { value.to_value() }
    }
}

pub struct Blank;
impl OptionItem for Blank {
    unsafe fn to_value(&self) -> Value {
        value::create_value! {
            "option_type": OptionType::Blank as u32
        }
    }
}

pub struct Command {
    pub id: u32,
    pub label: &'static str,
    pub desc: Option<&'static str>,
}

impl OptionItem for Command {
    unsafe fn to_value(&self) -> Value {
        value::create_value! {
            ~item_base(self.id, self.label, self.desc.unwrap_or("\0")),
            "option_type": OptionType::Command as u32
        }
    }
}

pub struct Header {
    pub label: &'static str
}

impl OptionItem for Header {
    unsafe fn to_value(&self) -> Value {
        value::create_value! {
            ~item_base(0, self.label, "\0"),
            "option_type": OptionType::Header as u32
        }
    }
}

pub struct Select {
    pub id: u32,
    pub label: String,
    pub desc: Option<String>,
    pub options: Vec<String>,
    pub selected: u32,
    pub default: u32,
}

impl Select {
    pub fn new(id: u32, mut label: String, desc: Option<String>, mut options: Vec<String>, selected: u32, default: u32) -> Select {
        utils::terminate_string(&mut label);
        let desc = match desc {
            Some(mut str) => { utils::terminate_string(&mut str); Some(str) },
            None => None
        };
        options.iter_mut().for_each(utils::terminate_string);
        Select {
            id, 
            label,
            desc, 
            options,
            selected,
            default
        }
    }
}

impl OptionItem for Select {
    unsafe fn to_value(&self) -> Value {
        let mut select_value = value::create_value! {
            // ~item_base(self.id, &self.label, &self.desc.unwrap_or("\0".into())),
            "id": self.id,
            "label": self.label.as_ptr(),
            "desc": self.desc.clone().unwrap_or("\0".into()).as_ptr(),
            "available": true,
            "active": true,
            "option_type": OptionType::Select as u32,
            "selection": self.selected,
            "value_default": self.default
        };
        let mut opt_array = Value::create_array();
        for opt in &self.options {
            opt_array.push_back(&Value::from(opt.as_str()));
        }
        select_value.set_member("options", &opt_array);
        select_value
    }
}

pub struct Slider {
    pub id: u32,
    pub label: &'static str,
    pub desc: Option<&'static str>,
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub default: f64,
    // pub fidelity: u32,
}

impl OptionItem for Slider {
    unsafe fn to_value(&self) -> Value {
        let slider_value = value::create_value! {
            ~item_base(self.id, self.label, self.desc.unwrap_or("\0")),
            "option_type": OptionType::Slider as u32,
            "value": self.value,
            "value_default": self.default,
            "minimum": self.minimum,
            "maximum": self.maximum
            // "fidelity": self.fidelity
        };
        slider_value
    }
}

#[derive(Clone, Copy)]
pub enum ColourType {
    Normal,
    Null,
    Custom
}

pub struct ColourElement {
    pub colour_index: u32,
    pub colour_type: ColourType,
    pub colour_value: u32,
    pub colour_name: &'static str
}

pub struct Colour {
    pub id: u32,
    pub label: &'static str,
    pub desc: Option<&'static str>,
    pub selected: u32,
    pub default: u32,
    pub colours: Vec<ColourElement>
}

impl OptionItem for Colour {
    unsafe fn to_value(&self) -> Value {
        let mut colour_value = value::create_value! {
            ~item_base(self.id, self.label, self.desc.unwrap_or("\0")),
            "option_type": OptionType::Colour as u32
        };
        let mut colour_data = value::create_value! {
            "selected_colorindex": self.selected,
            "default_colorindex": self.default
        };
        let mut colour_items = Value::create_array();
        for col in &self.colours {
            colour_items.push_back(&value::create_value! {
                "color_index": col.colour_index,
                "color_type": col.colour_type as u32,
                "color_value": col.colour_value,
                "color_name": col.colour_name
            });
        }
        colour_data.set_member("color_list", &colour_items);
        colour_value.set_member("color_data", &colour_data);

        colour_value
    }
}

// pub struct Keybind {
//     pub id: u32,
//     pub label: &'static str,
//     pub desc: Option<&'static str>,
//     pub binding: &'static str,
//     pub is_icon_type: bool,
//     pub locked: bool,
//     pub can_reset: bool
// }

// impl OptionItem for Keybind {
//     unsafe fn to_value(&self) -> Value {
//         let mut keybind_value = value::create_value! {
//             ~item_base(self.id, self.label, self.desc.unwrap_or("\0")),
//             "option_type": OptionType::Keybind as u32 
//         };
//         let mut binding_content = Value::create_array();
//         binding_content.push_back(&value::create_value! {
//             "binding": self.binding,
//             "isIconType": self.is_icon_type,
//             "locked": self.locked,
//             "canResetToDefault": self.can_reset
//         });
//         binding_content.push_back(&value::create_value! {
//             "binding": "{\0",
//             "isIconType": false,
//             "locked": false,
//             "canResetToDefault": true
//         });
//         keybind_value.set_member("bindingContent", &binding_content);
//         keybind_value
//     }
// }