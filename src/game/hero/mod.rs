use crate::make_type;

pub struct HeroHealthRaw {
    // 0x0
    pub vftable: *const u64,
    // 0x8
    pub _buf: [u8; 0x71],
    // 0x80
    pub max_health: u64,

}

make_type!(
    HeroHealth,
    [
        0x80 => max_health: u64,
        0x88 => current_health: u64
    ]
);

make_type!(
    Test,
    [
        0x1 => var: u64
    ],
    0x0 => init(this: u64): ()
);

make_type!(
    TestFns,
    0x0 => init(this: u64): ()
);