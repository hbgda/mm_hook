#[repr(C)]
pub struct UISystemMenuConfig {
    _0x0: [u8; 0x5F],
    pub item_list: *const [UISystemMenuItem; 8],
    pub item_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UISystemMenuItem {
    vftable: *const (),
    pub number: u8,
    pub title: *const u8,
    _pad1: [u8; 0x12],
    pub description: *const u8,
    _pad2: [u8; 0x12],
    pub image: *const u8,
    _pad3: [u8; 0x10],
    pub option_type_cmd: *const (),
    _pad4: [u8; 0x38],
}