use crate::{scan_func_static, patterns, logging::Logger};

scan_func_static!(patterns::HUD_CREATEMESSAGE, ADD_MESSAGE(*const PlayerHudMessage, u8, *const u8, f32, u32, u32, bool, bool) -> bool);
scan_func_static!(patterns::HUD_CLEARMESSAGE, CLEAR_MESSAGE(*const PlayerHudMessage, u8, *const u8, bool) -> bool);

pub struct PlayerHudMessage;

impl PlayerHudMessage {
    pub unsafe fn add_message(&self, msg_type: MessageType, text: &str, duration: Option<f32>) -> bool {
        let duration = match duration {
            Some(dur) => dur,
            None => 0.0
        };
        
        let msg_type_u8 = msg_type.into();
        Logger::sys_log(format!("PlayerHudMessage::AddMessage({msg_type_u8}, {text}, {duration})"));
        ADD_MESSAGE(self, msg_type_u8, text.as_ptr(), duration, 0, 0, false, true)
    }

    pub unsafe fn clear_message(&self, msg_type: MessageType) -> bool {
        let msg_type_u8 = msg_type.into();
        Logger::sys_log(format!("PlayerHudMessage::ClearMessage({msg_type_u8})"));
        CLEAR_MESSAGE(self, msg_type_u8, std::ptr::null(), false)
    }
}
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

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MessageType {
    LeftCenter = 3,          // 3
    BottomLeft = 11,         // 11
    CenterLower = 14,        // 14
    CenterBox = 17,          // 17
    CenterUpper = 18,        // 18
    LeftBox = 20,            // 20
}

impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        self as u8
    }
}