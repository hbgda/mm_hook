use crate::make_type;

make_type!(
    HeroHealth,
    0x80: max_health: u64,
    0x88: current_health: u64
);