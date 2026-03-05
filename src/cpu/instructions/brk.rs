use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct BRK;

impl BRK {
    // BRK (Break)
    // Memerintahkan CPU untuk berhenti sejenak, simpan pc dan status register ke stack
    // lalu lompat ke vector alamat tujuan dari $FFFE dan $FFFF, lalu mengisi PC dengan
    // alamat tersebut
    // Ukuran opcode : 2 (1 untuk BRK, 1 untuk padding kosong stlh BRK)
    // Jumlah cycle  : 7
    // Contoh opcode : BRK
    // Artinya : simpan pc dan status register ke stack, set  Flag B dan I sebelum memasukkan
    //           status register ke stack, lompat ke vector alamat tujuan dari $FFFE dan $FFFF, lalu isi
    //           PC dengan alamat itu
    pub fn brk(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        cpu.pc = cpu.pc.wrapping_add(2);
        let hi = cpu.pc >> 8;
        let lo = cpu.pc & 0xFF;
        bus.write(0x0100 + cpu.sp as u16, hi as u8);
        cpu.sp = cpu.sp.wrapping_sub(1);

        bus.write(0x0100 + cpu.sp as u16, lo as u8);
        cpu.sp = cpu.sp.wrapping_sub(1);

        let status_to_push = cpu.status | 0b00010000; // set bit ke 4 jadi 1, untuk
                                                      // menandai
                                                      // interrupt ini dipicu oleh brk,
                                                      // bukan hardware luar
        bus.write(0x0100 + cpu.sp as u16, status_to_push);
        cpu.sp = cpu.sp.wrapping_sub(1);

        cpu.status |= 0b00000100;

        let vector_lo = bus.read(0xFFFE) as u16;
        let vector_hi = bus.read(0xFFFF) as u16;

        let addr = (vector_hi << 8) | vector_lo;
        cpu.pc = addr;
        7
    }
}
