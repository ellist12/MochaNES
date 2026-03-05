use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct RTS;

impl RTS {
    // RTS (Return from Subroutine)
    // Mengambil alamat yang tersimpan di stack oleh JSR, lalu mengatur program counter (PC) agar CPU
    // kembali ke instruksi yang tepat setelah pemanggilan fungsi tadi
    // Ukuran opcode : 1 byte
    // Jumlah cycle  : 6
    // Contoh kode assembly : RTS
    // Artinya : ambil low byte dan high byte dari instruksi yang disimpan oleh JSR dari stack, lalu
    //           gabungkan, dan set register PC ke address itu + 1
    pub fn ret(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        cpu.sp = cpu.sp.wrapping_add(1);
        let lo = bus.read(0x0100 + cpu.sp as u16) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);
        let hi = bus.read(0x0100 + cpu.sp as u16) as u16;
        let addr = (hi << 8) | lo;
        cpu.pc = addr.wrapping_add(1); // Ditambah satu, karena JSR tidak menyimpan address berikutnya,
                                       // melainkan byte terakhir dari parameter instruki JSR nya
        cpu.cycle += 6;
        6
    }

}
