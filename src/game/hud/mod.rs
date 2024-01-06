use crate::{scan_func_static, utils, patterns};

pub mod message;

// intercept_static!(
//     PLAYER_HUD: *const PlayerHUD,
//     HOOK_HUD_CreatePlayerHUD_Intercept,
//     utils::scan(crate::patterns::HUD_CREATEPLAYERHUD).unwrap(),
//     [ hud ]
//     (hud: *const PlayerHUD, unk: i32) -> *const PlayerHUD
// );

#[repr(C)]
pub struct PlayerHUD {
    _0x0: [u8; 0x18F],
    pub hud_ammo: *const (),
    pub hud_reticule: *const (),
    pub hud_message: *const (),
    pub hud_poi: *const (),
    pub hud_progress_bar: *const (),
    pub hud_quick_select: *const (),
    pub hud_health: *const (),
    pub hud_app: *const (),
    pub hud_target_lock: *const (),
    pub hud_interact_button: *const (),
    pub hud_activities: *const (),
    pub hud_qte: *const (),
    pub hud_spider_assault: *const (),
    pub hud_combo_meter: *const (),
    pub hud_countdown: *const (),
    pub hud_mission: *const (),
    pub hud_counter: *const (),
    pub hud_inventory: *const (),
    pub hud_pip_clue: *const (),
    pub hud_placeable_button_prompt: *const (),
    pub hud_communicator: *const (),
    pub hud_eavesdrop_monitor: *const (),
    pub hud_photo_mode: *const (),
    pub hud_prowler_collectible: *const (),
    pub hud_map: *const ()
}

scan_func_static!(patterns::HUD_HIDEHUD, HIDE_HUD(*const PlayerHUD, u32, u32, f32));
scan_func_static!(patterns::HUD_GETHUD, GETHUD(u32) -> *const PlayerHUD);

pub unsafe fn get_hud<'l>() -> Option<&'l PlayerHUD> {
    Some(&*utils::option_ptr(GETHUD(0))?)
}

impl PlayerHUD {
    pub unsafe fn hide(&self, u0: u32, u1: u32, u2: f32) {
        HIDE_HUD(self, u0, u1, u2);
    }
}