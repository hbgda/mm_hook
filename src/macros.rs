#[macro_export]
macro_rules! message_box {
    ($title:expr, $content:expr, $style:expr) => {
        unsafe {
            $crate::MessageBoxA($crate::HWND(0), $crate::s!($content), $crate::s!($title), $crate::MESSAGEBOX_STYLE($style))
        }
    };
}

#[macro_export]
macro_rules! get_key {
    ($key:expr) => {
        unsafe {
            $crate::GetAsyncKeyState($key) & 1 == 1
        }
    };
}

#[macro_export]
macro_rules! make_func {
    ($addr:expr, ($($params:ty),*)) => {
        $crate::make_func!($addr, ($($params),*): ())
    };
    ($addr:expr, ($($params:ty),*) -> $ret:ty) => {
        std::mem::transmute::<*const (), unsafe extern "system" fn($($params,)*) -> $ret>($addr as _)
    };
}

#[macro_export]
macro_rules! scan_func {
    ($pattern:expr, ($($params:ty),*)) => {
        $crate::scan_func!($pattern, ($($params),*): ());
    };
    ($pattern:expr, ($($params:ty),*) -> $ret:ty) => {
        {
            let addr = $crate::utils::scan($pattern).unwrap();
            $crate::make_func!(addr, ($($params),*) -> $ret)
        }
    }
}

#[macro_export]
macro_rules! make_func_static {
    ($offset:expr, $name:ident ($($params:ty),*)) => {
        $crate::make_func_static!($offset, $name ($($params),*): ());
    };
    ($offset:expr, $name:ident ($($params:ty),*) -> $ret:ty) => {
        static $name: $crate::Lazy<unsafe extern "system" fn($($params,)*) -> $ret> = $crate::Lazy::new(|| unsafe { $crate::make_func!($crate::utils::get_offset_ptr($offset), ($($params),*) -> $ret) });
    };
}

#[macro_export]
macro_rules! scan_func_static {
    ($pattern:expr, $name:ident ($($params:ty),*)) => {
        $crate::scan_func_static!($pattern, $name($($params),*) -> ());
    };
    ($pattern:expr, $name:ident ($($params:ty),*) -> $ret:ty) => {
        static $name: $crate::Lazy<unsafe extern "system" fn($($params,)*) -> $ret> = $crate::Lazy::new(|| unsafe { $crate::scan_func!($pattern, ($($params),*) -> $ret) });
    };
}

#[macro_export]
macro_rules! make_hook {
    ($id:ident, $addr:expr, ($($param:ident: $type:ty),*) $code:block) => {
        $crate::make_hook!($id, $addr, ($($param: $type),*) -> () $code);
    };
    ($id:ident, $addr:expr, ($($param:ident: $ty:ty),*) -> $ret:ty $code:block) => {
        $crate::paste! {
            #[allow(non_upper_case_globals)]
            pub(crate) static $id: $crate::Lazy<$crate::GenericDetour<unsafe extern "system" fn($($ty,)*) -> $ret>> = $crate::Lazy::new(|| {
                unsafe {
                    let func = $crate::make_func!($addr, ($($ty),*) -> $ret);
                    $crate::GenericDetour::new(func, [<$id _Fn>])
                        .expect(&format!("Failed to create hook: {}", stringify!($id)))
                }
            });
            #[allow(non_snake_case)]
            unsafe extern "system" fn [<$id _Fn>]($($param: $ty,)*) -> $ret {
                $code
            }
        }
    };
    ($id:ident, $addr:expr, ($($param:ident: $type:ty),*) $code:block, $enabled:literal) => {
        $crate::make_hook!($id, $addr, ($($param: $type),*) -> () $code, $enabled);
    };
    ($id:ident, $addr:expr, ($($param:ident: $ty:ty),*) -> $ret:ty $code:block, $enabled:literal) => {
        $crate::paste! {
            #[allow(non_upper_case_globals)]
            pub(crate) static $id: $crate::Lazy<$crate::GenericDetour<unsafe extern "system" fn($($ty,)*) -> $ret>> = $crate::Lazy::new(|| {
                unsafe {
                    let func = $crate::make_func!($addr, ($($ty),*) -> $ret);
                    let hook = $crate::GenericDetour::new(func, [<$id _Fn>])
                        .expect(&format!("Failed to create hook: {}", stringify!($id)));
                    if $enabled {
                        hook.enable()
                            .expect(&format!("Failed to enable hook: {}", stringify!($id)));
                    }
                    hook
                }
            });
            #[allow(non_snake_case)]
            unsafe extern "system" fn [<$id _Fn>]($($param: $ty,)*) -> $ret {
                $code
            }
        }
    };
}

