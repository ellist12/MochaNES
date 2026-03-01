use crate::{bus::Bus, cpu::cpu::Cpu};

pub struct LSR;

impl LSR {

    // LSR (Logical Shift Right)
    // Menggeser semua bit di accumulator ke kanan sebanyak 1 posisi,
    // saat digeser, bit ke 0 akan terlempar keluar dan masuk ke flag carry
    // jadi jika bit 0 = 1, setelah digeser, maka nilai flag carry = 1
    // Ukuran opcode : 1 byte
    // Jumlah cycle : 2 cycle

    pub fn shift(cpu: &mut Cpu, bus: &mut Bus) -> u16 {
        let carry_flag = (cpu.a & 0b00000001) != 0;
        if carry_flag {
            cpu.status = cpu.status | 0b00000001;
        } else {
            cpu.status = cpu.status & 0b11111110;
        }
        cpu.a = cpu.a >> 1;
        cpu.update_zero_and_negative_flags(cpu.a);
        cpu.cycle += 2;
        2
    }
}
