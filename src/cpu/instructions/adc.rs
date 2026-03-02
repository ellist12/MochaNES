use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct ADC;

impl ADC {
    pub fn add(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let addr = bus.read(cpu.pc);
        cpu.pc = cpu.pc.wrapping_add(1);
        let data = bus.read(addr as u16);
        let carry = cpu.status & 0b00000001;
        let new_data = cpu.a as u16 + data as u16 + carry as u16;
        // cek carry
        if new_data > 0xFF {
            // set carry flag ke 1
            cpu.status = cpu.status | 0b00000001;
        } else {
            cpu.status = cpu.status & 0b11111110;
        }

        // cek overflow
        if ((cpu.a & 0b10000000) == (data & 0b10000000)) && (cpu.a & 0b10000000) != (new_data as u8 & 0b10000000) {
            cpu.status = cpu.status | 0b01000000;
        } else {
            cpu.status = cpu.status & 0b10111111;
        }

        cpu.a = new_data as u8;

        cpu.update_zero_and_negative_flags(cpu.a);
        3
    }
}
