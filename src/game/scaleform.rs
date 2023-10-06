use std::{collections::HashMap, time::Duration};

use once_cell::sync::Lazy;

use crate::{scan_func_static, intercept_static, utils, patterns, logging::Logger};


static mut SCALEFORM_CACHE: Lazy<HashMap<String, *const ()>> = Lazy::new(HashMap::new);
scan_func_static!(patterns::SCALEFORM_OPENFILE_DISC, SCALEFORM_OPENFILE_DISC(*const (), *const u8, u32, u32) -> *const ());
pub unsafe fn load_custom(path: &str) -> Option<*const ()> {
    if let Some(sf) = SCALEFORM_CACHE.get(path) {
        return Some(*sf)
    }
    let sf = SCALEFORM_OPENFILE_DISC(std::ptr::null(), format!("{path}\0").as_ptr(), 1 | 32, 444);
    if sf.is_null() {
        return None
    }
    SCALEFORM_CACHE.insert(path.into(), sf);
    // std::thread::sleep(Duration::from_secs(10));
    Some(sf)
}