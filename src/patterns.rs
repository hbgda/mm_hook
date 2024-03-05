pub(crate) const ACTOR_GETACTOR:                &'static str = "44 8B 01 41 8B D0 C1 EA 14 81 E2 FF 07 00 00 74 ?? 41 81 E0 FF FF 0F 00 4B 8D 0C ?? 48 C1 E1 06 48 03 0D ?? ?? ?? ?? 44 3B 05 ?? ?? ?? ?? 73 ?? 0F B7 41 ?? 3B D0 74";
pub(crate) const ACTOR_SPAWNACTOR:              &'static str = "40 53 48 83 EC 50 48 8B DA 48 8B D1 48 8D 0D";
pub(crate) const ACTOR_ENABLE:                  &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 56 57 41 56 48 81 EC 80 00 00 00 48 8B F9";

pub(crate) const TRANSFORM_SETPOSITION:         &'static str = "0F 10 51 ?? 4C 8B C1";
pub(crate) const TRANSFORM_SETSCALE:            &'static str = "40 53 48 81 EC 80 00 00 00 F2 0F 10 02";

pub(crate) const GUI_HASH:                      &'static str = "0F BE 11 B8 05 15 00 00";

pub(crate) const ASSETS_CREATEASSETHASH:        &'static str = "48 89 5C 24 ?? 57 48 83 EC 20 48 8B DA 48 8B F9 48 85 D2 74 ?? 80 3A 00";
pub(crate) const ASSETS_HASHSTRING:             &'static str = "40 53 48 83 EC 20 48 8B C2 48 8B D9 48 85 D2 74 ?? 80 3A 00 74 ?? 48 BA 42 0F 87 D7 95 57 6C C9";
pub(crate) const ASSETS_GETASSETMANAGER:        &'static str = "48 0F BE C1 48 8D 0D ?? ?? ?? ?? 48 8B 04 ?? C3";
pub(crate) const ASSETS_GETMANAGERBYASSETTYPE:  &'static str = "33 C0 81 F9 86 FE 08 72";
pub(crate) const ASSETS_LOADASSET:              &'static str = "48 89 54 24 ?? 53 56 57 41 55 41 56 48 83 EC 50 48 8B DA";

pub(crate) const HUD_CREATEMESSAGE:             &'static str = "48 8B C4 48 89 58 ?? 48 89 70 ?? 48 89 78 ?? 55 41 54 41 55 41 56 41 57 48 8D A8 ?? ?? ?? ?? 48 81 EC 30 05 00 00";
pub(crate) const HUD_ADDNEWMESSAGE:             &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 4C 89 64 24 ?? 55 41 56 41 57 48 8D 6C 24 ?? 48 81 EC C0 00 00 00 49 8B F9";
pub(crate) const HUD_CLEARMESSAGE:              &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 41 57 48 81 EC 90 00 00 00";
pub(crate) const HUD_CREATEPLAYERHUD:           &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 4C 24 ?? 57 48 83 EC 30 48 63 DA";
pub(crate) const HUD_HIDEHUD:                   &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 30 0F 57 C0 41 0F B6 F0 0F 2F D8";
pub(crate) const HUD_GETHUD:                    &'static str = "83 F9 01 77 ?? 48 63 C1 48 8D 0D ?? ?? ?? ?? 48 8B 04 ?? C3 33 C0";

pub(crate) const SCALEFORMLOADER_OPENFILE:      &'static str = "48 89 5C 24 ?? 57 48 81 EC 40 02 00 00 48 8B FA";
pub(crate) const SCALEFORM_OPENFILE_DISC:       &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 41 56 48 83 EC 30 33 DB 41 8B E8";
// pub(crate) const SCALEFORM_INVOKE:              &'static str = "48 8B C4 48 89 58 ?? 48 89 68 ?? 48 89 70 ?? 57 41 54 41 55 41 56 41 57 48 81 EC 60 02 00 00";
pub(crate) const SCALEFORM_INVOKE:              &'static str = "40 53 48 83 EC 40 8B 42 ?? 49 8B D9";

// pub(crate) const SF_EXTERNALINTERFACEHANDLER:   &'static str = "48 8B C4 48 89 58 ?? 48 89 70 ?? 48 89 78 ?? 4C 89 60 ?? 55 41 56 41 57 48 8D 68 ?? 48 81 EC 00 01 00 00";
pub(crate) const EXTERNALINTERFACE_LOBBY:       &'static str = "48 89 5C 24 ?? 48 89 74 24 ?? 48 89 7C 24 ?? 4C 89 64 24 ?? 55 41 56 41 57 48 8D 6C 24 ?? 48 81 EC 10 01 00 00 45 8B F9";

pub(crate) const SF_VALUE_CREATEOBJECT:         &'static str = "E8 ** ** ** ** 66 0F 6F 05 ?? ?? ?? ?? F3 0F 7F 45 ?? 33 F6 48 89 75 00";
pub(crate) const SF_VALUE_CREATEARRAY:          &'static str = "E8 ** ** ** ** 48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 85 C0 0F 84 ?? ?? ?? ?? 4C 8D 78";

pub(crate) const KEYBIND_CREATEKEYBIND:         &'static str = "48 89 5C 24 ?? 48 89 6C 24 ?? 56 57 41 56 48 83 EC 70 48 8B 05 ?? ?? ?? ?? 8B F2";
pub(crate) const KEYBIND_REGISTERKEYBINDS:      &'static str = "48 89 5C 24 ?? 57 48 83 EC 40 E8";

pub(crate) const KEYBIND_GETKEYBINDCATEGORY:    &'static str = "E8 ** ** ** ** 48 8B C8 C6 44 24 ?? 01 41 B1 01";
pub(crate) const KEYBIND_GETKEYBINDID:          &'static str = "E8 ** ** ** ** 4C 8B 17 45 33 C9 C7 44 24 ?? 00 00 00 00 45 33 C0 C7 44 24 ?? FF FF FF FF 48 8B D0 C7 44 24 ?? FF FF FF FF 48 8B CF";

pub(crate) const NX_INIT:                       &'static str = "48 83 EC 28 48 83 3D ?? ?? ?? ?? 00 74 ?? 32 C0";
pub(crate) const NX_GETNXACTIONS:               &'static str = "E8 ** ** ** ** 33 D2 C7 44 24 ?? E8 03 00 00 4C 8D 0D";

pub(crate) const HEROSYSTEM_OFFSET:             &'static str = "48 89 05 ** ** ** ** 48 89 35";