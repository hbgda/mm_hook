use std::error::Error;

use canny::mem::windows::{ProcessScanner, ProcessInfo};
use windows::{Win32::System::LibraryLoader::GetModuleHandleA, s};

pub unsafe fn get_module_base() -> isize {
    GetModuleHandleA(s!("MilesMorales.exe")).unwrap().0
}

pub unsafe fn get_offset(offset: isize) -> isize {
    get_module_base() + offset
}

pub unsafe fn get_offset_ptr<T>(offset: isize) -> *const T {
    (get_module_base() + offset) as *const T
}

pub unsafe fn get_offset_ptr_mut<T>(offset: isize) -> *mut T {
    (get_module_base() + offset) as *mut T
}

pub unsafe fn scan(pattern_str: &'static str) -> Result<*const (), Box<dyn Error>> {
    let mut scanner = create_scanner(pattern_str)?;
    match scanner.next() {
        Some(addr) => Ok(addr as *const ()),
        None => Err(format!("Failed to find address for pattern: {pattern_str}").into())
    }
}

pub unsafe fn create_scanner(pattern_str: &'static str) -> Result<ProcessScanner, Box<dyn Error>> {
    let pattern = canny::pattern::Pattern::new(pattern_str)?;
    let info = ProcessInfo::internal(s!("MilesMorales.exe"))?;
    Ok(ProcessScanner::scan(info, pattern))
}

pub unsafe fn scan_func_call(pattern: &'static str) -> Result<*const (), Box<dyn Error>> {
    let mut scanner = create_scanner(pattern).unwrap();
    let found = scanner.next().unwrap();
    let offset = i32::from_le_bytes(scanner.store[0..4].try_into().unwrap());
    Ok((found as isize + 5 + offset as isize) as *const ())
}

pub fn option_ptr<T>(ptr: *const T) -> Option<*const T> {
    if ptr.is_null() {
        return None
    }
    Some(ptr)
}