use crate::cpu::cpu::Cpu;

pub struct CLC;

impl CLC {
    pub fn clear(cpu: &mut Cpu) -> u16 {
        cpu.status = cpu.status & 0b11111110;
        cpu.cycle += 2;
        2
    }
}
