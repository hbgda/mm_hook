use std::{ffi::CStr, path::Path, collections::HashMap};

use once_cell::sync::Lazy;

use crate::{make_hook, utils, patterns, logging::Logger, game::scaleform};

make_hook!(
    HOOK_ScaleformLoader_OpenFile,
    utils::scan(patterns::SCALEFORMLOADER_OPENFILE).unwrap(),
    (this: *const (), path: *const i8) -> *const () {
        if let Ok(path_str) = CStr::from_ptr(path).to_str() {
            if let Some(idx) = path_str.find("export") {
                let check_path = &path_str[idx + 6..];
                let file_path = format!("./mods/assets/scaleform/{}", check_path.replace("\\", "/").trim_start_matches("/"));
                if Path::new(&file_path).exists() {
                    if let Some(sf) = scaleform::load_custom(&file_path) {
                        Logger::sys_log(format!("custom scaleform: {file_path} -> {:#X}", sf as usize));
                        return sf
                    }
                }
            }
        }
        HOOK_ScaleformLoader_OpenFile.call(this, path)
    }
);

pub unsafe fn init_scaleform() {
    HOOK_ScaleformLoader_OpenFile.enable()
        .expect("Failed to enable hook: ScaleformLoader::OpenFile()");
}