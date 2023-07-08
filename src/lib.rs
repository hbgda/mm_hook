pub mod macros;
pub mod game;

use std::{ffi::{c_char, CString, CStr}, thread};

use windows::{
    core::*, 
    Win32::Foundation::*, 
    Win32::System::Console::*, 
    Win32::System::LibraryLoader::*,
    Win32::System::SystemServices::*, 
    Win32::UI::Input::KeyboardAndMouse::*, 
    Win32::UI::WindowsAndMessaging::*,
};
use once_cell::sync::Lazy;

unsafe fn get_module_base() -> isize {
    GetModuleHandleA(s!("MilesMorales.exe")).unwrap().0
}

unsafe fn get_offset(offset: isize) -> isize {
    get_module_base() + offset
}

unsafe fn get_offset_ptr<T>(offset: isize) -> *const T {
    (get_module_base() + offset) as *const T
}

unsafe fn get_offset_ptr_mut<T>(offset: isize) -> *mut T {
    (get_module_base() + offset) as *mut T
}

pub trait GameType {
    fn from(handle: u64) -> Self;
}

make_hook!(
    HOOK_HeroHealth_Init,
    make_func!(get_offset_ptr(0x9aee50), [u64]),
    (this: u64) => {
        HOOK_HeroHealth_Init.call(this);
        println!("HeroHealth = {:#x}", this);
        // let hero_health = hero::HeroHealth(this);
        // println!("HeroHealth::max_health = {}", hero_health.get_max_health());
    }
);

make_hook!(
    HOOK_PlayerHudMessage_Init,
    make_func!(get_offset_ptr(0x8e8220), [u64]),
    (this: u64) => {
        HOOK_PlayerHudMessage_Init.call(this);
        println!("PlayerHudMessage: {:#x}", this);
        std::ptr::write((this + 0x18) as *mut CString, CString::new("Testing").unwrap())
    }
);

make_hook!(
    HOOK_GetEntity,
    make_func!(get_offset_ptr(game::OFFSET_GET_ENTITY_FN), [*const u64], u64),
    (handle_ptr: *const u64): u64 => {
        let entity = HOOK_GetEntity.call(handle_ptr);
        // println!("Entity Handle: {:#x}", handle_ptr.read());
        // println!("Entity: {entity:#x}");
        entity
    }
);
make_hook!(
    HOOK_Test,
    make_func!(0x123, [], u64),
    (): u64 => {
        0
    }
);

// unsafe extern "system" fn HOOK_HeroHealth_Init_Fn(this: u64) {
//     HOOK_HeroHealth_Init.call(this);
//     let hero_health = hero::HeroHealth(this);
//     println!("HeroHealth = {:#x}", this);
//     println!("HeroHealth::max_health = {}", hero_health.get_max_health());
// }

unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HeroHealth::Init()");

    HOOK_PlayerHudMessage_Init.enable()
        .expect("Failed to enable hook: PlayerHudMessage::Init()");

    HOOK_GetEntity.enable()
        .expect("Failed to enable hook: GetEntity()");
}

fn init() {
    unsafe {
        AllocConsole();
        println!("Injected!");

        enable_hooks();
        thread::spawn(|| update_loop());
    }
}

unsafe fn update_loop() {
    loop {
        // KeyCode T 0x54
        if get_key!(0x54) {
            // let create_msg = make_func!(get_offset_ptr(0x8e77c0), []);
            let hero = game::get_hero_entity();
            println!("{hero:#x?}");
            let hero_name = CStr::from_ptr(((hero + 0xB0) as *const u64).read() as *const i8).to_str().expect("Fuck");
            println!("{hero_name}");

            // let hero = game::get_hero();
            // println!("Hero: {hero:#x}");
            // let hero_name = game::get_entity_name(hero).expect("Fuck");
            // println!("Hero: {hero:#x} | {hero_name}");
        }

        // KeyCode U 0x55
        if get_key!(0x55) {
            message_box!("Exiting update loop.", "This should unload the DLL!", 0);
            break;
        }
    }
    panic!();
}


#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(_module: HMODULE, call_reason: u32, _: *mut ()) {
    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        _ => return,
    };

    // unsafe {
    //     FreeLibraryAndExitThread(module, 0);
    // }
}
