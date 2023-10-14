use once_cell::sync::Lazy;

use crate::{make_hook, utils, game::nx, patterns, logging::Logger};

use super::{KEYBIND_QUEUE, CREATE_KEYBIND, CREATED_KEYBINDS, keybind::Keybind, KEYBIND_MOD_CATEGORIES};

pub unsafe fn enable() {
    HOOK_GetKeybindCategory.enable()
        .expect("Failed to enable hook: GetKeybindCategory()");
    HOOK_CreateKeybinds.enable()
        .expect("Failed to enable hook: CreateKeybinds()");
    HOOK_GetKeybindId.enable()
        .expect("Failed to enable hook: GetKeybindId()");
}

const GET_KEYBIND_ID_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
    let mut scanner = utils::create_scanner(patterns::KEYBIND_GETKEYBINDID).unwrap();
    let found = scanner.next().unwrap();
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    (found as isize + 5 + offset as isize) as usize
});
static mut MOD_KEYBIND_IDS: Vec<String> = Vec::new();
make_hook!(
    // Dunno about this name but it's the best I've got.
    HOOK_GetKeybindId,
    // utils::get_offset_ptr(0x1bcd450),
    *GET_KEYBIND_ID_ADDR,
    (bind_id: u32) -> *const () {
        if bind_id >= 0x50 {
            return (MOD_KEYBIND_IDS.get((bind_id - 0x50) as usize).unwrap().as_ptr()) as *const ();
        }
        HOOK_GetKeybindId.call(bind_id)
    }
);

const GET_KEYBIND_CATEGORY_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
    let mut scanner = utils::create_scanner(patterns::KEYBIND_GETKEYBINDCATEGORY).unwrap();
    let found = scanner.next().unwrap();
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    // Logger::sys_log(format!("{found:#X} + 5 + {offset} = {:#X} {:#?}", found as isize + 5 + offset as isize, scanner.store));
    (found as isize + 5 + offset as isize) as usize
});
make_hook!(
    HOOK_GetKeybindCategory,
    // Not sure how I can avoid the static offset here
    // utils::get_offset_ptr(0x0d8ace0),
    *GET_KEYBIND_CATEGORY_ADDR,
    (cat: u32) -> *const u8 {
        if cat >= 4 {
            return KEYBIND_MOD_CATEGORIES[(cat - 4) as usize].as_ptr();
        }
        HOOK_GetKeybindCategory.call(cat)
    }
);

make_hook!(
    HOOK_CreateKeybinds,
    utils::scan(patterns::KEYBIND_REGISTERKEYBINDS).unwrap(),
    () {
        HOOK_CreateKeybinds.call();
        let nx_actions = nx::get_nx_actions(); 
        KEYBIND_QUEUE.sort_unstable_by(|a, b| a.group.cmp(&b.group));
        for partial in KEYBIND_QUEUE.iter() {
            MOD_KEYBIND_IDS.push(format!("Mkb_Custom_{}\0", partial.idx));
            let bind = CREATE_KEYBIND(nx_actions, partial.group, partial.idx, partial.name, partial.primary, partial.secondary) as *mut Keybind;
            (*bind).description = partial.desc;
            (*bind).locked = partial.locked;
            CREATED_KEYBINDS.insert(partial.idx, bind);
        }
        // let f = make_func!(*((*(nx_actions as *const u64) + 0x78) as *const *const ()), (*const (), u64));
        // f(nx_actions, 0xb);
    }
);