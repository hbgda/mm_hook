pub(crate) const ENTITY_GETENTITY:             &'static str = "44 8B 01 41 8B D0 C1 EA 14 81 E2 FF 07 00 00 74 ?? 41 81 E0 FF FF 0F 00 4B 8D 0C ?? 48 C1 E1 06 48 03 0D ?? ?? ?? ?? 44 3B 05 ?? ?? ?? ?? 73 ?? 0F B7 41 ?? 3B D0 74";

pub(crate) const TRANSFORM_SETPOSITION: &'static str = "0F 10 51 ?? 4C 8B C1";
pub(crate) const TRANSFORM_SETSCALE:    &'static str = "40 53 48 81 EC 80 00 00 00 F2 0F 10 02";

pub(crate) const HUD_CREATEMESSAGE:     &'static str = "48 8B C4 48 89 58 ?? 48 89 70 ?? 48 89 78 ?? 55 41 54 41 55 41 56 41 57 48 8D A8 ?? ?? ?? ?? 48 81 EC 30 05 00 00";
pub(crate) const HUD_CLEARMESSAGE:      &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 41 57 48 81 EC 90 00 00 00";
pub(crate) const HUD_CREATEPLAYERHUD:   &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 4C 24 ?? 57 48 83 EC 30 48 63 DA";
