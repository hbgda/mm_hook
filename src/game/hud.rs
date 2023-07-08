use std::time::Duration;

use super::OFFSET_PLAYERHUDMESSAGE_PTR;

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
            _ => 0
        }
    }
}

pub unsafe fn show_message(message: &'static str, message_type: MessageType, duration: Duration) {
    let phm = crate::get_offset(OFFSET_PLAYERHUDMESSAGE_PTR) as u64;
    let msg_type: u32 = message_type.into();
    let create = crate::make_func!(crate::get_offset_ptr(0x8e5890), [u64, u32, *const u8, u32, u32, u32, u8, u8], u64);
    create(phm, msg_type, message.as_ptr(), 0, 0, 0, 0, 1);
    std::thread::spawn(move || {
        let clear = crate::make_func!(crate::get_offset_ptr(0x8e6ab0), [u64, u32, *const u8, u32], u32);
        std::thread::sleep(duration);
        clear(phm, msg_type, message.as_ptr(), 0);
    });
}