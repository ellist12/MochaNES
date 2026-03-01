use crate::cpu::cpu::Cpu;

pub struct TAX;

impl TAX {
    // TAX (Transfer Accumulator to X)
    // Ambil nilai di register A, lalu salin ke X
    // Ukuran opcode : 1 byte
    // Jumlah cycle : 2
    // Contoh assembly : TAX
    // Artinya : Salin value register A ke X
    pub fn transfer(cpu: &mut Cpu) -> u16 {
        cpu.x = cpu.a;
        cpu.update_zero_and_negative_flags(cpu.x);
        cpu.cycle += 2;
        2
    }
}
