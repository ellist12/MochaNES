use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct STX;

impl STX {
    // STX Absolute
    // Tulis nilai register X ke alamat memori yang di specify di 2 byte berikutnya
    // Ukuran opcode : 3 byte
    // Jumlah cycle  : 4
    // Contoh kode assembly : STX $3003
    // Artinya : tulis nilai dari register X ke address $3003
    pub fn absolute(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let lo = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let hi = bus.read(cpu.pc) as u16;
        cpu.pc = cpu.pc.wrapping_add(1);
        let addr = (hi << 8) | lo;
        bus.write(addr, cpu.x);
        4
    }
}
