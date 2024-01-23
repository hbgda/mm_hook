mod asset_manager;

use std::{time::Duration};

pub use asset_manager::*;

use crate::{declare_native_func, logging::Logger, patterns, utils};

declare_native_func!(
    utils::scan(patterns::ASSETS_HASHSTRING).unwrap(),
    HASH_STRING(*mut u64, *const u8) -> *const u64
);

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

// Not positive what the difference between these two functions are but they both create asset hashes ?

// scan_func_static!(patterns::ASSETS_CREATEASSETHASH, CREATE_ASSET_ID(*mut u64, *const u8) -> *const u64);
// pub unsafe fn create_asset_hash(path: &str) -> u64  {
//     let mut hash = 0u64;
//     CREATE_ASSET_ID(&mut hash, format!("{path}\0").as_ptr());
//     hash
// }

// scan_func_static!(patterns::ASSETS_HASHSTRING, HASH_STRING(*mut u64, *const u8) -> *const u64);
pub unsafe fn hash_string(asset_string: &str) -> u64 {
    let mut hash = 0u64;
    HASH_STRING(&mut hash, format!("{asset_string}\0").as_ptr());
    hash
}

