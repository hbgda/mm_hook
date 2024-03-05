use std::{ffi::CStr, fmt};

use crate::{declare_native_func, logging::Logger, patterns, utils};

declare_native_func!(
    utils::scan_func_call(patterns::SF_VALUE_CREATEARRAY).unwrap(),
    VALUE_CREATE_ARRAY(*const (), *mut Value)
);

declare_native_func!(
    utils::scan_func_call(patterns::SF_VALUE_CREATEOBJECT).unwrap(),
    VALUE_CREATE_OBJECT(*const (), *mut Value, *const u8)
);

macro_rules! create_value {
    { $($member:literal : $member_value:expr),* } => {
        {
            let mut value = $crate::game::scaleform::value::Value::create_object();
            $(
                value.set_member($member, &$crate::game::scaleform::value::Value::from($member_value));
            )*
            value
        } 
    };
    { ~$base:expr, $($member:literal : $member_value:expr),* } => {
        {
            let mut new = $base.clone();
            $(
                new.set_member($member, &$crate::game::scaleform::value::Value::from($member_value));
            )*
            new
        }
    };
    [ $item:expr,* ] => {
        {
            let mut value = $crate::game::scaleform::value::Value::create_array();
            $(
                value.push_back(&$crate::game::scaleform::value::Value::from($item));
            )*
            value
        }      
    };
    [ ~$items:expr ] => {
        {
            let mut value = $crate::game::scaleform::value::Value::create_array();
            for item in $items.clone().into_iter() {
                value.push_back(&$crate::game::scaleform::value::Value::from(item));
            }
            value
        }
    }
}
pub(crate) use create_value;

#[repr(C)]
#[derive(Clone)]
pub struct Value {
    _unk_ptr_1: *const (),
    _unk_ptr_2: *const (),
    interface: *const *const ValueInterface,
    value_type: ValueType,
    data: ValueDataUnion,
    _0x28: [u8; 0x2]
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value")
            .field("type", &self.value_type)
            .field("value", &match &self.value_type {
                ValueType::UInt => format!("{}", self.get_uint()),
                ValueType::Int => format!("{}", self.get_int()),
                ValueType::Boolean => format!("{}", self.get_bool()),
                ValueType::Number => format!("{}", self.get_number()),
                _ => {
                    unsafe { format!("{:#X}", self.data.object as u64) }
                }
            })
            .finish()
    }
}

#[repr(C)]
pub struct ValueInterface {
    pub fn_1: *const (),
    pub fn_2: *const (),
    pub drop: fn(*const (), *const Value, *const ()),
    pub fn_4: *const (),
    // 0x20 GetMember(?, self.data, member, result)
    pub get_member: fn(*const (), *const (), *const u8, *mut Value) -> bool,
    // 0x28 SetMember(?, self.data, member, new)
    pub set_member: fn(*const (), *mut (), *const u8, *const Value) -> bool,
    // 0x30 Invoke(?, self.data, result, method, args, num_args)
    pub invoke:     fn(*const (), *const (), *mut Value, *const u8, *const Value, u32) -> bool,
    _0x38: [u8; 0x40],
    // 0x78 PushBack(?, self.data, value)
    pub push_back:  fn(*const (), *const (), *const Value) -> bool,
    _0x80: [u8; 0xC8],
    // 0x148 AttachMovie(?, self.data, result, symbol_name, instance_name, depth, init_args)
    pub attach_movie: fn(*const (), *const (), *mut Value, *const u8, *const u8, i32, *const ()) -> bool
}

impl Value {
    const fn alloc() -> Value {
        Value {
            _unk_ptr_1: std::ptr::null(),
            _unk_ptr_2: std::ptr::null(),
            interface: std::ptr::null(),
            value_type: ValueType::Undefined,
            data: ValueDataUnion { bool: false },
            _0x28: [0u8; 0x2]
        }
    }

    pub unsafe fn interface(&self) -> Option<&ValueInterface> {
        if !self.interface.is_null() {
            return Some(&**self.interface)
        }
        None
    }

    pub unsafe fn drop(&mut self) {
        if self.is_managed() {
            let interface = match self.interface() {
                Some(interface) => interface,
                None => return
            };
            // let obj_ref_count = self.get_data() as u64 + 0x20;
            // let ref_count = std::ptr::read(obj_ref_count as *const u16);
            // Logger::sys_log(format!("Ref Count: {ref_count}"));

            (interface.drop)(self.interface as *const (), self, self.get_data());
            self.interface = std::ptr::null();
            
            // let ref_count = std::ptr::read(obj_ref_count as *const u16);
            // Logger::sys_log(format!("Ref Count: {ref_count}"));
        }
    }

    pub fn is_managed(&self) -> bool {
        self.value_type as u8 & ValueTypeControl::ManagedBit as u8 != 0
    }

    pub fn get_type(&self) -> ValueType {
        unsafe {
            std::mem::transmute(self.value_type as u8 & ValueTypeControl::TypeMask as u8)
        }
    }

    pub unsafe fn push_back(&mut self, v: &Value) -> bool {
        let interface = match self.interface() {
            Some(interface) => interface,
            None => return false
        };
        (interface.push_back)(self.interface as *const (), self.get_data(), v)
    }

    pub unsafe fn get_member(&self, member: &str) -> Option<Value> {
        let mut out = Value::alloc();
        let interface = self.interface()?;
        if (interface.get_member)(self.interface as *const (), self.get_data(), format!("{member}\0").as_ptr(), &mut out) {
            return Some(out)
        }
        None
    }

    pub unsafe fn get_nested_member(&self, path: &str) -> Option<Value> {
        let mut parts = path.split(".");
        let mut member_value = self.get_member(parts.nth(0)?)?;
        for member in parts {
            member_value = member_value.get_member(member)?;
        }
        Some(member_value)
    }

