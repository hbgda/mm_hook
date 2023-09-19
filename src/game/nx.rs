use crate::make_func_static;

make_func_static!(0x21AB870, GET_NX_ACTIONS() -> *const ());
pub unsafe fn get_nx_actions() -> *const () {
    GET_NX_ACTIONS()
}

// static mut NX_INSTANCE: *const () = std::ptr::null();
// pub unsafe fn get_instance() -> Option<*const ()> {
//     if NX_INSTANCE.is_null() {
//         return None
//     }
//     Some(NX_INSTANCE)
// }

// Aint work idk
// make_hook!(
//     HOOK_Nx_Init,
//     crate::utils::scan(crate::patterns::NX_INIT).unwrap(),
//     (p1: *const ()) -> u32 {
//         let ret = HOOK_Nx_Init.call(p1);
//         asm!("
//         mov {}, rcx
//         ", out(reg) NX_INSTANCE);
//         ret
//     }
// );