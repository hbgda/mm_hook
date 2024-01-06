use std::{time::Duration, sync::atomic::AtomicUsize};

use crate::{scan_func_static, patterns, utils, make_func_static, message_box, logging::Logger};

use super::get_hud;

/*
fn ShowNotification(phm, style, label, subtitle, unk, unk, unk, delay, duration?, unk)
*/
// make_func_static!(0x8eb8b0, SHOW_NOTIFICATION(*const PlayerHudMessage, u32, *const u8, *const u8, u32, u32, u32, f32, f64, u32) -> u8);
// pub unsafe fn show_notification(label: &'static str, subtitle: &'static str, notif_type: NotificationType) -> Result<u8, String> {
//     let hud = match get_hud() {
//         Some(hud) => hud,
//         None => return Err("get_hud() is None".into())
//     };

//     Ok(SHOW_NOTIFICATION(
//         hud.hud_message,
//         18,
//         label.as_ptr(),
//         subtitle.as_ptr(),
//         0, 0, 0, 0.0, -10.0, 0
//     ))
// }

// pub enum NotificationType {
//     LevelUp = 0,
//     FocusGain = 17,
//     MaxHpUp = 18
// }

scan_func_static!(patterns::HUD_CREATEMESSAGE, CREATE_MESSAGE(*const (), u32, *const u8, u32, u32, u32, u8, u8) -> u64);
scan_func_static!(patterns::HUD_CLEARMESSAGE, CLEAR_MESSAGE(*const (), u32, *const u8, u32) -> u32);

pub unsafe fn show_message(message: &'static str, message_type: MessageType, duration: Option<Duration>) {
    create_message(message, message_type.clone());
    if let Some(duration) = duration {
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            clear_message(message, message_type);
        });
    }
}

pub unsafe fn create_message(message: &'static str, message_type: MessageType) -> u64 {
    let hud = &*get_hud().unwrap();
    CREATE_MESSAGE(hud.hud_message, message_type as u32, message.as_ptr(), 0, 0, 0, 0, 1)
}

pub unsafe fn clear_message(message: &'static str, message_type: MessageType) {
    let hud = &*get_hud().unwrap();
    CLEAR_MESSAGE(hud.hud_message, message_type as u32, message.as_ptr(), 0);
}

#[derive(Clone)]
pub enum MessageType {
    LeftCenter = 3,         // 3
    BottomLeft = 11,         // 11
    CenterLower = 14,        // 14
    CenterBox = 17,          // 17
    CenterUpper = 18,        // 18
    LeftBox = 20,            // 20
}