use mm_hook::*;

init_mod!("Example Mod", "1.0", "L", {
    unsafe {
        mm_hook::init()
        Logger.log("Injected".into());
        enable_hooks();
        let keybinds = create_keybinds();
        std::thread::spawn(|| update_loop(keybinds));
    }
});

make_logger!();

unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HeroHealth::Init()");
}

unsafe fn update_loop(keybinds: KeybindManager) {
    loop {
        keybinds.poll();
        // 0x54 T
        if get_key!(0x54) {
            let hero = game::hero::get_hero_entity().expect("Failed to get hero entity.");
            let hero_name = hero.get_name().expect("Failed to get hero name.");
            Logger.log(format!("Hero Name: {hero_name}"));
        }
    }
}

unsafe fn create_keybinds() -> Option<KeybindManager> {
    let mut manager = KeybindManager::new();

    manager.add_keybind(
        "BIND 1".into(), 
        "A custom keybind!".into(), 
        KeyCode::Comma, 
        None, 
        false, 
        |_| {
            let hero = match game::hero::get_hero_mut() {
                Some(hero) => hero,
                None => return
            };
            Logger.log(format!("Hero: {}", hero.get_name().unwrap()));
            if let Some(component) = hero.get_component_mut::<game::hero::HeroHealth>("HeroHealth") {
                Logger.log("Found".into());
                component.current_health += 100f32;
            }
            // Logger.log("Keybind pressed!".into());
        }
    );
    Some(manager)
}

make_hook!(
    HOOK_HeroHealth_Init,
    utils::get_offset_ptr(0x9aee50),
    (this: u64) {
        HOOK_HeroHealth_Init.call(this);
        Logger.log(format!("HeroHealth = {:#x}", this));
        let hero_health = &*(this as *const game::hero::HeroHealth);
        Logger.log(format!("HeroHealth::current_health = {}", hero_health.current_health));
        Logger.log(format!("HeroHealth::max_health = {}", hero_health.max_health));
    }
);