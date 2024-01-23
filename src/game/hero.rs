use once_cell::sync::Lazy;

use crate::{patterns, utils};

use super::actor::{self, Actor};

const HEROSYSTEM: Lazy<Option<*const HeroSystem>> = Lazy::new(|| unsafe {
    let mut scanner = match utils::create_scanner(patterns::HEROSYSTEM_OFFSET) {
        Ok(scanner) => scanner,
        Err(_) => return None
    };
    let found = scanner.next()?;
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    Some((found as isize + 7 + offset as isize) as *const HeroSystem)
});

#[repr(C)]
pub struct HeroSystem {
    _0x0: [u8; 0x1C],
    hero_handle: u32
}

impl HeroSystem {
    pub unsafe fn get() -> Option<&'static HeroSystem> {
        Some(
            &*utils::option_ptr(
                (*HEROSYSTEM)?
            )?
        )
    }

    pub unsafe fn get_mut() -> Option<&'static mut HeroSystem> {
        Some(
            &mut *(utils::option_ptr(
                (*HEROSYSTEM)?
            )? as *mut HeroSystem)
        )
    }

    pub unsafe fn hero_handle(&self) -> u32 {
        self.hero_handle
    }

    pub unsafe fn get_hero(&self) -> Option<&Actor> {
        actor::get_actor(&self.hero_handle)
    }

    pub unsafe fn get_hero_mut(&mut self) -> Option<&mut Actor> {
        actor::get_actor_mut(&self.hero_handle)
    }
}