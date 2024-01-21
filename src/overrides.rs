use std::{ffi::CStr, path::Path};
use crate::{make_hook, utils, patterns, logging::Logger, game::{scaleform, assets::{hash_string, Asset}}};

static mut CACHED: Vec<u64> = Vec::new();
make_hook!(
    HOOK_ScaleformLoader_OpenFile,
    utils::scan(patterns::SCALEFORMLOADER_OPENFILE).unwrap(),
    (this: *const (), path: *const i8) -> *const () {
        if let Ok(path_str) = CStr::from_ptr(path).to_str() {
            // let hash = hash_string(path_str);
            // if CACHED.contains(&hash) {
            //     Logger::sys_log(format!("Cached scaleform: {hash:#X}"));
            //     return HOOK_ScaleformLoader_OpenFile.call(this, path);
            // }

            if let Some(idx) = path_str.find("export") {
                let check_path = &path_str[idx + 6..];
                let file_path = format!("./mods/assets/scaleform/{}", check_path.replace("\\", "/").trim_start_matches("/"));
                if Path::new(&file_path).exists() {
                    if let Some(sf) = scaleform::load_custom(&file_path) {
                        Logger::sys_log(format!("custom scaleform: {file_path} -> {:#X}", sf as usize));
                        // CACHED.push(hash);
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

// TWO METHODS:
// Figure out what writes to Asset.Status (cleaner)
// Write Status::Loaded to Asset.Status and read asset data manually (jank)
// make_hook!(
//     HOOK_AssetManager_LoadAsset,
//     utils::scan(patterns::ASSETS_LOADASSET).unwrap(),
//     (manager: *const (), hash: u64, unk: u64, name: *const u8, unk1: u64, unk2: u64, unk3: u64) -> *const Asset {
//         // let path = Path::new(&format!("./mods/assets/{hash:#X}"));
//         if path.exists() {
            
//         }
//         HOOK_AssetManager_LoadAsset.call(manager, hash, unk, name, unk1, unk2, unk3)
//     }
// );

// pub unsafe fn init_asset_loader() {
//     HOOK_AssetManager_LoadAsset.enable()
//         .expect("Failed to enable hook: AssetManager::LoadAsset()");
// }