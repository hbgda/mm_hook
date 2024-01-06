use crate::utils::get_offset_ptr;

use super::{entity::{Entity, self}, OFFSET_HERO_HANDLE_PTR};

pub unsafe fn get_hero<'l>() -> Option<&'l Entity> {
    entity::get_entity(get_offset_ptr(OFFSET_HERO_HANDLE_PTR))
}

pub unsafe fn get_hero_mut<'l>() -> Option<&'l mut Entity> {
    entity::get_entity_mut(get_offset_ptr(OFFSET_HERO_HANDLE_PTR))
}