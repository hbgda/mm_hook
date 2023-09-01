use super::get_player_hud;

pub unsafe fn create_popup(header: &'static str, message: &'static str) {
    let hud = &*get_player_hud().unwrap();

    hud.hide(1, 1, 0.0);
    
}