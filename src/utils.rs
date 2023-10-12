use std::error::Error;

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
    let pattern = canny::pattern::Pattern::new(pattern_str)?;
    let info = canny::mem::windows::ProcessInfo::internal(s!("MilesMorales.exe"))?;
    let mut scanner = canny::mem::windows::ProcessScanner::scan(info, pattern);
    match scanner.next() {
        Some(addr) => Ok(addr as *const ()),
        None => Err(format!("Failed to find address for pattern: {pattern_str}").into())
    }
}

pub fn option_ptr<T>(ptr: *const T) -> Option<*const T> {
    if ptr.is_null() {
        return None
    }
    Some(ptr)
}