use crate::scan_func_static;

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

scan_func_static!(crate::patterns::ASSETS_GETASSETMANAGER, GET_ASSET_MANAGER(u8) -> *const ());
pub unsafe fn get_asset_manager(asset_manager: AssetManager) -> *const () {
    GET_ASSET_MANAGER(asset_manager as u8)
}