mod asset_manager;

pub use asset_manager::*;

use crate::scan_func_static;

scan_func_static!(crate::patterns::ASSETS_HASHSTRING, HASH_STRING(*mut u64, *const u8) -> *const u64);
pub unsafe fn hash_string(asset_string: &str) -> u64 {
    let mut hash = 0u64;
    HASH_STRING(&mut hash, format!("{asset_string}\0").as_ptr());
    hash
}
// LOAD_ASSET(asset_manager, hash, unknown, asset_path, unknown, unknown, unknown)
scan_func_static!(crate::patterns::ASSETS_LOADASSET, LOAD_ASSET(*const (), u64, u64, u64, u64, u64, u64) -> *const ());
pub unsafe fn load_asset(asset_manager: *const (), hash: u64) -> *const () {
    LOAD_ASSET(asset_manager, hash, 0, 0, 0, 0, 0)
}

pub unsafe fn load_asset_ex(asset_manager: *const (), hash: u64, u1: u64, u2: u64, u3: u64, u4: u64, u5: u64) -> *const () {
    LOAD_ASSET(asset_manager, hash, u1, u2, u3, u4, u5)
}