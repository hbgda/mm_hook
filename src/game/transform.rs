use crate::make_func_static;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct Transform {
    _0x0: [u8; 0x2F],
    position: Vector3,
    _0x39: [u8; 0x36],
    scale: Vector3
}

make_func_static!(0x1E24190, SET_POSITION(*mut Transform, *const Vector3));

impl Transform {
    pub unsafe fn set_position(&mut self, new_pos: &Vector3) {
        SET_POSITION(self, new_pos);
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    pub fn set_scale(&mut self, new_scale: &Vector3) {
        // SET_SCALE(self, new_scale);
        todo!()
    }

    pub fn get_scale(&self) -> Vector3 {
        self.scale
    }
}