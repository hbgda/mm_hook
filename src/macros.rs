#[macro_export]
macro_rules! message_box {
    ($title:expr, $content:expr, $style:expr) => {
        unsafe {
            MessageBoxA(HWND(0), s!($content), s!($title), MESSAGEBOX_STYLE($style))
        }
    };
}

#[macro_export]
macro_rules! get_key {
    ($key:expr) => {
        unsafe {
            GetAsyncKeyState($key) & 1 == 1
        }
    };
}

#[macro_export]
macro_rules! make_func {
    ($addr:expr, [$($params:ty),*]) => {
        make_func!($addr, [$($params),*], ())
    };
    ($addr:expr, [$($params:ty),*], $ret:ty) => {
        std::mem::transmute::<*const (), unsafe extern "system" fn($($params,)*) -> $ret>($addr as _)
    };
}

#[macro_export]
macro_rules! make_func_static {
    ($offset:literal, $name:ident ($($params:ty),*)) => {
        make_func_static!($offset, $name ($($params),*): ());
    };
    ($offset:literal, $name:ident ($($params:ty),*): $ret:ty) => {
        static $name: once_cell::sync::Lazy<unsafe extern "system" fn($($params,)*) -> $ret> = once_cell::sync::Lazy::new(|| unsafe { $crate::make_func!($crate::get_offset_ptr($offset), [$($params),*], $ret) });
    };
}

#[macro_export]
macro_rules! make_hook {
    ($id:ident, $ori:expr, ($($param:ident: $ty:ty),*) => $code:block) => {
        make_hook!($id, $ori, ($($param: $ty),*): () => $code);
    };
    ($id:ident, $ori:expr, ($($param:ident: $ty:ty),*): $ret:ty => $code:block) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            static $id: once_cell::sync::Lazy<retour::GenericDetour<unsafe extern "system" fn($($ty,)*) -> $ret>> = Lazy::new(|| {
                unsafe {
                    retour::GenericDetour::new($ori, [<$id _Fn>])
                        .expect(&format!("Failed to create hook: {}", stringify!($id)))
                }
            });
            #[allow(non_snake_case)]
            unsafe extern "system" fn [<$id _Fn>]($($param: $ty,)*) -> $ret {
                $code
            }
        }
    };
}

/// Old
#[macro_export]
macro_rules! make_hook_1 {
    ($id:ident, $ori:expr, $hook:ident: [$($params:ty),*]) => {
        static $id: once_cell::sync::Lazy<retour::GenericDetour<unsafe extern "system" fn($($params,)*)>> = Lazy::new(|| {
            unsafe { retour::GenericDetour::new($ori, $hook) }
                .expect(&format!("Failed to create hook: {}", stringify!($id)))
        });
    };
    ($id:ident, $ori:expr, $hook:ident: [$($params:ty),*] => $ret:ty) => {
        static $id: once_cell::sync::Lazy<retour::GenericDetour<unsafe extern "system" fn($($params,)*) -> $ret>> = Lazy::new(|| {
            unsafe { retour::GenericDetour::new($ori, $hook) }
                .expect(&format!("Failed to create hook: {}", stringify!($id)))
        });
    };
}

#[macro_export]
macro_rules! make_type {
    ($name:ident, [$($offset:literal => $field:ident: $ty:ty),*]) => {
        paste::paste! {
            pub struct $name(pub u64);
            impl $name {
                $(pub unsafe fn [<get_ $field>](&self) -> $ty { 
                    std::ptr::read((self.0 + $offset) as *const $ty) 
                })*

                $(pub unsafe fn [<set_ $field>](&self, val: $ty) {
                    std::ptr::write((self.0 + $offset) as *mut $ty, val)
                })*
            }
        }
    };
    ($name:ident, [$($offset:literal => $field:ident: $ty:ty),*], $($fn_offset:literal => $fn:ident ($($param:ident: $paramty:ty),*): $ret:ty),*) => {
        make_type!($name, [$($offset => $field: $ty)*]);
        impl $name {
            $(pub unsafe fn $fn(&self, $($param: $paramty)*) -> $ret {
                $crate::make_func!(self.0 + $fn_offset, [$($paramty)*], $ret)($($param)*)
            })*
        }
    };
    ($name:ident, $($fn_offset:literal => $fn:ident ($($param:ident: $paramty:ty),*): $ret:ty),*) => {
        make_type!($name, [], $($fn_offset => $fn($($param: $paramty)*): $ret)*);
    }
}