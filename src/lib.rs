use std::thread::{self, JoinHandle};
use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::KeyboardAndMouse::*,
    Win32::System::Threading::*,
    Win32::System::SystemServices::*,
    Win32::System::LibraryLoader::*, 
    Win32::Foundation::*,
};

macro_rules! message_box {
    ($title:expr, $content:expr, $style:expr) => {
        unsafe {
            MessageBoxA(HWND(0), s!($content), s!($title), MESSAGEBOX_STYLE($style))
        }
    };
}

macro_rules! get_key {
    ($key:expr) => {
        unsafe {
            GetAsyncKeyState($key) & 1 == 1
        }
    };
}

unsafe fn update_loop() {
    loop {
        // KeyCode T 0x54
        if get_key!(0x54) {
            message_box!("Testing T", "", 0);
        }

        // KeyCode U 0x55
        if get_key!(0x55) {
            message_box!("Exiting update loop.", "This should unload the DLL!", 0);
            break;
        }
    }
}

fn init() -> Option<JoinHandle<()>> {
    message_box!("Test", "Spidey Script", 0);

    unsafe {
        update_loop();
    }

    None
}



// fn detach(thread: )

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(
    module: HMODULE,
    call_reason: u32,
    _: *mut ()
) {

    let mut thread: Option<JoinHandle<()>> = None;

    match call_reason {
        DLL_PROCESS_ATTACH => thread = init(),
        // DLL_PROCESS_DETACH => {
        //     if let Some(thread) = thread {
        //         if let Err(e) = thread.join() {
        //             message_box!("Smth fucking broke idk g", "", 0);
        //         }
        //     }
        // },
        _ => {}
    }

    thread.unwrap().join().unwrap();
    unsafe { 
        FreeLibraryAndExitThread(module, 0);
    }
}