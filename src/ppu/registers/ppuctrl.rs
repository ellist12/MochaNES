use bitflags::bitflags;

bitflags! {
    pub struct PpuCtrl: u8 {
        const NAMETABLE1     = 0b0000_0001;
        const NAMETABLE2     = 0b0000_0010;
        const INCREMENT_MODE = 0b0000_0100;
        const SPRITE_TABLE   = 0b0000_1000;
        const BG_TABLE       = 0b0001_0000;
        const SPRITE_SIZE    = 0b0010_0000;
        const MASTER_SLAVE   = 0b0100_0000;
        const NMI_ENABLE     = 0b1000_0000;
    }
}
