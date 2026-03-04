use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct PLA;

impl PLA {
    // PLA (Pull Accumulator)
    // Decrement stack pointer nya dengan sebanyak 1, lalu ambil data yang disimpan di stack,
    // dan masukkan ke register A
    // Ukuran opcode : 1 byte
    // Jumlah cycle : 4
    pub fn pull(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        cpu.sp = cpu.sp.wrapping_add(1);
        let data = bus.read(0x0100 + cpu.sp as u16);
        cpu.a = data;
        cpu.update_zero_and_negative_flags(cpu.a);
        4
    }
}
