pub mod hero;
pub mod entity;
pub mod hud;
pub mod transform;
pub mod nx;

// TODO: Pattern Scanning for offsets
pub(crate) const OFFSET_HERO_HANDLE_PTR: isize = 0x66EAE2C;
pub(crate) const OFFSET_PLAYERHUD: isize = 0x77D77B0;
pub(crate) const OFFSET_PLAYERHUDMESSAGE: isize = 0x777D7E0;