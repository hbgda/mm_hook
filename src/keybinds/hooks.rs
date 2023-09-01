use crate::{make_hook, utils, game::nx, make_func};

use super::{KEYBIND_QUEUE, CREATE_KEYBIND, CREATED_KEYBINDS, keybind::Keybind, KEYBIND_INDEX};

pub unsafe fn enable() {
    HOOK_GetKeybindCategory.enable()
        .expect("Failed to enable hook: GetKeybindCategory()");
    HOOK_CreateKeybinds.enable()
        .expect("Failed to enable hook: CreateKeybinds()");
    HOOK_GetKeybindId.enable()
        .expect("Failed to enable hook: GetKeybindId()");
}

static mut MOD_KEYBIND_IDS: Vec<String> = Vec::new();

make_hook!(
    // Dunno about this name but it's the best I've got.
    HOOK_GetKeybindId,
    utils::get_offset_ptr(0x1bcd450),
    (bind_id: u32) -> *const () {
        if bind_id >= 0x50 {
            return (MOD_KEYBIND_IDS.get((bind_id - 0x50) as usize).unwrap().as_ptr()) as *const ();
        }
        HOOK_GetKeybindId.call(bind_id)
    }
);

const KEYBIND_CATEGORY_MODS: &'static str = "MODS\0";
make_hook!(
    HOOK_GetKeybindCategory,
    // Not sure how I can avoid the static offset here
    crate::utils::get_offset_ptr(0x0d8ace0),
    (cat: u32) -> *const u8 {
        if cat == 4 {
            return KEYBIND_CATEGORY_MODS.as_ptr();
        }
        HOOK_GetKeybindCategory.call(cat)
    }
);

make_hook!(
    HOOK_CreateKeybinds,
    crate::utils::scan(crate::patterns::KEYBIND_REGISTERKEYBINDS).unwrap(),
    () {
        HOOK_CreateKeybinds.call();
        let nx_actions = nx::get_nx_actions() as *const u64;
        // let nx_actions = ((nx as u64) + 0xa8) as *const ();
        for partial in KEYBIND_QUEUE.iter() {
            MOD_KEYBIND_IDS.push(format!("Mkb_Custom_{}\0", partial.idx));
            let bind = CREATE_KEYBIND(nx_actions as *const (), 4, partial.idx, partial.name, partial.primary, partial.secondary) as *mut Keybind;
            (*bind).description = partial.desc;
            (*bind).locked = partial.locked;
            // (*bind).default_secondary = partial.secondary;
            CREATED_KEYBINDS.insert(partial.idx, bind);
        }
        let f = make_func!(*((*nx_actions + 0x78) as *const *const ()), (*const (), u64));
        f(nx_actions as *const (), 0xb);
    }
);