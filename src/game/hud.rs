use std::time::Duration;

use super::OFFSET_PLAYERHUDMESSAGE;
use crate::{make_func_static, scan_func_static};

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

scan_func_static!(crate::patterns::HUD_CREATEMESSAGE, CREATE_MESSAGE(u64, u32, *const u8, u32, u32, u32, u8, u8): u64);
scan_func_static!(crate::patterns::HUD_CLEARMESSAGE, CLEAR_MESSAGE(u64, u32, *const u8, u32): u32);

// make_func_static!(0x8E5890, CREATE_MESSAGE(u64, u32, *const u8, u32, u32, u32, u8, u8): u64);
// make_func_static!(0x8E6AB0, CLEAR_MESSAGE(u64, u32, *const u8, u32): u32);

pub unsafe fn show_message(message: &'static str, message_type: MessageType, duration: Option<Duration>) {
    create_message(message, message_type.clone());
    if let Some(duration) = duration {
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            clear_message(message, message_type);
        });
    }
}

pub unsafe fn create_message(message: &'static str, message_type: MessageType) {
    let phm = crate::utils::get_offset(OFFSET_PLAYERHUDMESSAGE) as u64;
    CREATE_MESSAGE(phm, message_type.into(), message.as_ptr(), 0, 0, 0, 0, 1);
}

pub unsafe fn clear_message(message: &'static str, message_type: MessageType) {
    let phm = crate::utils::get_offset(OFFSET_PLAYERHUDMESSAGE) as u64;
    CLEAR_MESSAGE(phm, message_type.into(), message.as_ptr(), 0);
}