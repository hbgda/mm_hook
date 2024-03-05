// 0x2230f90
pub fn gui_hash(string: &str) -> u32 {
    let mut hash: u32 = 0x1505;
    for char in string.chars() {
        hash = hash * 0x21 + (char as u32);
    }
    hash
}