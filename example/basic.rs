use mm_hook::*;

init_mod!("Example Mod", "1.0", "L", {
    unsafe {
        mm_hook::init()
        Logger.log("Injected");
        enable_hooks();
        std::thread::spawn(|| update_loop());
    }
});

make_logger!();

make_hook!(
    HOOK_HeroHealth_Init,
    utils::get_offset_ptr(0x9aee50),
    (this: u64) {
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