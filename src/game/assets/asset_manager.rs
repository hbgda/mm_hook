use crate::{scan_func_static, utils, patterns};

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

scan_func_static!(patterns::ASSETS_GETASSETMANAGER, GET_ASSET_MANAGER(u8) -> *const ());
pub unsafe fn get_asset_manager(asset_manager: AssetManager) -> Option<*const ()> {
    utils::option_ptr(GET_ASSET_MANAGER(asset_manager as u8))
}

scan_func_static!(patterns::ASSETS_GETMANAGERBYASSETTYPE, GET_MANAGER_BY_ASSET_TYPE(u32) -> *const ());
pub unsafe fn get_asset_manager_by_type(magic: u32) -> Option<*const ()> {
    utils::option_ptr(GET_MANAGER_BY_ASSET_TYPE(magic))
}