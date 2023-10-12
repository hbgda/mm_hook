use std::time::Duration;

use crate::{scan_func_static, patterns};

use super::get_hud;


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
    CREATE_MESSAGE(hud.hud_message, message_type.into(), message.as_ptr(), 0, 0, 0, 0, 1)
}

pub unsafe fn clear_message(message: &'static str, message_type: MessageType) {
    let hud = &*get_hud().unwrap();
    CLEAR_MESSAGE(hud.hud_message, message_type.into(), message.as_ptr(), 0);
}

#[derive(Clone)]
pub enum MessageType {
    LeftCenter,         // 3
    BottomLeft,         // 11
    CenterLower,        // 14
    CenterBox,          // 17
    CenterUpper,        // 18
    LeftBox,            // 20
    ShowFNSMReminder    // 21
}

impl Into<u32> for MessageType {
    fn into(self) -> u32 {
        match self {
            Self::LeftCenter => 3,
            Self::BottomLeft => 11,
            Self::CenterLower => 14,
            Self::CenterBox => 17,
            Self::CenterUpper => 18,
            Self::LeftBox => 20,
            Self::ShowFNSMReminder => 21,
        }
    }
}
