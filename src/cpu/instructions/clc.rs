use crate::cpu::cpu::Cpu;

pub struct CLC;

impl CLC {
    // CLC (Clear Carry)
    // Set status carry di register status cpu menjadi 0
    // Ukuran opcode : 2 byte
    // Jumlah cycle : 2
    // Contoh kode assembly : CLC [18]
    // Artinya : Clear status carry di register status
    pub fn clear(cpu: &mut Cpu) -> u16 {
        cpu.status = cpu.status & 0b11111110;
        cpu.cycle += 2;
        2
    }
}
