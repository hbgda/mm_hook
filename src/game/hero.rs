use crate::utils::get_offset_ptr;

use super::{entity::{Entity, self}, OFFSET_HERO_HANDLE_PTR};

// pub unsafe fn get_hero_entity() -> Entity {
//     let p1: *const u64 = get_offset_ptr(OFFSET_HERO_PTR);
//     let p2: *const u64 = p1.read() as *const u64;
//     let p3: u64 = ((p2.read() + 8) as *const u64).read();
//     Entity(p3)
// }

pub unsafe fn get_hero<'l>() -> Option<&'l Entity> {
    entity::get_entity(get_offset_ptr(OFFSET_HERO_HANDLE_PTR))
}

pub unsafe fn get_hero_mut<'l>() -> Option<&'l mut Entity> {
    entity::get_entity_mut(get_offset_ptr(OFFSET_HERO_HANDLE_PTR))
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