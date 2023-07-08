use crate::{make_type, get_offset_ptr};

use super::{OFFSET_HERO_PTR, entity::Entity};

pub unsafe fn get_hero_entity() -> Entity {
    let p1: *const u64 = get_offset_ptr(OFFSET_HERO_PTR);
    let p2: *const u64 = p1.read() as *const u64;
    let p3: u64 = ((p2.read() + 8) as *const u64).read();
    Entity(p3)
}

#[repr(C)]
pub struct HeroHealth {
    _0x0: [u8; 0x7F],
    // 0x80
    pub max_health: f32,
    _0x84: u32,
    // 0x88
    pub current_health: f32
}

make_type!(
    _HeroHealth,
    [
        0x80 => max_health: u64,
        0x88 => current_health: u64
    ]
);

make_type!(
    Test,
    [
        0x1 => var: u64
    ],
    0x0 => init(this: u64): ()
);

make_type!(
    TestFns,
    0x0 => init(this: u64): ()
);