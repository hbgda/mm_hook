use crate::scan_func_static;

scan_func_static!(crate::patterns::SCALEFORMLOADER_OPENFILE, OPEN_FILE(*const (), *const u8) -> *const ());
pub unsafe fn load_file(path: &str) -> *const () {
    OPEN_FILE(std::ptr::null(), path.as_ptr())
}