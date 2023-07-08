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

use crate::game::hero::HeroHealthRaw;

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
    HOOK_HeroHealth_Init,
    make_func!(get_offset_ptr(0x9aee50), [u64]),
    (this: u64) => {
        HOOK_HeroHealth_Init.call(this);
        println!("HeroHealth = {:#x}", this);
        // let hero_health = hero::HeroHealth(this);
        // println!("HeroHealth::max_health = {}", hero_health.get_max_health());
    }
);

unsafe fn update_loop() {
    loop {
        // KeyCode T 0x54
        if get_key!(0x54) {
            let hero = game::hero::get_hero_entity();
            println!("{hero:#x?}");
            let hero_name = game::entity::get_entity_name(hero).expect("Bad");
            println!("{hero_name}");

            let hero_health = match game::entity::get_component_by_name(hero, "HeroHealth") {
                Some(handle) => handle,
                None => { 
                    println!("Failed to get component.");
                    continue 
                }
            };

            println!("HeroHealth: {hero_health:#x}");

            let mut hero_health = &mut *(hero_health as *mut HeroHealthRaw);
            println!("HeroHealth: {:p}", hero_health);
            println!("HeroHealth::current_health = {} | {:p}", hero_health.current_health, &hero_health.current_health);
            println!("HeroHealth::max_health = {} | {:p}", hero_health.max_health, &hero_health.max_health);
            hero_health.max_health += 100f32;

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


unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HeroHealth::Init()");

    // HOOK_PlayerHudMessage_Init.enable()
    //     .expect("Failed to enable hook: PlayerHudMessage::Init()");

    // HOOK_GetEntity.enable()
    //     .expect("Failed to enable hook: GetEntity()");
}

fn init() {
    unsafe {
        AllocConsole();
        println!("Injected!");

        enable_hooks();
        thread::spawn(|| update_loop());
    }
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
