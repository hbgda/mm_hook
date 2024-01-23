pub mod value;

use crate::{declare_native_func, patterns, utils};

// scan_func_static!(patterns::SCALEFORM_INVOKE, SCALEFORM_INVOKE(*const (), *const (), *const u8, *mut (), *const (), u32) -> bool);
// pub unsafe fn invoke(value: *const (), event: &str, result: *mut (), args: *const (), num_args: u32) -> bool {
//     SCALEFORM_INVOKE(std::ptr::null(), value, event.as_ptr(), result, args, num_args)
// }

declare_native_func!(
    utils::scan(patterns::SCALEFORM_OPENFILE_DISC).unwrap(), 
    SCALEFORM_OPENFILE_DISC(*const (), *const u8, u32, u32) -> *const ()
);

pub unsafe fn load_custom(path: &str) -> Option<*const ()> {
    let sf = SCALEFORM_OPENFILE_DISC(std::ptr::null(), format!("{path}\0").as_ptr(), 1 | 32, 444);
    if sf.is_null() {
        return None
    }
    Some(sf)
}