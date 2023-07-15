use mm_hook::*;

make_hook!(
    HOOK_HeroHealth_Init,
    make_func!(utils::get_offset_ptr(0x9aee50), [u64]),
    (this: u64) => {
        HOOK_HeroHealth_Init.call(this);
        Logger.log(&format!("HeroHealth = {:#x}", this));
        let hero_health = &*(this as *const game::hero::HeroHealth);
        Logger.log(&format!("HeroHealth::current_health = {}", hero_health.current_health));
        Logger.log(&format!("HeroHealth::max_health = {}", hero_health.max_health));
    }
);

unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HeroHealth::Init()");
}

unsafe fn update_loop() {
    loop {
        // 0x54 T
        if get_key!(0x54) {
            let hero = game::hero::get_hero_entity().expect("Failed to get hero entity.");
            let hero_name = hero.get_name().expect("Failed to get hero name.");
            Logger.log(&format!("Hero Name: {hero_name}"));
        }
    }
}

unsafe fn init() {
    Logger.log("Injected");

    enable_hooks();
    std::thread::spawn(|| update_loop());
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(_module: mm_hook::HMODULE, call_reason: u32, _: *mut ()) {
    match call_reason {
        DLL_PROCESS_ATTACH => unsafe { init() },
        _ => return,
    };
}

make_logger!(GetModInfo());

#[no_mangle]
extern "system" fn GetModInfo() -> CModInfo {
    ModInfo { 
        title: "Test Script", 
        version: "0.0.1", 
        author: "L" 
    }.into()
}