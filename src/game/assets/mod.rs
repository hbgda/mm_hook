mod asset_manager;

use std::{time::Duration};

pub use asset_manager::*;

use crate::{scan_func_static, patterns, utils, logging::Logger};

#[allow(dead_code)]
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
enum Status {
    Loading = 2,
    Loaded = 4
}

#[repr(C)]
pub struct Asset {
    status: Status,
    _0x0: [u8; 0x9],
    path: *const i8
}

impl Asset {
    // Hate everything about this but rust will nuke it during compile otherwise
    pub unsafe fn wait_for_loaded(&self) {
        let ptr = self as *const Asset as *const u8;
        loop {
            let status = std::ptr::read_volatile(ptr);
            Logger::sys_log(format!("Loading Asset: {status}"));
            if status == 4 {
                Logger::sys_log("Loaded Asset".into());
                break;
            }
            // Wait a lil bit
            std::thread::sleep(Duration::from_millis(100))
        }
    }
}

scan_func_static!(patterns::ASSETS_CREATEASSETHASH, CREATE_ASSET_ID(*mut (), *const u8) -> *const ());
pub unsafe fn create_asset_hash(out: *mut (), path: &str) -> *const ()  {
    CREATE_ASSET_ID(out, format!("{path}\0").as_ptr())
}

scan_func_static!(patterns::ASSETS_HASHSTRING, HASH_STRING(*mut u64, *const u8) -> *const u64);
pub unsafe fn hash_string(asset_string: &str) -> u64 {
    let mut hash = 0u64;
    HASH_STRING(&mut hash, format!("{asset_string}\0").as_ptr());
    hash
}

// LOAD_ASSET(asset_manager, hash, unknown, asset_path, unknown, unknown, unknown)
scan_func_static!(patterns::ASSETS_LOADASSET, LOAD_ASSET(*const (), u64, u64, *const u8, u64, u64, u64) -> *const Asset);
pub unsafe fn load_asset<'l>(asset_manager: *const (), hash: u64) -> Option<&'l Asset> {
    Some(&*utils::option_ptr(
        LOAD_ASSET(asset_manager, hash, 0, std::ptr::null(), 0, 0, 0)
    )?)
}