    pub unsafe fn set_member(&mut self, member: &str, new_val: &Value) -> bool {
        let interface = match self.interface() {
            Some(interface) => interface,
            None => return false 
        };
        (interface.set_member)(self.interface as *const (), self.get_data() as *mut (), format!("{member}\0").as_ptr(), new_val)
    }

    pub unsafe fn set_nested_member(&mut self, path: &str, new_val: &Value) -> bool {
        let mut members: Vec<Value> = Vec::new();
        
        let mut parts: Vec<_> = path.split(".").collect();
        let final_member = parts.pop().unwrap();
        
        for member in parts.clone() {
            let member_value = match members.last().unwrap_or(self).get_member(member) {
                Some(value) => value,
                None => return false
            };
            members.push(member_value);
        }

        let mut last = members.pop().unwrap();
        last.set_member(final_member, new_val);
        for _ in 0..members.len() {
            let mut member_value = members.pop().unwrap();
            if !member_value.set_member(parts.pop().unwrap(), &last) {
                return false
            }
            last = member_value
        }

        self.set_member(parts.pop().unwrap(), &last)
    }

    pub fn get_data(&self) -> *const () { unsafe { self.data.object } }
    pub fn get_bool(&self) -> bool { unsafe { self.data.bool } }
    pub fn get_int(&self) -> i32  { unsafe { self.data.int } }
    pub fn get_uint(&self) -> u32  { unsafe { self.data.uint } }
    pub fn get_number(&self) -> f64 { unsafe { self.data.number } }
    pub fn get_string(&self) -> Option<&str> { 
        let str_ptr = if self.is_managed() {
            unsafe { *self.data.managed_string }
        }
        else {
            unsafe { self.data.string }
        };

        if let Ok(string) = unsafe { CStr::from_ptr(str_ptr) }.to_str() {
            return Some(string)
        }
        None
    }
    
    fn set_type(&mut self, t: ValueType) { self.value_type = t }
    pub fn set_bool(&mut self, v: bool) { self.set_type(ValueType::Boolean); self.data.bool = v }
    pub fn set_int(&mut self, v: i32) { self.set_type(ValueType::Int); self.data.int = v }
    pub fn set_uint(&mut self, v: u32) { self.set_type(ValueType::UInt); self.data.uint = v }
    pub fn set_number(&mut self, v: f64) { self.set_type(ValueType::Number); self.data.number = v }
    pub fn set_string(&mut self, v: &str) { self.set_string_ptr(v.as_ptr()) }
    pub fn set_string_ptr(&mut self, v: *const u8) { self.set_type(ValueType::String); self.data.string = v as *const i8 }
}

impl Value {
    pub unsafe fn create_array() -> Value {
        let mut value = Value::alloc();
        VALUE_CREATE_ARRAY(std::ptr::null(), &mut value);
        value
    }

    pub unsafe fn create_object() -> Value {
        Value::create_object_of_type("Object\0")
    }

    pub unsafe fn create_object_of_type(type_name: &str) -> Value {
        let mut value = Value::alloc();
        VALUE_CREATE_OBJECT(std::ptr::null(), &mut value, type_name.as_ptr());
        value
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        let mut v = Value::alloc();
        v.set_bool(value);
        v
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        let mut v = Value::alloc();
        v.set_int(value);
        v
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        let mut v = Value::alloc();
        v.set_uint(value);
        v
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        let mut v = Value::alloc();
        v.set_number(value);
        v
    }
}

impl From<*const u8> for Value {
    fn from(value: *const u8) -> Self {
        let mut v = Value::alloc();
        v.set_string_ptr(value);
        v
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let mut v = Value::alloc();
        v.set_string(value);
        v
    }
} 

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueType {
    Undefined,
    Null,
    Boolean,
    Int,
    UInt,
    Number,
    String,
    StringW,
    Object,
    Array,
    DisplayObject,
    Closure,
    ConvertBoolean = ValueTypeControl::ConvertBit as u8 | ValueType::Boolean as u8,
    ConvertInt = ValueTypeControl::ConvertBit as u8 | ValueType::Int as u8,
    ConvertUInt = ValueTypeControl::ConvertBit as u8 | ValueType::UInt as u8,
    ConvertNumber = ValueTypeControl::ConvertBit as u8 | ValueType::Number as u8,
    ConvertString = ValueTypeControl::ConvertBit as u8 | ValueType::String as u8,
    ConvertStringW = ValueTypeControl::ConvertBit as u8 | ValueType::StringW as u8
}

#[repr(u8)]
pub enum ValueTypeControl {
    ConvertBit = 0x80,
    ManagedBit = 0x40,
    TypeMask = ValueTypeControl::ConvertBit as u8 | 0x0F
}

#[derive(Clone, Copy)]
pub union ValueDataUnion {
    bool: bool,
    int: i32,
    uint: u32,
    number: f64,
    string: *const i8,
    managed_string: *const *const i8,
    stringw: *const i16,
    object: *const (),
}

// Don't think any of this is necessary
// pub trait IsValueType {
//     fn get_type() -> ValueType;
// }

// impl IsValueType for bool {
//     fn get_type() -> ValueType {
//         ValueType::Boolean
//     }
// }

// impl IsValueType for i32 {
//     fn get_type() -> ValueType {
//         ValueType::Int
//     }
// }

// impl IsValueType for u32 {
//     fn get_type() -> ValueType {
//         ValueType::UInt
//     }
// }

// impl IsValueType for &str {
//     fn get_type() -> ValueType {
//         ValueType::String
//     }
// }
