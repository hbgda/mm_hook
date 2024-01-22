use std::ffi::CStr;

#[repr(C)]
pub struct Value {
    _0x0: [u8; 0x10],
    interface: *const *const ValueInterface,
    value_type: ValueType,
    data: ValueDataUnion,
    _0x28: [u8; 0x2]
}

#[repr(C)]
struct ValueInterface {
    pub fn_1: *const (),
    pub fn_2: *const (),
    pub fn_3: *const (),
    pub fn_4: *const (),
    // GetMember(self, member, result)
    pub get_member: fn(*const Value, *const u8, *mut Value, bool) -> bool,
    // SetMember(self, member, new)
    pub set_member: fn(*mut Value, *const u8, *const Value, bool) -> bool,
    // Invoke(self, method, result, args, num_args)
    pub invoke:     fn(*const Value, *const u8, *mut Value, *const Value, u32, bool) -> bool,
    _0x38: [u8; 0x110],
    // // Assuming this is AttachMovie based on strings in the function, but unsure
    // // Ghidra shows this function as having 6 args (excluding self) but docs say 5, so idk ??
    // // AttachMovie(self, result, symbol_name, instance_name, depth, init_args)
    // pub attach_movie: fn(*const Value, *mut Value, *const u8, *const u8, i32, *const ()) -> bool,
}

impl Value {
    pub const fn alloc() -> Value {
        Value {
            _0x0: [0u8; 0x10],
            interface: std::ptr::null(),
            value_type: ValueType::Undefined,
            data: ValueDataUnion { bool: false },
            _0x28: [0u8; 0x2]
        }
    }

    unsafe fn interface(&self) -> &ValueInterface {
        &**self.interface
    }

    pub unsafe fn is_managed(&self) -> bool {
        self.value_type as u8 & ValueTypeControl::ManagedBit as u8 != 0
    }

    pub unsafe fn get_type(&self) -> ValueType {
        std::mem::transmute(self.value_type as u8 & ValueTypeControl::TypeMask as u8)
    }

    pub unsafe fn get_member(&self, member: &str, ) -> Option<Value> {
        let mut out = Value::alloc();
        let interface = self.interface();
        if (interface.get_member)(self, format!("{member}\0").as_ptr(), &mut out, self.is_managed()) {
            return Some(out)
        }
        None
    }

    pub unsafe fn set_member(&mut self, member: &str, new_val: *const Value) -> bool {
        let interface = self.interface();
        (interface.set_member)(self, format!("{member}\0").as_ptr(), new_val, self.is_managed())
    }

    pub unsafe fn get_data(&self) -> *const () { self.data.object }
    pub unsafe fn get_bool(&self) -> bool { self.data.bool }
    pub unsafe fn get_int(&self) -> i32  { self.data.int }
    pub unsafe fn get_uint(&self) -> u32  { self.data.uint }
    pub unsafe fn get_number(&self) -> f64 { self.data.number }
    pub unsafe fn get_string(&self) -> Option<&str> { 
        let str_ptr = if self.is_managed() {
            *self.data.managed_string
        }
        else {
            self.data.string
        };

        if let Ok(string) = CStr::from_ptr(str_ptr).to_str() {
            return Some(string)
        }
        None
    }
    
    unsafe fn set_type(&mut self, t: ValueType) { self.value_type = t }
    pub unsafe fn set_bool(&mut self, v: bool) { self.set_type(ValueType::Boolean); self.data.bool = v }
    pub unsafe fn set_int(&mut self, v: i32) { self.set_type(ValueType::Int); self.data.int = v }
    pub unsafe fn set_uint(&mut self, v: u32) { self.set_type(ValueType::UInt); self.data.uint = v }
    pub unsafe fn set_number(&mut self, v: f64) { self.set_type(ValueType::Number); self.data.number = v }
    pub unsafe fn set_string(&mut self, v: &str) { self.set_type(ValueType::String); self.data.string = v.as_ptr() as *const i8 }
}

#[repr(u8)]
#[derive(Clone, Copy)]
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

pub union ValueDataUnion {
    bool: bool,
    int: i32,
    uint: u32,
    number: f64,
    string: *const i8,
    managed_string: *const *const i8,
    stringw: *const i16,
    object: *const ()
}

pub trait IsValueType {
    fn get_type() -> ValueType;
}

impl IsValueType for bool {
    fn get_type() -> ValueType {
        ValueType::Boolean
    }
}

impl IsValueType for i32 {
    fn get_type() -> ValueType {
        ValueType::Int
    }
}

impl IsValueType for u32 {
    fn get_type() -> ValueType {
        ValueType::UInt
    }
}

impl IsValueType for &str {
    fn get_type() -> ValueType {
        ValueType::String
    }
}