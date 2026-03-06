use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct EOR;

impl EOR {
    // EOR immediate
    // Lakukan operasi XOR (Exclusive OR) antara accumulator (A) dan sebuah nilai immediate.
    // Hasilnya simpan di register A
    // Ukuran opcode : 2 byte
    // Jumlah cycle  : 2
    pub fn immediate(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let param = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        println!("EOR #${:x}", param);
        let result = cpu.a ^ param;
        cpu.a = result;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }

    // EOR zeropage
    // Lakukan operasi XOR (Exclusive OR) antara accumulator (A) dan sebuah nilai di address zeropage ram.
    // Hasilnya simpan di register A
    // Ukuran opcode : 2 byte
    // Jumlah cycle  : 3
    pub fn zeropage(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let zeropage_addr = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let data = bus.read(zeropage_addr as u16);
        cpu.a = cpu.a ^ data;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 3;
        3
    }
}
