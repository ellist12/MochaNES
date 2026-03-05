use crate::cpu::cpu::Cpu;

pub struct TAY;

impl TAY {
    // TAY (Transfer A to Y)
    // mentransfer value dari register A (accumulator) ke register Y
    // Ukuran opcode : 1 byte
    // Jumlah cycle : 2
    pub fn transfer(cpu: &mut Cpu) -> u16 {
        cpu.y = cpu.a;
        cpu.update_zero_and_negative_flags(cpu.y);
        cpu.cycle += 2;
        2
    }
}
