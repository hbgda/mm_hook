use crate::{scan_func_static, utils, patterns};

use super::Asset;

scan_func_static!(patterns::ASSETS_GETASSETMANAGER, GET_ASSET_MANAGER(u8) -> *const ());
scan_func_static!(patterns::ASSETS_LOADASSET, pub(crate) LOAD_ASSET(*const (), u64, u64, *const u8, u64, u64, u64) -> *const Asset);
scan_func_static!(patterns::ASSETS_GETMANAGERBYASSETTYPE, GET_MANAGER_BY_ASSET_TYPE(u32) -> *const ());

#[derive(Clone, Copy)]
pub enum AssetManager {
    LevelManager,
    ZoneManager,
    ActorAssetManager,
    ConduitAssetManager,
    ConfigAssetManager,
    Cinematic2Manager,
    ModelManager,
    AnimClipManager,
    AnimSetManager,
    MaterialManager,
    MaterialTemplateManager,
    TextureManager,
    AtmosphereManager,
    VisualEffectManager,
    SoundBankManager,
    LocalizationAssetManager,
    ZoneCoverManager,
    ModelVariantManager,
    LightGridManager,
    LevelLightManager,
    NodeGraphAssetManager,
    BreakableAssetManager,
    WwiseLookupAssetManager,
    TerrainAssetManager
}

impl AssetManager {
    pub unsafe fn get_ptr(&self) -> Option<*const ()> {
        Some(&*utils::option_ptr(
            GET_ASSET_MANAGER(*self as u8)
        )?)
    }

    pub unsafe fn get_ptr_by_magic<'l>(magic: u32) -> Option<*const ()> {
        Some(&*utils::option_ptr(
            GET_MANAGER_BY_ASSET_TYPE(magic)
        )?)
    }

    pub unsafe fn load_asset(&self, hash: u64) -> Option<&Asset> {
        Some(&*utils::option_ptr(
            LOAD_ASSET(self.get_ptr()?, hash, 0, std::ptr::null(), 0, 0, 0)
        )?)
    }
}