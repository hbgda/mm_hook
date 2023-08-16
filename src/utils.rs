use canny::{pattern::Pattern, ScanPtr};
use windows::{Win32::System::{LibraryLoader::GetModuleHandleA, ProcessStatus::{MODULEINFO, GetModuleInformation}, Threading::GetCurrentProcess}, s};

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
