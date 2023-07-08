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

use crate::game::{hero::HeroHealth, OFFSET_HERO_HANDLE_PTR, OFFSET_PLAYERHUDMESSAGE_PTR};

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
    HOOK_Something,
    make_func!(get_offset_ptr(0x8eef60), [u64]),
    (thing: u64) => {
        println!("Thing: {thing:#x}");
        HOOK_Something.call(thing);
    }
);

make_hook!(
    HOOK_ShowMessage,
    make_func!(get_offset_ptr(0x8e5890), [u64, u32, *const u8, u32, u32, u32, u8, u8], u64),
    (param1: u64, param2: u32, param3: *const u8, param4: u32, param5: u32, param6: u32, param7: u8, param8: u8): u64 => {
        println!("{param1:#x} {param2} {param3:p} {param4:#x} {param5} {param6} {param7} {param8}");
        // println!("Message: {:?}\n---------\n", CStr::from_ptr(param3 as *const i8));
        HOOK_ShowMessage.call(param1, param2, param3, param4, param5, param6, param7, param8)
    }
);

make_hook!(
    HOOK_PlayerHudMessage_Init,
    make_func!(get_offset_ptr(0x8e8220), [u64]),
    (this: u64) => {
        HOOK_PlayerHudMessage_Init.call(this);
        println!("PlayerHudMessage: {:#x}", this);
        // std::ptr::write((this + 0x18) as *mut CString, CString::new("Testing").unwrap())
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

            let phm = get_offset(OFFSET_PLAYERHUDMESSAGE_PTR);
            // ((phm + 0x2c) as *mut u8).write(0);
            // let thing = make_func!(get_offset_ptr(0x8eff10), [*const u64]);
            // thing(std::ptr::addr_of!(phm) as *const u64);
            // let show_message = make_func!(get_offset_ptr(0x8ebab0), [u64, i32, u64, u64, u64, u8, u64, f32, f32, *const i32], u8);
            // dbg!(show_message(phm as u64, 14, "Testing".as_ptr() as u64, 0, 0, 1, 0, 0f32, 0xbf800000u32 as f32, 0 as *const i32));
            // let show_message = make_func!(get_offset_ptr(0x8e5890), [u64, u32, *const u8, u32, u32, u32, u8, u8], u64);
            // show_message(phm as u64, 14, "Testing0\0".as_ptr(), 0x3ff, 0, 0, 1, 1);
        }

        // KeyCode U 0x55
        // if get_key!(0x55) {
        //     message_box!("Exiting update loop.", "This should unload the DLL!", 0);
        //     break;
        // }
    }
    panic!();
}


unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HeroHealth::Init()");

    HOOK_PlayerHudMessage_Init.enable()
        .expect("Failed to enable hook: PlayerHudMessage::Init()");

    HOOK_ShowMessage.enable()
        .expect("Failed to enable hook: ShowMessage()");

    HOOK_Something.enable()
        .expect("Failed to enable hook: Something()");

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
