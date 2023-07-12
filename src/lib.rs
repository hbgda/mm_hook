pub mod macros;
pub mod game;

use std::{ffi::{c_char, CString, CStr}, thread, time::Duration};

use game::entity::Entity;
use windows::{
    core::*, 
    Win32::Foundation::*, 
    Win32::System::Console::*, 
    Win32::System::LibraryLoader::*,
    Win32::System::SystemServices::*, 
    Win32::UI::Input::KeyboardAndMouse::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::Threading::*
};
use once_cell::sync::Lazy;

use crate::game::{hero::HeroHealth, OFFSET_HERO_HANDLE_PTR, OFFSET_PLAYERHUDMESSAGE, hud::MessageType};

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

// make_hook!(
//     HOOK_GetEntity,
//     make_func!(get_offset_ptr(game::OFFSET_GET_ENTITY_FN), [*const u64], u64),
//     (handle_ptr: *const u64): u64 => {
//         let entity = HOOK_GetEntity.call(handle_ptr);
//         // if entity != 0 && Entity(entity).name() == Ok("Spider-Man") {
//         //     println!("{:#x}", handle_ptr as isize - get_module_base());
//         // }
//         // println!("Entity Handle: {:#x}", handle_ptr.read());
//         // println!("Entity: {entity:#x}");
//         entity
//     }
// );

make_hook!(
    HOOK_Something,
    make_func!(get_offset_ptr(0x8e6ab0), [u64, i32, u64, u32], u64),
    (p1: u64, p2: i32, p3: u64, p4: u32): u64 => {
        println!("{p1:#x} {p2:#x} {p3:#x} {p4:#x}");
        HOOK_Something.call(p1, p2, p3, p4)
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

make_hook!(
    HOOK_LoadConfigSystem,
    make_func!(get_offset_ptr(0x1b9e7c0), [u64, u64, u64, *const u8, u64], u64),
    (asset_manager: u64, config: u64, idk: u64, system: *const u8, idk2: u64): u64 => {
        println!("{config:#x}");
        HOOK_LoadConfigSystem.call(asset_manager, config, idk, system, idk2)
    }
);

make_hook!(
    HOOK_LoadConfigThing,
    make_func!(get_offset_ptr(0x1b305f0), [*const u64, u64, u64, *const u8, u64, *const u8, u64], *const u8),
    (asset_manager: *const u64, config: u64, idk: u64, idk2: *const u8, idk3: u64, system_type: *const u8, idk4: u64): *const u8 => {
        let ret = HOOK_LoadConfigThing.call(asset_manager, config, idk, idk2, idk3, system_type, idk4);
        println!("ConfigThing: {:?}", CStr::from_ptr(ret as *const i8).to_str().unwrap());
        ret
    }
);

make_hook!(
    HOOK_MissionAbandon_CreatePopup,
    make_func!(get_offset_ptr(0x9414d0), [u64]),
    (this: u64) => {
        println!("{this:#x}");
        HOOK_MissionAbandon_CreatePopup.call(this);
    }
);

make_hook!(
    HOOK_SetMember,
    make_func!(get_offset_ptr(0x3a4a4c0), [u64, *const u64, *const u8, u64]),
    (this: u64, idk: *const u64, member: *const u8, thing: u64) => {
        println!("{this:#x} {idk:p} {} {thing:#x}", CStr::from_ptr(member as *const i8).to_str().unwrap());
        HOOK_SetMember.call(this, idk, member, thing);
    }
);

unsafe fn update_loop() {
    loop {
        // KeyCode T 0x54
        if get_key!(0x54) {
            let hero = game::hero::get_hero_entity().expect("Failed to get hero entity.");
            println!("{hero:#x?}");
            let hero_name = hero.get_name().expect("Failed to get hero name");
            println!("{hero_name}");

            let hero_transform = hero.get_transform_mut();
            println!("Hero Position: {:?} | Hero Scale: {:?}", hero_transform.get_position(), hero_transform.get_scale());
            let mut position = hero_transform.get_position();
            position.y += 100f32;
            hero_transform.set_position(&position);

            // game::hud::show_message("Testing hud stuff\0", MessageType::CenterLower, Some(Duration::from_secs(3)));
            // game::hud::show_message("<bold><u>Header</u></bold> [BTN_X]\nTest\0", MessageType::LeftBox, Some(Duration::from_secs(3)));

            // let hero_health = match hero.get_component("HeroHealth") {
            //     Some(handle) => handle,
            //     None => { 
            //         println!("Failed to get component.");
            //         continue 
            //     }
            // };

            // let mut hero_health = &mut *(hero_health as *mut HeroHealth);
            // println!("HeroHealth: {:p}", hero_health);
            // println!("HeroHealth::current_health = {} | {:p}", hero_health.current_health, &hero_health.current_health);
            // println!("HeroHealth::max_health = {} | {:p}", hero_health.max_health, &hero_health.max_health);
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
    // HOOK_HeroHealth_Init.enable()
    //     .expect("Failed to enable hook: HeroHealth::Init()");

    // HOOK_PlayerHudMessage_Init.enable()
    //     .expect("Failed to enable hook: PlayerHudMessage::Init()");

    // HOOK_ShowMessage.enable()
    //     .expect("Failed to enable hook: ShowMessage()");

    // HOOK_Something.enable()
    //     .expect("Failed to enable hook: Something()");

    // HOOK_GetEntity.enable()
    //     .expect("Failed to enable hook: GetEntity()");

    // HOOK_LoadConfigSystem.enable()
    //     .expect("Failed to enable hook: LoadConfigSystem");

    // HOOK_LoadConfigThing.enable()
    //     .expect("Failed to enable hook: LoadConfigThing()");

    // HOOK_MissionAbandon_CreatePopup.enable()
    //     .expect("Fucking idk");

    // HOOK_SetMember.enable()
    //     .expect("SSSSSS");
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
