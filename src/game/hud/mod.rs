use crate::{intercept_static, scan_func_static};

pub mod message;

intercept_static!(
    PLAYER_HUD: *const PlayerHUD,
    HOOK_HUD_CreatePlayerHUD_Intercept,
    crate::utils::scan(crate::patterns::HUD_CREATEPLAYERHUD).unwrap(),
    [ hud ]
    (hud: *const PlayerHUD, unk: i32) -> *const PlayerHUD
);

#[repr(C)]
pub struct PlayerHUD {
    _0x0: [u8; 0x18F],
    hud_ammo: *const (),
    hud_reticule: *const (),
    hud_message: *const (),
    hud_poi: *const (),
    hud_progress_bar: *const (),
    hud_quick_select: *const (),
    hud_health: *const (),
    hud_app: *const (),
    hud_target_lock: *const (),
    hud_interact_button: *const (),
    hud_activities: *const (),
    hud_qte: *const (),
    hud_spider_assault: *const (),
    hud_combo_meter: *const (),
    hud_countdown: *const (),
    hud_mission: *const (),
    hud_counter: *const (),
    hud_inventory: *const (),
    hud_pip_clue: *const (),
    hud_placeable_button_prompt: *const (),
    hud_communicator: *const (),
    hud_eavesdrop_monitor: *const (),
    hud_photo_mode: *const (),
    hud_prowler_collectible: *const (),
    hud_map: *const ()
}

scan_func_static!(crate::patterns::HUD_HIDEHUD, HIDE_HUD(*const PlayerHUD, u32, u32, f32));

impl PlayerHUD {
    pub unsafe fn hide(&self, u0: u32, u1: u32, u2: f32) {
        HIDE_HUD(self, u0, u1, u2);
    }
}