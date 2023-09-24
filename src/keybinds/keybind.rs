#[repr(C)]
#[derive(Eq, PartialEq, Hash)]
pub struct Keybind {
    pub state: *const KeybindState,
    pub default_primary: u32,
    pub default_secondary: u32,
    pub locked: bool,
    pub group: u32,
    pub name: *const u8,
    pub description: *const u8,
    _0x28: u32,
    _0x2c: u32
}

#[repr(C)]
pub struct KeybindState {
    _0x0: [u8; 0xA3],
    pub primary_key: u32,
    pub secondary_key: u32,
    pub primary_pressed: bool,
    unknown: u32, // Possibly related to held time?
    _0xb4: [u8; 0x124],
    pub pressed: f32
}

pub enum KeyCode {
    // Keyboard
    // 0x1 - 0x53
    Escape = 1, 
    D1, D2, D3, D4, D5, D6, D7, D8, D9, D0, Minus, Equals, Backspace,
    Tab, Q, W, E, R, T, Y, U, I, O, P, LeftBracket, RightBracket, Enter,
    LeftControl, A, S, D, F, G, H, J, K, L, Semicolon, Apostrophe, Tilde,
    LeftShift, BackSlash, Z, X, C, V, B, N, M, Comma, Period, ForwardSlash, RightShift, 
    NumpadAsterisk,
    LeftAlt, Spacebar,
    CapsLock,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10,
    NumLock, ScrollLock,
    Numpad7, Numpad8, Numpad9, NumpadMinus, 
    Numpad4, Numpad5, Numpad6, NumpadPlus,
    Numpad1, Numpad2, Numpad3, 
    Numpad0, NumpadPeriod,
    // 0x57 - 0x58
    F11 = 0x57, F12,
    // 0x9C - 0x9D
    NumpadEnter = 0x9C, RightControl,
    // 0xB5 - 0xD3
    NumpadForwardSlash = 0xB5,
    RightAlt = 0xB8,
    Pause = 0xC5,
    Home = 0xC7,
    Up,
    PageUp,
    LeftArrow = 0xCB,
    RightArrow = 0xCD,
    End = 0xCF,
    DownArrow,
    PageDown,
    Insert,
    Delete,
    
    // Mouse 
    // 0x12D - 0x136
    LeftMouse = 0x12D,
    RightMouse,
    Mouse4,
    Mouse5,
    
    MouseWheelUp,
    MouseWheelDown,
    MouseWheelLeft,
    MouseWheelRight,
}