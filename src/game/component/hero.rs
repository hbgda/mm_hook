#[repr(C)]
pub struct HeroHealth { 
    _0x0: [u8; 0x80],       // 0x80
    pub max_health: f32,
    _0x84: u32,
    pub current_health: f32 // 0x88
}

impl super::Component for HeroHealth {
    const NAME: &'static str = "HeroHealth";
}