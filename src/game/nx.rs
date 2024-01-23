use crate::{declare_native_func, patterns, utils};

// const GET_NX_ACTIONS_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
//     let mut scanner = utils::create_scanner(patterns::NX_GETNXACTIONS).unwrap();
//     let found = scanner.next().unwrap();
//     let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
//     (found as isize + 5 + offset as isize) as usize
// });

declare_native_func!(
    utils::scan_func_call(patterns::NX_GETNXACTIONS).unwrap(),
    GET_NX_ACTIONS() -> *const ()
);

pub unsafe fn get_nx_actions() -> *const () {
    GET_NX_ACTIONS()
}