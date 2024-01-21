pub mod value;

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{scan_func_static, patterns, logging::Logger};

scan_func_static!(patterns::SCALEFORM_INVOKE, SCALEFORM_INVOKE(*const (), *const (), *const u8, *mut (), *const (), u32) -> bool);
pub unsafe fn invoke(value: *const (), event: &str, result: *mut (), args: *const (), num_args: u32) -> bool {
    SCALEFORM_INVOKE(std::ptr::null(), value, event.as_ptr(), result, args, num_args)
}

static mut SCALEFORM_CACHE: Lazy<HashMap<String, *const ()>> = Lazy::new(HashMap::new);
scan_func_static!(patterns::SCALEFORM_OPENFILE_DISC, SCALEFORM_OPENFILE_DISC(*const (), *const u8, u32, u32) -> *const ());
pub unsafe fn load_custom(path: &str) -> Option<*const ()> {
    // if let Some(sf) = SCALEFORM_CACHE.get(path) {
    //     Logger::sys_log("Using cached scaleform.".into());
    //     return Some(*sf)
    // }
    let sf = SCALEFORM_OPENFILE_DISC(std::ptr::null(), format!("{path}\0").as_ptr(), 1 | 32, 444);
    if sf.is_null() {
        return None
    }
    // SCALEFORM_CACHE.insert(path.into(), sf);
    Some(sf)
}