#[macro_export]
macro_rules! intercept_static {
    ($id:ident: $ty:ty, $hook_ident:ident, $addr:expr, [ $intercept:ident ] ($($param:ident: $pty:ty),*)) => {
        $crate::intercept_static!($id: $ty, $hook_ident:ident, $addr, [ $intercept ] ($($param: $pty),*) -> () );
    };
    ($id:ident: $ty:ty, $hook_ident:ident, $addr:expr, [ $intercept:ident ] ($($param:ident: $pty:ty),*) -> $ret:ty ) => {
        $crate::paste! {
            static mut $id: std::sync::RwLock<Option<$ty>> = std::sync::RwLock::new(None);
            $crate::make_hook!(
                $hook_ident,
                $addr,
                ($($param: $pty),*) -> $ret {
                    let mut lock = $id.write().unwrap();
                    *lock = Some($intercept);
                    $hook_ident.disable().unwrap();
                    $hook_ident.call($($param),*)
                }
            );
            pub unsafe fn [<get_ $id:lower>]() -> Option<$ty> {
                match $id.read() {
                    Ok(s) => *s,
                    Err(_) => None
                }
            }
        }
    };
}

// #[macro_export]
// macro_rules! make_hook {
//     ($id:ident, $ori:expr, ($($param:ident: $ty:ty),*) => $code:block) => {
//         $crate::make_hook!($id, $ori, ($($param: $ty),*): () => $code);
//     };
//     ($id:ident, $ori:expr, ($($param:ident: $ty:ty),*): $ret:ty => $code:block) => {
//         $crate::paste! {
//             #[allow(non_upper_case_globals)]
//             static $id: $crate::Lazy<$crate::GenericDetour<unsafe extern "system" fn($($ty,)*) -> $ret>> = $crate::Lazy::new(|| {
//                 unsafe {
//                     $crate::GenericDetour::new($ori, [<$id _Fn>])
//                         .expect(&format!("Failed to create hook: {}", stringify!($id)));
//                 }
//             });
//             #[allow(non_snake_case)]
//             unsafe extern "system" fn [<$id _Fn>]($($param: $ty,)*) -> $ret {
//                 $code
//             }
//         }
//     };
//     ($id:ident, $ori:expr, ($($param:ident: $ty:ty),*) => $code:block, $enabled:literal) => {
//         $crate::make_hook!($id, $ori, ($($param: $ty),*): () => $code, $enabled);
//     };
//     ($id:ident, $ori:expr, ($($param:ident: $ty:ty),*): $ret:ty => $code:block, $enabled:literal) => {
//         $crate::paste! {
//             #[allow(non_upper_case_globals)]
//             static $id: $crate::Lazy<$crate::GenericDetour<unsafe extern "system" fn($($ty,)*) -> $ret>> = $crate::Lazy::new(|| {
//                 unsafe {
//                     let hook = $crate::GenericDetour::new($ori, [<$id _Fn>])
//                         .expect(&format!("Failed to create hook: {}", stringify!($id)));
//                     if $enabled {
//                         hook.enable();
//                     }
//                     hook
//                 }
//             });
//             #[allow(non_snake_case)]
//             unsafe extern "system" fn [<$id _Fn>]($($param: $ty,)*) -> $ret {
//                 $code
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! load_library_func {
    ($module:literal, $module_fn:literal, $fn:ident ($($ty:ty),*)) => {
        $crate::load_library_func!($module, $module_fn, $fn ($($ty),*) -> ());
    };
    ($module:literal, $module_fn:literal, $fn:ident ($($ty:ty),*) -> $ret:ty) => {
        const $fn: $crate::Lazy<Option<extern "system" fn($($ty,)*)>> = $crate::Lazy::new(|| unsafe {
            let handle = match $crate::GetModuleHandleA($crate::s!($module)) {
                Ok(handle) => handle,
                Err(_) => return None
            };

            let func = match $crate::GetProcAddress(handle, $crate::s!($module_fn)) {
                Some(func) => func,
                None => return None
            };
            Some(std::mem::transmute::<_, extern "system" fn($($ty,)*) -> $ret>(func))
        });
    };
}

#[macro_export]
macro_rules! make_logger {
    () => {
        #[allow(non_upper_case_globals)]
        const Logger: $crate::logging::Logger = $crate::logging::Logger { mod_info: MOD_INFO };
        // const Logger: $crate::Lazy<$crate::logging::Logger> = $crate::Lazy::new(|| {
        //     let logger = $crate::logging::Logger { mod_info: MOD_INFO.clone() };
        //     // logger.log("Logging enabled.".into());
        //     logger
        // });
    };
}

#[macro_export]
macro_rules! init_mod {
    ($name:literal, $version:literal, $author:literal, $init:block) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        extern "system" fn DllMain(_module: $crate::HMODULE, call_reason: u32, _: *mut ()) {
            match call_reason {
                $crate::DLL_PROCESS_ATTACH => $init,
                _ => return,
            };
        }

        const MOD_INFO: $crate::ModInfo = $crate::ModInfo { title: $name, version: $version, author: $author };
    };
}