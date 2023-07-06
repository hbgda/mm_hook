pub mod macros;
pub mod hero;

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

unsafe fn get_offset_ptr<T>(offset: isize) -> *const T {
    (get_module_base() + offset) as *const T
}

unsafe fn get_offset_ptr_mut<T>(offset: isize) -> *mut T {
    (get_module_base() + offset) as *mut T
}

make_hook!(
    HOOK_HeroHealth_Init,
    make_func!(get_offset_ptr(0x9aee50), [u64]),
    HOOK_HeroHealth_Init_Fn: [u64]
);

unsafe extern "system" fn HOOK_HeroHealth_Init_Fn(this: u64) {
    HOOK_HeroHealth_Init.call(this);
    let hero_health = hero::HeroHealth(this);
    println!("HeroHealth = {:#x}", this);
    println!("HeroHealth::max_health = {}", hero_health.get_max_health());
}

unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HeroHealth::Init()");
}

fn init() {
    unsafe {
        AllocConsole();
        println!("Injected!");

        enable_hooks();
    }
}

unsafe fn update_loop() {
    loop {
        // KeyCode T 0x54
        if get_key!(0x54) {
            // let hero = 
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
extern "system" fn DllMain(module: HMODULE, call_reason: u32, _: *mut ()) {
    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        _ => return,
    };

    // unsafe {
    //     FreeLibraryAndExitThread(module, 0);
    // }
}
