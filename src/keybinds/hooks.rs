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

// const GET_KEYBIND_ID_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
//     let mut scanner = utils::create_scanner(patterns::KEYBIND_GETKEYBINDID).unwrap();
//     let found = scanner.next().unwrap();
//     let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
//     (found as isize + 5 + offset as isize) as usize
// });
static mut MOD_KEYBIND_IDS: Vec<String> = Vec::new();
make_hook!(
    HOOK_GetKeybindId,
    utils::scan_func_call(patterns::KEYBIND_GETKEYBINDID).unwrap(),
    (bind_id: u32) -> *const () {
        if bind_id >= 0x50 {
            return (MOD_KEYBIND_IDS.get((bind_id - 0x50) as usize).unwrap().as_ptr()) as *const ();
        }
        HOOK_GetKeybindId.call(bind_id)
    }
);

// const GET_KEYBIND_CATEGORY_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
//     let mut scanner = utils::create_scanner(patterns::KEYBIND_GETKEYBINDCATEGORY).unwrap();
//     let found = scanner.next().unwrap();
//     let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
//     (found as isize + 5 + offset as isize) as usize
// });
make_hook!(
    HOOK_GetKeybindCategory,
    utils::scan_func_call(patterns::KEYBIND_GETKEYBINDCATEGORY).unwrap(),
    (cat: u32) -> *const u8 {
        if cat >= 4 {
            let cat = &KEYBIND_MOD_CATEGORIES[(cat - 4) as usize];
            Logger::sys_log(cat.to_string());
            return cat.as_ptr();
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
            Logger::sys_log(format!("{partial:?}"));
            MOD_KEYBIND_IDS.push(format!("Mkb_Custom_{}\0", partial.idx));
            let bind = CREATE_KEYBIND(nx_actions, partial.group, partial.idx, partial.name, partial.primary, partial.secondary) as *mut Keybind;
            (*bind).description = partial.desc;
            (*bind).locked = partial.locked;
            CREATED_KEYBINDS.insert(partial.idx, bind);
        }
    }
);