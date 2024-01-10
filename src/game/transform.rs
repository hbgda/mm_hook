use crate::{scan_func_static, patterns};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpatialData {
    _0x0: [u8; 0x30],
    pos: Vector3
}

impl SpatialData {
    pub fn from_pos(pos: Vector3) -> SpatialData {
        SpatialData {
            _0x0: [0; 0x30],
            pos
        }
    }
}

#[repr(C)]
pub struct Transform {
    spatial_data: SpatialData
}

scan_func_static!(patterns::TRANSFORM_SETPOSITION, SET_POSITION(*mut Transform, *const Vector3));
scan_func_static!(patterns::TRANSFORM_SETSCALE, SET_SCALE(*mut Transform, *const Vector3));

impl Transform {
    pub fn spatial_data(&self) -> &SpatialData {
        &self.spatial_data
    }

    pub unsafe fn set_position(&mut self, new_pos: &Vector3) {
        SET_POSITION(self, new_pos);
    }

    pub fn position(&self) -> &Vector3 {
        &self.spatial_data.pos
    }

    pub unsafe fn set_scale(&mut self, new_scale: &Vector3) {
        SET_SCALE(self, new_scale);
    }

    // pub fn get_scale(&self) -> Vector3 {
    //     self.scale
    // }
}