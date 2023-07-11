use super::entity::Entity;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug)]
pub struct Transform {
    _addr: u64,
    pub position: Vector3,
    pub scale: Vector3
}

impl Entity {
    pub unsafe fn transform(&self) -> Transform {
        let _addr = (self.0 as *const u64).read();
        
        let position = Vector3 {
            x: ((_addr + 0x30) as *const f32).read(),
            y: ((_addr + 0x34) as *const f32).read(),
            z: ((_addr + 0x38) as *const f32).read()
        };

        let scale = Vector3 {
            x: ((_addr + 0x70) as *const f32).read(),
            y: ((_addr + 0x74) as *const f32).read(),
            z: ((_addr + 0x78) as *const f32).read()
        };


        Transform { _addr, position, scale }
    }
}

impl Transform {
    pub unsafe fn set_position(&self, new_pos: Vector3) {
        std::ptr::write((self._addr + 0x30) as *mut f32, new_pos.x);
        std::ptr::write((self._addr + 0x34) as *mut f32, new_pos.y);
        std::ptr::write((self._addr + 0x38) as *mut f32, new_pos.z);
    }

    pub unsafe fn set_scale(&self, new_scale: Vector3) {
        std::ptr::write((self._addr + 0x70) as *mut f32, new_scale.x);
        std::ptr::write((self._addr + 0x74) as *mut f32, new_scale.y);
        std::ptr::write((self._addr + 0x78) as *mut f32, new_scale.z);
    }
}