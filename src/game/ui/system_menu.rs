#[repr(C)]
pub struct UISettingsMenuSystem {
    _0x0: [u8; 0x20],
    pub system_menu_cfg: *const UISystemMenuConfig
}

#[repr(C)]
pub struct UISystemMenuConfig {
    _0x0: [u8; 0x5F],
    /// 0x60
    pub item_list: *const [UISystemMenuItem; 0],
    /// 0x68
    pub item_count: u32,
}

#[repr(C)]
pub struct UISystemMenu {
    vftable: *const (),
    _pad1: u8,
    /// 0x10
    pub title: *const u8,
    _pad2: u8,
    /// 0x20
    pub header: *const u8,
    _pad3: [u8; 0x10],
    /// 0x38
    /// 0 sized as the length is different between instances,
    /// make sure to cast to a correctly sized array when using
    pub item_list: *const [UISystemMenuItem; 0],
    /// 0x40
    pub item_count: u32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UISystemMenuItem {
    vftable: *const (),
    /// 0x8
    pub id: u8,
    /// 0x10
    pub title: *const u8,
    _pad1: [u8; 0x12],
    /// 0x30
    pub description: *const u8,
    _pad2: [u8; 0x12],
    /// 0x50
    pub image: *const u8,
    _pad3: [u8; 0x10],
    /// 0x68
    pub option_type: *const (),
    _pad4: [u8; 0x38],
}