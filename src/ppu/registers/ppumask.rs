use bitflags::bitflags;

bitflags! {
    pub struct PpuMask: u8 {
        const GRAYSCALE      = 0b0000_0001;
        const BG_LEFT        = 0b0000_0010;
        const SPRITE_LEFT    = 0b0000_0100;
        const BACKGROUND     = 0b0000_1000;
        const SPRITES        = 0b0001_0000;
        const RED            = 0b0010_0000;
        const GREEN          = 0b0100_0000;
        const BLUE           = 0b1000_0000;
    }
}
