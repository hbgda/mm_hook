pub mod macros;
pub mod game;

use std::{ffi::{c_char, CString, CStr}, thread};

use game::entity::Entity;
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

use crate::game::{hero::HeroHealth, OFFSET_HERO_HANDLE_PTR};

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


// 0x7ff74ba1ae2c

make_hook!(
    HOOK_GetEntity,
    make_func!(get_offset_ptr(game::OFFSET_GET_ENTITY_FN), [*const u64], u64),
    (handle_ptr: *const u64): u64 => {
        let entity = HOOK_GetEntity.call(handle_ptr);
        // if entity != 0 && Entity(entity).name() == Ok("Spider-Man") {
        //     println!("{:#x}", handle_ptr as isize - get_module_base());
        // }
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
            let hero = game::hero::get_hero_entity().expect("Sad");
            println!("{hero:#x?}");
            let hero_name = hero.name().expect("Not good");
            println!("{hero_name}");

            let hero_health = match hero.get_component("HeroHealth") {
                Some(handle) => handle,
                None => { 
                    println!("Failed to get component.");
                    continue 
                }
            };

            let mut hero_health = &mut *(hero_health as *mut HeroHealth);
            println!("HeroHealth: {:p}", hero_health);
            println!("HeroHealth::current_health = {} | {:p}", hero_health.current_health, &hero_health.current_health);
            println!("HeroHealth::max_health = {} | {:p}", hero_health.max_health, &hero_health.max_health);
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
