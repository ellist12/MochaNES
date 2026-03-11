use bitflags::bitflags;

bitflags! {
    pub struct PpuStatus: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT    = 0b0100_0000;
        const V_BLANK         = 0b1000_0000;
    }
}
