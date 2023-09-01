#[repr(C)]
#[derive(Eq, PartialEq, Hash)]
pub struct Keybind {
    pub info: *const KeybindInfo,
    pub default_primary: u32,
    pub default_secondary: u32,
    pub locked: bool,
    pub group: u32,
    pub name: *const u8,
    pub description: *const u8,
    _0x28: u32,
    _0x2c: u32
}

#[repr(C)]
pub struct KeybindInfo {
    _0x0: [u8; 0xA3],
    pub primary_key: u32,
    pub secondary_key: u32,
    pub pressed: bool,
    unknown: u32 // Possibly related to held time?
}