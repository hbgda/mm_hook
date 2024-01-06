use once_cell::sync::Lazy;

use crate::{make_func_static, utils, make_func, patterns};

const GET_NX_ACTIONS_ADDR: Lazy<usize> = Lazy::new(|| unsafe {
    let mut scanner = utils::create_scanner(patterns::NX_GETNXACTIONS).unwrap();
    let found = scanner.next().unwrap();
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    (found as isize + 5 + offset as isize) as usize
});
pub unsafe fn get_nx_actions() -> *const () {
    make_func!(*GET_NX_ACTIONS_ADDR, () -> *const ())()
}