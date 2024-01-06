use mm_hook::keybinds::keybind::KeybindState;
use mm_hook::{init_mod, make_logger, make_hook, utils};
use mm_hook::{keybinds::{KeybindManager, keybind::KeyCode}, create_keybinds};
use mm_hook::game::{hero::get_hero_mut, component::hero::HeroHealth};

//////////
//// SETUP
//////////

// Initialize logger debugger.
// Not yet fully implemented
// make_logger!();

// Define mod information and entry function
init_mod!("Test Mod", "0.0.1", "L", {
    unsafe {
        // Enable important stuff
        mm_hook::init();
        // Logger.log("Injected".into());

        enable_hooks();

        let binds = create_keybinds();
        // Spawn thread for update loop
        std::thread::spawn(|| update_loop(binds))
    }
});

// Update loop, necessary for keybinds (might change in the future) 
// and any other logic you might need
pub unsafe fn update_loop(mut binds: KeybindManager) {
    loop {
        binds.poll();
    }
}

//////////
//// KEYBINDS
//////////


pub unsafe fn change_health(change: f32) -> Option<()> {
    // Get mutable reference to hero Entity
    let hero = get_hero_mut()?;
    // Get mutable reference to HeroHealth component
    let hero_health = hero.get_component_mut::<HeroHealth>()?;
    hero_health.current_health += change;
    Some(())
}

pub unsafe fn bind_health_inc(_state: &KeybindState) {
    change_health(10.0);
}


pub unsafe fn bind_health_dec(_state: &KeybindState) {
    change_health(-10.0);
}

unsafe fn create_keybinds() -> KeybindManager {
    // Macro for creating keybinds.
    // Not necessary but a convenient
    // shorthand for manually creating a KeybindManager 
    // and calling ::set_category(...) and ::add_keybind(...) when needed.
    create_keybinds!(
        // Format:
        // "Category Name" => [
        //      (bind_name, bind_description, primary_keycode, Option<secondary_keycode>, locked, bind_event),
        //      ...
        // ]
        "TestMod" => [
            (
                "Increase Health", 
                "Increases current health.", 
                KeyCode::D7, None, 
                false,
                bind_health_inc
            ),
            (
                "Decrease Health",
                "Decreases current health.",
                KeyCode::D8, None,
                false,
                bind_health_dec
            ),
            (
                "Reset Health",
                "Reset health to max.",
                KeyCode::D9, Some(KeyCode::D0),
                true,
                |_| {
                    let hero = match get_hero_mut() {
                        Some(hero) => hero,
                        None => return
                    };

                    let hero_health = match hero.get_component_mut::<HeroHealth>() {
                        Some(hero_health) => hero_health,
                        None => return
                    };

                    hero_health.current_health = hero_health.max_health
                }
            )
        ]
    )
}

//////////
//// COMPONENTS
//////////

// While you can use components defined in mm_hook::game::component,
// it is unlikely this library will ever have a definition for every component in the game,
// or anywhere close for that matter.
// So, you are freely able to implement the Component trait on any struct of your making (or use Entity::get_component_by_name())
// Note that your struct must accurately represent its layout in the game, so add padding where needed and use correct types.
// Rust will not always retain the layout of a struct as you define it, so you must also add #[repr(C)] to your definitions.

use mm_hook::impls::component::*;

#[repr(C)]
#[derive(Component)]
pub struct MyHeroHealth {
    _0x0: [u8; 0x80],
    max_health: f32,    // Offset 0x80
    _0x84: [u8; 0x4],
    current_health: f32 // Offset 0x88
}

//////////
//// HOOKS
//////////

// Important note:
// Hooking a function changes it's signature, so it not usually possible for two mods to hook the same function with pattern scanning.
// This is not so much an issue at the moment because this library still needs to be rewritten to actually support multiple mods,
// but important to keep in mind.

// Hooks are defined with the `make_hook!()` macro.
// Format:
// make_hook!(
//      HOOK_FuncName,
//      func_ptr,
//      // Return type
//      (param: T, ...) -> T {}
//      // Or no return
//      (param: T, ...) {}
// )
// In most cases you will probably need the native function to run,
// which you can do with `HOOK_FuncName.call(...params)`.
// Placement does of course matter, so call the native function when you feel it is necessary.

// Pointers to native functions can be obtained two ways,
// patterns scanning or hard coding the offset.
//
// Hard coding the offset is much easier and useful for testing, but it should ideally not be used in release builds,
// while it is unlikely Insomniac will update the game at this point it is still possible, 
// and it would be painful to update an entire mods worth of offsets.
// You can use `mm_hook::utils::get_offset_ptr<T>(offset)` to obtain a pointer to the address in memory.
make_hook!(
    HOOK_HeroHealth_Init,
    utils::get_offset_ptr(0x9aefc0),
    (hero_health_ptr: *const ()){
        HOOK_HeroHealth_Init.call(hero_health_ptr);
        let hero_health = &mut *(hero_health_ptr as *mut HeroHealth);
        // Logger.log(format!("HeroHealth::max_health: {}", hero_health.max_health));
    }
);

// The more desirable method of obtaining a function pointer is through pattern scanning.
// Best case scenario the method you want to hook has a unique pattern.
make_hook!(
    HOOK_HeroHealth_Init__scan,
    utils::scan("48 8B C4 48 89 58 ?? 48 89 70 ?? 48 89 78 ?? 55 48 8D 68 ?? 48 81 EC A0 00 00 00 0F 29 70 ?? 48 8B F9 E8 ?? ?? ?? ?? B2 03").unwrap(),
    (hero_health_ptr: *const ()){
        HOOK_HeroHealth_Init__scan.call(hero_health_ptr);
        let hero_health = &mut *(hero_health_ptr as *mut HeroHealth);
        // Logger.log(format!("HeroHealth::max_health: {}", hero_health.max_health));
    }
);

// If the function you want to hook does not have a unique pattern, you can scan for calls to it.
// Example from `mm_hook::keybinds::hooks`
// const PATTERN: &'static str = "E8 ** ** ** ** 48 8B C8 C6 44 24 ?? 01 41 B1 01";
// const GET_KEYBIND_CATEGORY_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
//     // Create scanner for pattern
//     let mut scanner = utils::create_scanner(PATTERN).unwrap();
//     // Get result
//     let found = scanner.next().unwrap();
//     // Parse offset
//     let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
//     // Calculate actual memory address
//     (found as isize + 5 + offset as isize) as usize
// });
// make_hook!(
//     HOOK_GetKeybindCategory,
//     *GET_KEYBIND_CATEGORY_ADDR,
//     (cat: u32) -> *const u8 { ... }
// );

// For other usage of pattern scanning see: https://github.com/hbgda/canny

// Hooks must be enabled by HOOK_YourHook.enable()
// If you need multiple hooks I would recommend you create a function where they get enabled.
unsafe fn enable_hooks() {
    HOOK_HeroHealth_Init.enable()
        .expect("Failed to enable hook: HOOK_HeroHealth_Init");
}