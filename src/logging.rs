use crate::{CModInfo, load_library_func};

pub struct Logger {
    mod_info: CModInfo
}

// const LOG: crate::Lazy<extern "system" fn(*const u8)> = crate::Lazy::new(|| unsafe {
//     std::mem::transmute::<_, extern "system" fn(*const u8)>(crate::GetProcAddress(
//         crate::HMODULE(crate::GetModuleHandleA(crate::s!("mm_hook_debugging")).unwrap().0),
//         crate::s!("Log")
//     ).unwrap())
// });

load_library_func!("mm_hook_debugging", "Log", LOG(*const CModInfo, *const u8));

impl Logger {
    pub fn log(&self, msg: &str) {
        LOG(&self.mod_info, msg.as_ptr());
    }
}