pub mod macros;

use retour::GenericDetour;
use std::{
    sync::OnceLock,
    thread::{self, JoinHandle},
};
use windows::{
    core::*, Win32::Foundation::*, Win32::System::Console::*, Win32::System::LibraryLoader::*,
    Win32::System::SystemServices::*, Win32::System::Threading::*,
    Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*,
};
use once_cell::sync::Lazy;

const COMBO_PTR_OFFSET: isize = 0x77836B0;
// const POSSIBLE_HERO_ADDR: isize =  0xEDB88320;


unsafe fn get_module_base() -> isize {
    GetModuleHandleA(s!("MilesMorales.exe")).unwrap().0
}

unsafe fn get_offset_ptr<T>(offset: isize) -> *const T {
    (get_module_base() + offset) as *const T
}

unsafe fn get_offset_ptr_mut<T>(offset: isize) -> *mut T {
    (get_module_base() + offset) as *mut T
}

// make_hook!(
//     HOOK_Combo, 
//     make_func!(get_offset_ptr(0xABEE40), [i64]),
//     combo_func_hook,
//     [i64]
// );

make_hook!(
    HOOK_SomethingComboRelated, 
    make_func!(get_offset_ptr(0xABFD30), [i64, i64]),
    HOOK_SomethingComboRelated_Fn,
    [i64, i64]
);

unsafe extern "system" fn HOOK_SomethingComboRelated_Fn(param1: i64, param2: i64) {
    println!("Hook Called: SomethingComboRelated: param1 = {param1} {param1:#x} | param2 = {param2} {param2:x}");
    let combo_addr = param1 + 0x50;
    let ptr = combo_addr as *mut u32;
    let curr = ptr.read();
    ptr.write(34);
    println!("Changed {combo_addr:#x} to {curr} + 10");
    // let combo_ptr = get_offset_ptr::<u32>(COMBO_PTR_OFFSET);
    // let combo = combo_ptr.read();
    HOOK_SomethingComboRelated.call(param1, param2);
}

// extern "system" fn func_hook(param1: *const u8, param2: u64) {
//     // println!("Hook: param1: {param1} {param1:#x}");
//     unsafe { 
//     HOOK_Test.call(param1, param2); 
//     let p = param1.clone();
//     match format!("{:?}", p.clone()).as_str() {
//         "0x7ff62bce6330" |
//         "0x7ff657d6c5d0" | 
//         "0x7ff6563dce50" => {
//             return;
//         }
//         _ => {}
//     };

//     let mut s = String::new();
//     let mut p1c = param1 as isize;
//     loop {
//         let c = std::ptr::read(p1c as *const u8).to_string();
//         // println!("Char: {c}");
//         if c == "\0" {
//             break;
//         }

//         s.push_str(&c);
//         p1c += 1;
//     }
//     // println!("----------------------------\nHook: \nparam1: {param1:?} \nparam2: {param2} {param2:#x}");
//     println!("Component?: {s}\n-------------------\n");
//     // message_box!("HOOKED", "Please", 0);
//     }

// }

unsafe fn enable_hooks() {
    HOOK_SomethingComboRelated.enable()
        .expect("Failed to enable hook: SetCombo");
}

fn init() {
    //JoinHandle<()> {
    unsafe {
        AllocConsole();
        println!("Injected!");

        enable_hooks();
        // update_loop();
    }
}

// unsafe fn update_loop() {
//     loop {
//         // KeyCode T 0x54
//         if get_key!(0x54) {
//             let combo_ptr = get_offset_ptr::<u32>(COMBO_PTR_OFFSET);
//             let combo_curr = std::ptr::read(combo_ptr);
//             println!("Current Combo: {combo_curr}");
//         }

//         if get_key!(0x59) {
//             let combo_ptr = get_offset_ptr_mut::<u32>(COMBO_PTR_OFFSET);
//             let combo_curr = std::ptr::read(combo_ptr);
//             std::ptr::write(combo_ptr, combo_curr + 10);
//             println!("Increased combo?");
//         }

//         // KeyCode U 0x55
//         if get_key!(0x55) {
//             message_box!("Exiting update loop.", "This should unload the DLL!", 0);
//             break;
//         }
//     }

//     DllMain(
//         HMODULE(0),
//         DLL_PROCESS_DETACH,
//         std::ptr::null::<*mut ()>() as *mut (),
//     );
// }


#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(module: HMODULE, call_reason: u32, _: *mut ()) {
    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        _ => return,
    };

    // unsafe {
    //     FreeLibraryAndExitThread(module, 0);
    // }
}
