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
        std::mem::transmute::<*const (), unsafe extern "system" fn($($params,)*)>($addr as _)
    };
    ($addr:expr, [$($params:ty),*], $ret:ty) => {
        std::mem::transmute::<*const (), unsafe extern "system" fn($($params,)*) -> $ret>($addr as _)
    };
}

#[macro_export]
macro_rules! make_hook {
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
    ($name:ident, $($offset:literal: $field:ident: $ty:ty),*) => {
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
}