use crate::{load_library_func, ModInfo};

pub struct Logger {
    pub mod_info: ModInfo
}

// const LOG: crate::Lazy<extern "system" fn(*const u8)> = crate::Lazy::new(|| unsafe {
//     std::mem::transmute::<_, extern "system" fn(*const u8)>(crate::GetProcAddress(
//         crate::HMODULE(crate::GetModuleHandleA(crate::s!("mm_hook_debugging")).unwrap().0),
//         crate::s!("Log")
//     ).unwrap())
// });

load_library_func!("mm_hook_debugging", "Log", LOG(ModInfo, String));
// load_library_func!("mm_hook_debuging", "NotifyMod", NOTIFY_MOD(*const CModInfo));

impl Logger {
    const MMHOOK_INFO: ModInfo = ModInfo {
        title: "mm_hook",
        version: "",
        author: "L"
    };

    pub fn log(&self, msg: String) {
        if let Some(log_fn) = &*LOG {
            log_fn(self.mod_info.clone(), msg);
        }
    }

    pub(crate) fn sys_log(msg: String) {
        if let Some(log_fn) = &*LOG {
            log_fn(Logger::MMHOOK_INFO, msg);
        }
    }
}